pub mod c;

use core::fmt::Display;
use num_enum::TryFromPrimitive;
use c_mine::pointer_from_hook;
use tag_structs::primitives::color::{ColorARGB, ColorRGB};
use crate::id::ID;
use crate::memory::table::DataTable;
use crate::timing::{FixedTimer, TICK_RATE};
use crate::util::{CStrPtr, PointerProvider, StaticStringBytes, VariableProvider};

const MAX_LOG_LEN: usize = 1024;

pub static mut SHOW_DEBUG_MESSAGES: u8 = 1;

pub fn show_debug_messages() -> bool {
    // SAFETY: This is probably going to cause UB on systems that aren't x86 Windows, and thus it
    //         should be changed to an atomic when globals are reworked to use atomics.
    //
    //         In this case, the risk is acceptable in the interim.
    //
    unsafe { SHOW_DEBUG_MESSAGES != 0 }
}

pub const ERROR_WAS_SET: VariableProvider<u8> = variable! {
    name: "ERROR_WAS_SET",
    cache_address: 0x00B016C8,
    tag_address: 0x00BB8C80
};

/// Print an error to the console with the given formatting and log it.
#[allow(unused_macros)]
macro_rules! error {
    ($($args:tt)*) => {{
        crate::console::error_put_args(crate::console::ErrorPriority::Console, format_args!($($args)*));
    }};
}

/// Log a message without printing it to the console.
#[allow(unused_macros)]
macro_rules! log {
    ($($args:tt)*) => {{
        crate::console::error_put_args(crate::console::ErrorPriority::FileOnly, format_args!($($args)*));
    }};
}

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
    Console = 2,

    /// Log only; do not print to console
    FileOnly = 3,
}

const WRITE_TO_ERROR_FILE: PointerProvider<extern "C" fn(text: *const u8, some_bool: bool)> = pointer_from_hook!("write_to_error_file");

pub fn error_put_args(priority: ErrorPriority, fmt: core::fmt::Arguments) {
    let err = StaticStringBytes::<MAX_LOG_LEN>::from_fmt(fmt)
        .expect("failed to write error");

    // SAFETY: Hopefully safe???
    unsafe { log_error_message(priority, err); }
}

/// Print the formatted string to the in-game console.
#[allow(unused_macros)]
macro_rules! console {
    ($($args:tt)*) => {{
        crate::console::console_put_args(None, format_args!($($args)*));
    }};
}

/// Print the formatted string to the in-game console with a given color.
///
/// The first argument must be [`ColorARGB`] or [`&ColorARGB`].
#[allow(unused_macros)]
macro_rules! console_color {
    ($color:expr, $($args:tt)*) => {{
        let color: &tag_structs::primitives::color::ColorARGB = tag_structs::primitives::color::ColorARGB::as_ref(&$color);
        crate::console::console_put_args(Some(color), format_args!($($args)*));
    }};
}

pub fn console_put_args(color: Option<&ColorARGB>, fmt: core::fmt::Arguments) {
    let data = StaticStringBytes::<0xFE>::from_fmt(fmt)
        .expect("failed to write console message");
    console_put_message(color, data.as_bytes_with_null());
}

fn console_put_message(color: Option<&ColorARGB>, message_bytes: &[u8]) {
    const CONSOLE_PRINTF: PointerProvider<unsafe extern "C" fn(color: Option<&ColorARGB>, fmt: *const u8, arg: *const u8)> = pointer_from_hook!("console_printf");

    assert!(message_bytes.last() == Some(&0u8), "should be null-terminated");

    // SAFETY: PointerProvider is probably right.
    unsafe {
        CONSOLE_PRINTF.get()(color, b"%s\x00".as_ptr(), message_bytes.as_ptr());
    }
}

const TERMINAL_SALT: u16 = 0x6574;

#[repr(C)]
struct TerminalOutput {
    pub identifier: u16,
    pub unknown: u16,
    pub some_id: ID<TERMINAL_SALT>,
    pub unknown1: u32,
    pub unknown2: u8,
    pub text: [u8; 0xFF],
    pub unknown3: u32,
    pub color: ColorARGB,
    pub timer: u32
}

type TerminalOutputTable = DataTable<TerminalOutput, TERMINAL_SALT>;

const TERMINAL_INITIALIZED: VariableProvider<u8> = variable! {
    name: "TERMINAL_INITIALIZED",
    cache_address: 0x00C8AEE0,
    tag_address: 0x00D42490
};

const TERMINAL_OUTPUT_TABLE: VariableProvider<Option<&mut TerminalOutputTable>> = variable! {
    name: "TERMINAL_OUTPUT_TABLE",
    cache_address: 0x00C8AEE4,
    tag_address: 0x00D42494
};

const LIMIT_TICKS: u32 = 150;
const CONSOLE_FADE_FRAME_RATE: f64 = TICK_RATE;

/// Fades all terminal output
///
/// Only works once every 1/[`CONSOLE_FADE_FRAME_RATE`]th of a second. This is a temporary solution
/// until the console is replaced so at least the console is faded at the correct rate for now
/// instead of being unusable at high frame rates.
///
/// Unsafe because we cannot guarantee the table won't be concurrently written to at this moment...
unsafe fn fade_console_text(table: &'static mut TerminalOutputTable) {
    static RATE: FixedTimer = FixedTimer::new(
        1.0 / CONSOLE_FADE_FRAME_RATE,
        30
    );

    RATE.run(|| {
        for i in table.iter() {
            i.get_mut().timer = (i.get().timer + 1).min(LIMIT_TICKS);
        }
    });
}

