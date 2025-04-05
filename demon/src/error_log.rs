use core::fmt::Display;
use num_enum::TryFromPrimitive;
use spin::Lazy;
use c_mine::pointer_from_hook;
use tag_structs::primitives::color::ColorRGB;
use crate::util::{CStrPtr, PointerProvider, StaticStringBytes, VariableProvider};

const MAX_LOG_LEN: usize = 1024;

pub const ERROR_WAS_SET: VariableProvider<u8> = variable! {
    name: "ERROR_WAS_SET",
    cache_address: 0x00B016C8,
    tag_address: 0x00BB8C80
};

#[derive(Copy, Clone, PartialEq, TryFromPrimitive)]
#[repr(i16)]
pub enum ErrorPriority {
    /// Closes the executable
    Death = 0,

    /// Logs to disk; sets failure flag to propagate errors (poor man's exception)
    Exception = 1,

    /// Prints to the console and logs to disk
    ///
    /// This will stop printing to the console if 10+ messages are sent in a short amount of time.
    Normal = 2,

    /// Log only; do not print to console
    FileOnly = 3,
}

const WRITE_TO_ERROR_FILE: PointerProvider<extern "C" fn(text: *const u8, some_bool: bool)> = pointer_from_hook!("write_to_error_file");

pub fn error_put_args(priority: ErrorPriority, fmt: core::fmt::Arguments) {
    let err = StaticStringBytes::<MAX_LOG_LEN>::from_fmt(fmt)
        .expect("failed to write error");

    // SAFETY: Shouldn't explode.
    unsafe { log_error_message(priority, err); }
}

static DEBUG_LOGGING: Lazy<bool> = Lazy::new(|| ini!("log", "debug_logging") == Some("true"));

unsafe extern "C" fn log_error_message(priority: ErrorPriority, message: impl Display) {
    let message = StaticStringBytes::<MAX_LOG_LEN>::from_display(message);

    if priority == ErrorPriority::Normal {
        console_color!(&const { ColorRGB::WHITE.as_colorargb() }, "{message}");
    }

    let message_to_log: StaticStringBytes<{ MAX_LOG_LEN + 32 }> = if priority == ErrorPriority::Death {
        StaticStringBytes::from_fmt(format_args!("(death) {message}\r\n"))
            .expect("failed to die; task failed successfully!")
    }
    else if priority == ErrorPriority::Exception {
        StaticStringBytes::from_fmt(format_args!("(exception) {message}\r\n"))
            .expect("failed to die; task failed successfully!")
    }
    else {
        StaticStringBytes::from_fmt(format_args!("{message}\r\n"))
            .expect("an error occurred while loading the previous error")
    };

    if *DEBUG_LOGGING {
        WRITE_TO_ERROR_FILE.get()(message_to_log.as_bytes().as_ptr(), true);
    }

    if priority == ErrorPriority::Death {
        panic!("Fatal error (ErrorPriority::Death): {message}")
    }

    if priority == ErrorPriority::Exception {
        panic!("Fatal error (ErrorPriority::Exception): {message}")
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn demon_error_catcher(priority: i16, message: CStrPtr) {
    let desired_priority = ErrorPriority::try_from(priority).expect("invalid priority!");
    log_error_message(desired_priority, message.display_lossy());
}
