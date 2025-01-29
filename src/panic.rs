use crate::init::{get_exe_type_if_available, ExeType};
use crate::util::{get_exe_dir, write_to_file};
use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use c_mine::c_mine;
use core::ffi::CStr;
use core::fmt;
use core::mem::zeroed;
use core::panic::PanicInfo;
use core::ptr::{null, null_mut};
use core::ops::Range;
use core::sync::atomic::{AtomicBool, Ordering};
use windows_sys::s;
use windows_sys::Win32::Foundation;
use windows_sys::Win32::Foundation::{GetLastError, HANDLE, HMODULE, TRUE};
use windows_sys::Win32::System::Diagnostics::Debug::{RtlCaptureStackBackTrace, SymFromAddr, SymGetLineFromAddr64, SymInitialize, SymSetOptions, EXCEPTION_POINTERS, IMAGEHLP_LINE64, SYMBOL_INFO, SYMOPT_ALLOW_ABSOLUTE_SYMBOLS, SYMOPT_LOAD_ANYTHING, SYMOPT_LOAD_LINES};
use windows_sys::Win32::System::ProcessStatus::{EnumProcessModules, GetModuleBaseNameA, GetModuleInformation, MODULEINFO};
use windows_sys::Win32::System::Threading::{ExitProcess, GetCurrentProcess, TerminateProcess};
use windows_sys::Win32::UI::WindowsAndMessaging::{MB_ICONERROR, MB_OK};

#[cfg(not(test))]
#[panic_handler]
unsafe fn on_panic(panic_info: &PanicInfo) -> ! {
    let displayed_output = generate_panic_message(panic_info);
    let msg = displayed_output
        .as_ref()
        .map(|o| o.as_ptr())
        .unwrap_or(s!("(could not generate a panic message)"));
    windows_sys::Win32::UI::WindowsAndMessaging::MessageBoxA(
        null_mut(),
        msg,
        s!("Panic!"),
        MB_ICONERROR | MB_OK
    );
    crash_process();
}

pub unsafe fn generate_panic_message(panic_info: &PanicInfo) -> Option<Vec<u8>> {
    let (mut output_full, mut output_brief) = {
        let mut brief = String::with_capacity(2048);
        fmt::write(&mut brief, format_args!("A fatal error occurred!\n\nMessage: {}", panic_info.message())).ok()?;

        if let Some(location) = panic_info.location() {
            fmt::write(&mut brief, format_args!("\n\nPanic location:\n- {location}")).ok()?;
        }

        let exe_type = match get_exe_type_if_available() {
            Some(ExeType::Tag) => "tag",
            Some(ExeType::Cache) => "cache",
            None => "unknown (not fully loaded)"
        };

        fmt::write(&mut brief, format_args!("\n\nEXE type:\n- {exe_type}")).ok()?;

        let mut full = String::with_capacity(4096);
        full += &brief;

        (full, brief.into_bytes())
    };

    let process = GetCurrentProcess();
    let enumerated_modules = enumerate_modules(process);

    let mut pointers = [null_mut(); u16::MAX as usize];
    let captured = RtlCaptureStackBackTrace(0, pointers.len() as u32, pointers.as_mut_ptr(), null_mut()) as usize;

    fmt::write(&mut output_full, format_args!("\n\nBacktrace:")).ok()?;
    if initialize_sym_data(process) {
        let pointers = &pointers[..captured];
        for i in pointers.iter().copied() {
            resolve_address_symbol_data(&enumerated_modules, i as usize, &mut output_full, process);
        }
    }
    else {
        fmt::write(&mut output_full, format_args!("\n<no backtrace data due to an error>\n")).ok()?;
    }

    fmt::write(&mut output_full, format_args!("\n\nLoaded modules:\n")).ok()?;
    if let Some(m) = enumerated_modules.as_ref() {
        for module in m {
            let module_name = module.name();
            if let Some(r) = &module.range {
                fmt::write(&mut output_full, format_args!("- 0x{:08X}...0x{:08X}: {module_name}\n", r.start, r.end)).ok()?;
            }
            else {
                fmt::write(&mut output_full, format_args!("- 0x{:08X} [GetModuleInformation() failed]: {module_name}\n", module.module as usize)).ok()?;
            }
        }
    }
    else {
        fmt::write(&mut output_full, format_args!("...no module data\n")).ok();
    }

    let error_path = get_exe_dir() + "\\demon-panic.txt";

    match write_to_file(&error_path, output_full.as_bytes()) {
        Ok(_) => {
            output_brief.extend_from_slice(b"\n\nAn error report is output at:\n");
            output_brief.extend_from_slice(error_path.as_bytes());
        }
        Err(e) => {
            output_brief.extend_from_slice(b"\n\nCould not write the error report:\n");
            output_brief.extend_from_slice(e.as_bytes());
        }
    }

    output_brief.push(0);
    Some(output_brief)
}

