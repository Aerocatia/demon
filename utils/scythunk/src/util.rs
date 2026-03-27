use core::ptr::null_mut;

use windows_sys::Win32::{System::Threading::{ExitProcess, GetCurrentProcess, TerminateProcess}, UI::WindowsAndMessaging::MessageBoxW};

pub fn messagebox(
    header: &str,
    content: &str
) {
    let header_utf16 = encode_utf16_with_nul(header);
    let content_utf16 = encode_utf16_with_nul(content);

    unsafe {
        MessageBoxW(null_mut(), content_utf16.as_ptr(), header_utf16.as_ptr(), 0);
    }
}

pub fn encode_utf16_with_nul(what: &str) -> alloc::vec::Vec<u16> {
    let mut q = alloc::vec::Vec::with_capacity(what.len() * 2 + 2);
    for i in what.encode_utf16() {
        q.push(i);
    }
    q.push(0);
    q
}

pub fn terminate() -> ! {
    unsafe {
        TerminateProcess(GetCurrentProcess(), 111);
        ExitProcess(111)
    }
}
