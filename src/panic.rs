use alloc::string::String;
use alloc::vec::Vec;
use core::mem::zeroed;
use core::panic::PanicInfo;
use core::ptr::{null, null_mut};
use windows_sys::s;
use windows_sys::Win32::Foundation::{GetLastError, TRUE};
use windows_sys::Win32::System::Diagnostics::Debug::{RtlCaptureStackBackTrace, SymFromAddr, SymGetLineFromAddr64, SymInitialize, SymSetOptions, IMAGEHLP_LINE64, SYMBOL_INFO, SYMOPT_ALLOW_ABSOLUTE_SYMBOLS, SYMOPT_LOAD_ANYTHING, SYMOPT_LOAD_LINES};
use windows_sys::Win32::System::Threading::{ExitProcess, GetCurrentProcess};
use windows_sys::Win32::UI::WindowsAndMessaging::MESSAGEBOX_STYLE;
use crate::util::{get_exe_dir, write_to_file};

#[panic_handler]
unsafe fn on_panic(panic_info: &PanicInfo) -> ! {
    let displayed_output = generate_panic_message(panic_info);
    windows_sys::Win32::UI::WindowsAndMessaging::MessageBoxA(
        null_mut(),
        displayed_output.as_ref().map(|o| o.as_ptr()).unwrap_or(s!("(could not generate a panic message)")),
        s!("Panic!"),
        MESSAGEBOX_STYLE::default()
    );
    ExitProcess(197);
}

pub unsafe fn generate_panic_message(panic_info: &PanicInfo) -> Option<Vec<u8>> {
    let (mut output_full, mut output_brief) = {
        let mut brief = String::with_capacity(2048);
        alloc::fmt::write(&mut brief, format_args!("A fatal error occurred!\n\nMessage: {}", panic_info.message())).ok()?;

        if let Some(location) = panic_info.location() {
            alloc::fmt::write(&mut brief, format_args!("\n\nLocation: {location}")).ok()?;
        }

        let mut full = String::with_capacity(4096);
        full += &brief;

        (full, brief.into_bytes())
    };

    let mut bt_printed = false;

    let mut pointers = [null_mut(); u16::MAX as usize];
    let captured = RtlCaptureStackBackTrace(0, pointers.len() as u32, pointers.as_mut_ptr(), null_mut()) as usize;
    let process = GetCurrentProcess();

    SymSetOptions(SYMOPT_ALLOW_ABSOLUTE_SYMBOLS | SYMOPT_LOAD_LINES | SYMOPT_LOAD_ANYTHING);
    if SymInitialize(process, null(), TRUE) == TRUE {
        let pointers = &pointers[..captured];
        for i in pointers.iter().copied() {
            const SYMBOL_LEN: usize = 2048;
            const NAME_LEN: usize = 512;
            let mut symbol_info: [u8; SYMBOL_LEN] = [0u8; SYMBOL_LEN];
            let symbol_info_ref = &mut symbol_info as *mut _ as *mut SYMBOL_INFO;
            const _: () = assert!(size_of::<SYMBOL_INFO>() + NAME_LEN < SYMBOL_LEN);

            let symbol_info_ref = &mut *symbol_info_ref;
            symbol_info_ref.MaxNameLen = NAME_LEN as u32;
            symbol_info_ref.SizeOfStruct = size_of_val(symbol_info_ref) as u32;

            if !bt_printed {
                bt_printed = true;
                let _ = alloc::fmt::write(&mut output_full, format_args!("\n\nBacktrace:"));
            }

            let mut displacement = 0;
            let symbol = SymFromAddr(process, i as u64, &mut displacement, symbol_info_ref);
            let no_symbol_reason = GetLastError();

            let _ = alloc::fmt::write(&mut output_full, format_args!("\n0x{:08X}", i as usize));
            if symbol == TRUE {
                let name = core::slice::from_raw_parts(symbol_info_ref.Name.as_ptr() as *const u8, NAME_LEN);
                let Ok(name_cstr) = core::ffi::CStr::from_bytes_until_nul(name) else {
                    continue
                };

                let _ = alloc::fmt::write(&mut output_full, format_args!(" {}", name_cstr.to_string_lossy()));

                let mut img: IMAGEHLP_LINE64 = zeroed();
                img.SizeOfStruct = size_of_val(&img) as u32;

                let mut asdf = [0u8; 1024];
                img.FileName = asdf.as_mut_ptr();

                let mut displacement = 0;
                if SymGetLineFromAddr64(process, i as u64, &mut displacement, &mut img) == TRUE {
                    let _ = alloc::fmt::write(&mut output_full, format_args!("\n           ...in {}:{}", core::ffi::CStr::from_ptr(img.FileName as *const _).to_string_lossy(), img.LineNumber));
                }
            }
            else if no_symbol_reason == 487 {
                let _ = alloc::fmt::write(&mut output_full, format_args!(" <no symbol>"));
            }
            else {
                let _ = alloc::fmt::write(&mut output_full, format_args!(" <no symbol> ({no_symbol_reason})"));
            }
        }
    }
    else {
        output_brief.extend_from_slice(b"\n\n(unable to load symbols)");
    }

    output_full.push('\n');

    let error_path = get_exe_dir() + "/demon-panic.txt";

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

#[no_mangle]
fn rust_eh_personality() {}