struct EnumeratedModule {
    name: [u8; 256],
    module: HMODULE,
    range: Option<Range<usize>>
}
impl EnumeratedModule {
    fn name(&self) -> Cow<str> {
        CStr::from_bytes_until_nul(&self.name)
            .expect("must be null terminated")
            .to_string_lossy()
    }
}

unsafe fn enumerate_modules(process: HANDLE) -> Option<Vec<EnumeratedModule>> {
    let mut modules: [HMODULE; 8192] = [null_mut(); 8192];
    let mut actual_size = 0;
    if EnumProcessModules(process, modules.as_mut_ptr(), size_of_val(&modules) as u32, &mut actual_size) != TRUE {
        return None
    }

    let modules = &mut modules[..actual_size as usize / size_of::<HMODULE>()];
    modules.sort();

    let mut enumerated = Vec::with_capacity(modules.len());

    for module in modules {
        let mut module_name_bytes = [0u8; 256];
        let module_name_length = GetModuleBaseNameA(
            process,
            *module,
            module_name_bytes.as_mut_ptr(),
            module_name_bytes.len() as u32 - 1
        );
        module_name_bytes[module_name_length as usize] = 0;

        let mut module_info: MODULEINFO = zeroed();
        let range = if GetModuleInformation(process, *module, &mut module_info, size_of_val(&module_info) as u32) == TRUE {
            let start = module_info.lpBaseOfDll as usize;
            let size = module_info.SizeOfImage as usize;
            Some(start..start + size)
        }
        else {
            None
        };

        let module_to_add = EnumeratedModule {
            name: module_name_bytes,
            module: *module,
            range
        };

        enumerated.push(module_to_add);
    }

    Some(enumerated)
}

unsafe fn initialize_sym_data(process: HANDLE) -> bool {
    static SYM_DATA_LOADED: AtomicBool = AtomicBool::new(false);
    static SYM_DATA_RESULT: AtomicBool = AtomicBool::new(false);

    if !SYM_DATA_LOADED.swap(true, Ordering::Relaxed) {
        SymSetOptions(SYMOPT_ALLOW_ABSOLUTE_SYMBOLS | SYMOPT_LOAD_LINES | SYMOPT_LOAD_ANYTHING);
        SYM_DATA_RESULT.store(SymInitialize(process, null(), TRUE) == TRUE, Ordering::Relaxed);
    }
    SYM_DATA_RESULT.load(Ordering::Relaxed)
}

unsafe fn resolve_address_symbol_data(enumerated_modules: &Option<Vec<EnumeratedModule>>, address: usize, output_full: &mut String, process: HANDLE) {
    const SYMBOL_LEN: usize = 2048;
    const NAME_LEN: usize = 512;
    let mut symbol_info: [u8; SYMBOL_LEN] = [0u8; SYMBOL_LEN];
    let symbol_info_ref = &mut symbol_info as *mut _ as *mut SYMBOL_INFO;
    const _: () = assert!(size_of::<SYMBOL_INFO>() + NAME_LEN < SYMBOL_LEN);

    let symbol_info_ref = &mut *symbol_info_ref;
    symbol_info_ref.MaxNameLen = NAME_LEN as u32;
    symbol_info_ref.SizeOfStruct = size_of_val(symbol_info_ref) as u32;

    let mut displacement = 0;
    let symbol = SymFromAddr(process, address as u64, &mut displacement, symbol_info_ref);
    let no_symbol_reason = GetLastError();

    let _ = fmt::write(output_full, format_args!("\n- 0x{:08X}", address));
    if symbol == TRUE {
        let name = core::slice::from_raw_parts(symbol_info_ref.Name.as_ptr() as *const u8, NAME_LEN);
        let Ok(name_cstr) = CStr::from_bytes_until_nul(name) else {
            return
        };

        let mut module_name_bytes = [0u8; 256];
        let module_name_length = GetModuleBaseNameA(
            process,
            symbol_info_ref.ModBase as HMODULE,
            module_name_bytes.as_mut_ptr(),
            module_name_bytes.len() as u32 - 1
        );
        module_name_bytes[module_name_length as usize] = 0;

        let module_name = if module_name_length == 0 {
            Cow::Borrowed("???")
        }
        else {
            CStr::from_bytes_until_nul(&module_name_bytes).expect("should have zero").to_string_lossy()
        };

        let _ = fmt::write(output_full, format_args!(" {} ({module_name})", name_cstr.to_string_lossy()));

        let mut img: IMAGEHLP_LINE64 = zeroed();
        img.SizeOfStruct = size_of_val(&img) as u32;

        let mut asdf = [0u8; 1024];
        img.FileName = asdf.as_mut_ptr();

        let mut displacement = 0;
        if SymGetLineFromAddr64(process, address as u64, &mut displacement, &mut img) == TRUE {
            let _ = fmt::write(output_full, format_args!("\n             ...in {}:{}", CStr::from_ptr(img.FileName as *const _).to_string_lossy(), img.LineNumber));
        }
    }
    else if no_symbol_reason == 487 {
        let _ = fmt::write(output_full, format_args!(" <no symbol>"));
    }
    else {
        let _ = fmt::write(output_full, format_args!(" <no symbol due to error {no_symbol_reason}>"));
    }

    if symbol != TRUE {
        if let Some(f) = enumerated_modules.as_ref().and_then(|m| m.iter().find(|m| m.range.clone().is_some_and(|r| r.contains(&address)))) {
            let _ = fmt::write(output_full, format_args!(" ({})", f.name()));
        }
        else {
            let _ = fmt::write(output_full, format_args!(" (can't get module)"));
        }
    }
}