const CONSOLE_IS_ACTIVE: VariableProvider<u8> = variable! {
    name: "CONSOLE_IS_ACTIVE",
    cache_address: 0x00C98AE0,
    tag_address: 0x00D500A0
};

const CONSOLE_COLOR: VariableProvider<ColorARGB> = variable! {
    name: "CONSOLE_COLOR",
    cache_address: 0x00C98B68,
    tag_address: 0x00D50128
};

const CONSOLE_PROMPT_TEXT: VariableProvider<[u8; 32]> = variable! {
    name: "CONSOLE_PROMPT_TEXT",
    cache_address: 0x00C98B78,
    tag_address: 0x00D50138
};

const CONSOLE_TEXT: VariableProvider<[u8; 256]> = variable! {
    name: "CONSOLE_TEXT",
    cache_address: 0x00C98B98,
    tag_address: 0x00D50158
};

const CONSOLE_HISTORY_LENGTH: VariableProvider<u16> = variable! {
    name: "CONSOLE_HISTORY_LENGTH",
    cache_address: 0x00C9949C,
    tag_address: 0x00D5015C
};

const CONSOLE_HISTORY_NEXT_INDEX: VariableProvider<u16> = variable! {
    name: "CONSOLE_HISTORY_NEXT_INDEX",
    cache_address: 0x00C9949E,
    tag_address: 0x00D5015E
};

const CONSOLE_HISTORY_SELECTED_INDEX: VariableProvider<u16> = variable! {
    name: "CONSOLE_HISTORY_SELECTED_INDEX",
    cache_address: 0x00C994A0,
    tag_address: 0x00D50160
};

const CONSOLE_ENABLED: VariableProvider<bool> = variable! {
    name: "CONSOLE_ENABLED",
    cache_address: 0x00C98AE1,
    tag_address: 0x00D500A1
};

const DEFAULT_CONSOLE_PROMPT_TEXT: &str = "halo( ";

pub const DEFAULT_CONSOLE_COLOR: ColorARGB = ColorARGB {
    a: 1.0,
    color: ColorRGB {
        r: 1.0,
        g: 0.3,
        b: 1.0
    }
};

/// Set the console prompt text.
///
/// # Panics
///
/// Panics if `string.len() >= 32`
pub fn set_console_prompt_text(string: &str) {
    // SAFETY: it's fairly safe because we know this text is here in the EXE
    // TODO: make console_prompt_text a String when all things that access it are replaced
    let prompt_text = unsafe { CONSOLE_PROMPT_TEXT.get_mut() };
    let Some(remaining_space) = (prompt_text.len() - 1).checked_sub(string.len()) else {
        panic!("console prompt text `{string}` is too long")
    };

    // also zeroes out the remainder of the buffer
    let mut copy_iterator = string
        .bytes()
        .chain(core::iter::repeat_n(0, remaining_space + 1));
    prompt_text.fill_with(|| copy_iterator.next().expect("should still be enough space"));
}

/// Set the console color
///
/// # Panics
///
/// Panics if `!color.is_valid()`
pub fn set_console_color_text(color: ColorARGB) {
    assert!(color.is_valid(), "invalid console color {color:?}");

    // SAFETY: should be present in the program
    *unsafe { CONSOLE_COLOR.get_mut() } = color;
}

unsafe extern "C" fn log_error_message(desired_priority: ErrorPriority, message: impl Display) {
    let actual_priority = if show_debug_messages() {
        desired_priority
    }
    else {
        match desired_priority {
            ErrorPriority::Console => ErrorPriority::FileOnly,
            ErrorPriority::Exception => ErrorPriority::Exception,
            ErrorPriority::Death => ErrorPriority::Death,
            ErrorPriority::FileOnly => ErrorPriority::FileOnly
        }
    };

    let message = StaticStringBytes::<MAX_LOG_LEN>::from_display(message);

    if actual_priority == ErrorPriority::Console {
        let color = &ColorARGB { a: 1.0, color: ColorRGB::WHITE };
        console_put_args(Some(color), format_args!("{message}"));
    }

    let message_to_log = if actual_priority == ErrorPriority::Death {
        StaticStringBytes::<{ MAX_LOG_LEN + 32 }>::from_fmt(format_args!("(death) {message}\r\n"))
            .expect("failed to die; task failed successfully!")
    }
    else if actual_priority == ErrorPriority::Exception {
        StaticStringBytes::<{ MAX_LOG_LEN + 32 }>::from_fmt(format_args!("(exception) {message}\r\n"))
            .expect("failed to die; task failed successfully!")
    }
    else {
        StaticStringBytes::<{ MAX_LOG_LEN + 32 }>::from_fmt(format_args!("{message}\r\n"))
            .expect("an error occurred while loading the previous error")
    };

    WRITE_TO_ERROR_FILE.get()(message_to_log.as_bytes().as_ptr(), true);

    if actual_priority == ErrorPriority::Death {
        panic!("Fatal error (ErrorPriority::Death): {message}")
    }

    if actual_priority == ErrorPriority::Exception {
        panic!("Fatal error (ErrorPriority::Exception): {message}")
    }
}

#[no_mangle]
unsafe extern "C" fn demon_error_catcher(priority: i16, message: CStrPtr) {
    let desired_priority = ErrorPriority::try_from(priority).expect("invalid priority!");
    log_error_message(desired_priority, message.display_lossy());
}

