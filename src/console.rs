use alloc::vec::Vec;
use core::mem::transmute;
use crate::util::VariableProvider;

pub const ERROR: VariableProvider<[u8; 0]> = VariableProvider {
    name: "ERROR",
    cache_address: 0x00408607 as *mut _,
    tags_address: 0x0040785B as *mut _
};

pub const ERROR_WAS_SET: VariableProvider<u8> = VariableProvider {
    name: "ERROR_WAS_SET",
    cache_address: 0x00B016C8 as *mut _,
    tags_address: 0x00BB8C80 as *mut _
};

#[repr(i16)]
pub enum ErrorPriority {
    /// Closes the executable
    Death = 0,

    /// Logs to disk; sets failure flag to propagate errors (poor man's exception)
    Error = 1,

    /// Prints to the console and logs to disk
    ///
    /// This will stop printing to the console if 10+ messages are sent in a short amount of time.
    Console = 2,

    /// Log only; do not print to console
    FileOnly = 3,
}

pub fn error(priority: ErrorPriority, message: &str) {
    let mut message_bytes = Vec::with_capacity(message.len() + 1);
    message_bytes.extend_from_slice(message.as_bytes());
    message_bytes.push(0);

    // SAFETY: VariableProvider is probably right.
    unsafe {
        let what = ERROR.get() as *const _;
        let what: unsafe extern "C" fn(priority: i16, fmt: *const u8, arg: *const u8) = transmute(what);
        what(priority as i16, b"%s\x00".as_ptr(), message_bytes.as_ptr());

        *ERROR_WAS_SET.get_mut() = 0;
    }
}