/// Closes the process quickly with error code 197.
pub fn crash_process() -> ! {
    unsafe {
        TerminateProcess(GetCurrentProcess(), 197);

        // in case TerminateProcess fails for some reason
        ExitProcess(197);
    }
}

#[c_mine]
pub unsafe extern "C" fn gathering_exception_data(pointers: &EXCEPTION_POINTERS) -> ! {
    static PANICKED: AtomicBool = AtomicBool::new(false);

    if PANICKED.swap(true, Ordering::Relaxed) {
        // in case our panic somehow triggered another segfault
        crash_process();
    }

    let exception_record = &*pointers.ExceptionRecord;
    let context = &*pointers.ContextRecord;
    let code = exception_record.ExceptionCode;

    let error_kind = match code {
        Foundation::EXCEPTION_ACCESS_VIOLATION => " (segfault; param1=access, param2=address)",
        Foundation::EXCEPTION_BREAKPOINT => " (breakpoint; likely a failed assertion)",
        _ => "",
    };

    let address = exception_record.ExceptionAddress as usize;
    let flags = exception_record.ExceptionFlags;

    let mut params = String::with_capacity(1024);
    for i in &exception_record.ExceptionInformation[..(exception_record.NumberParameters as usize).min(exception_record.ExceptionInformation.len())] {
        if params.is_empty() {
            let _ = fmt::write(&mut params, format_args!("Exception params:\n"));
        }
        let _ = fmt::write(&mut params, format_args!("- 0x{i:08X}\n"));
    }
    if !params.is_empty() {
        let _ = fmt::write(&mut params, format_args!("\n"));
    }

    let mut address_symbol_info = String::with_capacity(1024);

    let process = GetCurrentProcess();
    if initialize_sym_data(process) {
        resolve_address_symbol_data(&enumerate_modules(process), address, &mut address_symbol_info, process);
    }
    else {
        let _ = fmt::write(&mut address_symbol_info, format_args!("\n0x{address:08X} (can't get any more info)"));
    }

    let mut register_dump = String::with_capacity(4096);
    let _ = fmt::write(&mut register_dump, format_args!("- EAX: 0x{:08X}\n", context.Eax));
    let _ = fmt::write(&mut register_dump, format_args!("- EBX: 0x{:08X}\n", context.Ebx));
    let _ = fmt::write(&mut register_dump, format_args!("- ECX: 0x{:08X}\n", context.Ecx));
    let _ = fmt::write(&mut register_dump, format_args!("- EDX: 0x{:08X}\n", context.Edx));
    let _ = fmt::write(&mut register_dump, format_args!("- EDI: 0x{:08X}\n", context.Edi));
    let _ = fmt::write(&mut register_dump, format_args!("- EBP: 0x{:08X}\n", context.Ebp));
    let _ = fmt::write(&mut register_dump, format_args!("- ESP: 0x{:08X}\n", context.Esp));
    let _ = fmt::write(&mut register_dump, format_args!("- EFlags: 0x{:08X}", context.EFlags));

    panic!("Exception!\n\nException code:\n- 0x{code:08X}{error_kind}\n\nException flags:\n- {flags}\n\n{params}Exception address:{address_symbol_info}\n\nException register dump:\n{register_dump}");
}

#[cfg(not(test))]
#[no_mangle]
fn rust_eh_personality() {}
