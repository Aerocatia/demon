pub mod c;
use crate::init::{get_exe_type_if_available, ExeType};
use crate::util::{get_exe_dir, write_to_file};
use alloc::borrow::Cow;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::ffi::CStr;
use core::fmt;
use core::mem::zeroed;
use core::ops::Range;
use core::panic::PanicInfo;
use core::ptr::{null, null_mut};
use core::sync::atomic::{AtomicBool, Ordering};
use windows_sys::s;
use windows_sys::Win32::Foundation::{GetLastError, HANDLE, HMODULE, TRUE};
use windows_sys::Win32::System::Diagnostics::Debug::{RtlCaptureStackBackTrace, SymFromAddr, SymGetLineFromAddr64, SymInitialize, SymSetOptions, IMAGEHLP_LINE64, SYMBOL_INFO, SYMOPT_ALLOW_ABSOLUTE_SYMBOLS, SYMOPT_LOAD_ANYTHING, SYMOPT_LOAD_LINES};
use windows_sys::Win32::System::ProcessStatus::{EnumProcessModules, GetModuleBaseNameA, GetModuleInformation, MODULEINFO};
use windows_sys::Win32::System::SystemInformation::GetSystemTime;
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
    let mut system_time = zeroed();
    GetSystemTime(&mut system_time);

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

        let mut full = String::with_capacity(32768);
        full += &brief;

        fmt::write(&mut full, format_args!(
            "\n\nTime:\n- {year:04}-{month:02}-{day:02} {hour:02}:{minute:02}:{second:02}",
            year = system_time.wYear,
            month = system_time.wMonth,
            day = system_time.wDay,
            hour = system_time.wHour,
            minute = system_time.wMinute % 60,
            second = system_time.wSecond
        )).ok()?;

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

    let error_path = format!(
        "{exe_dir}\\demon-panic-{year:04}-{month:02}-{day:02}T{hour:02}-{minute:02}-{second:02}.txt",
        exe_dir = get_exe_dir(),
        year = system_time.wYear,
        month = system_time.wMonth,
        day = system_time.wDay,
        hour = system_time.wHour,
        minute = system_time.wMinute % 60,
        second = system_time.wSecond
    );

    match write_to_file(&error_path, output_full.as_bytes()) {
        Ok(_) => {
            output_brief.extend_from_slice(b"\n\nAn error report was saved to:\n");
            output_brief.extend_from_slice(error_path.as_bytes());
        }
        Err(e) => {
            output_brief.extend_from_slice(format!("\n\nCould not write an error report to {error_path}:\n").as_bytes());
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

#[cfg(not(test))]
#[no_mangle]
fn rust_eh_personality() {}
