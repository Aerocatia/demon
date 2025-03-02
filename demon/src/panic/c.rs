use alloc::string::String;
use core::fmt;
use core::sync::atomic::{AtomicBool, Ordering};
use windows_sys::Win32::Foundation;
use windows_sys::Win32::System::Diagnostics::Debug::EXCEPTION_POINTERS;
use windows_sys::Win32::System::Threading::GetCurrentProcess;
use c_mine::c_mine;
use crate::panic::{crash_process, enumerate_modules, initialize_sym_data, resolve_address_symbol_data};
use crate::util::{CStrPtr, StaticStringBytes};

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

#[c_mine]
pub unsafe extern "C" fn display_assert(assertion: CStrPtr, file: CStrPtr, line: i32, is_fatal_error: bool) {
    // Horrible hack for getting the caller
    let return_address = *((&assertion as *const _) as *const usize).sub(1);

    let reason = assertion.display_lossy();
    let file = file.display_lossy();

    let error_type = if is_fatal_error { "ERROR" } else { "WARN" };

    let err = StaticStringBytes::<1024>::from_fmt(
        format_args!("Halo assertion failed ({error_type}): {file}:{line} @ 0x{return_address:08X}: {reason}")
    ).unwrap();

    error!("Halo assertion failed ({error_type}): {file}:{line} @ 0x{return_address:08X}: {reason}");

    if is_fatal_error {
        panic!("Halo assertion failed ({error_type})\n\nLocation:\n- {file}:{line}\n\nAddress:\n- 0x{return_address:08X}\n\nAssertion:\n- {reason}");
    }
}
