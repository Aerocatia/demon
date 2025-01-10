use core::mem::transmute;
use c_mine::c_mine;
use crate::id::ID;
use crate::table::DataTable;
use crate::timing::FixedTimer;
use crate::util::VariableProvider;

pub const ERROR_WAS_SET: VariableProvider<u8> = VariableProvider {
    name: "ERROR_WAS_SET",
    cache_address: 0x00B016C8 as *mut _,
    tags_address: 0x00BB8C80 as *mut _
};

/// Print an error to the console with the given formatting and log it.
#[allow(unused_macros)]
macro_rules! error {
    ($($args:tt)*) => {{
        crate::console::error_put_args(crate::console::ErrorPriority::Console, format_args!($($args), *));
    }};
}

/// Log a message without printing it to the console.
#[allow(unused_macros)]
macro_rules! log {
    ($($args:tt)*) => {{
        crate::console::error_put_args(crate::console::ErrorPriority::FileOnly, format_args!($($args), *));
    }};
}

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

pub fn error_put_args(priority: ErrorPriority, fmt: core::fmt::Arguments) {
    struct ErrorBuffer {
        offset: usize,
        data: [u8; 0xFFE]
    }
    impl core::fmt::Write for ErrorBuffer {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            let max_len = self.data.len();
            let remainder = &mut self.data[self.offset..max_len-1];
            let bytes_to_add = s.as_bytes();
            let bytes = &bytes_to_add[..remainder.len().min(bytes_to_add.len())];
            if !bytes.is_empty() {
                remainder[..bytes.len()].copy_from_slice(bytes);
                self.offset += bytes.len();
            }
            Ok(())
        }
    }

    let mut message_bytes = ErrorBuffer {
        offset: 0,
        data: [0u8; 0xFFE]
    };

    // never fails!
    let _ = core::fmt::write(&mut message_bytes, fmt);

    error_put_message(priority, &message_bytes.data);
}

pub fn error_put_message(priority: ErrorPriority, error_bytes: &[u8]) {
    const ERROR: VariableProvider<[u8; 0]> = VariableProvider {
        name: "ERROR",
        cache_address: 0x00408607 as *mut _,
        tags_address: 0x0040785B as *mut _
    };

    assert!(error_bytes.last() == Some(&0u8), "should be null-terminated");

    // SAFETY: VariableProvider is probably right.
    unsafe {
        let what = ERROR.get() as *const _;
        let what: unsafe extern "C" fn(priority: i16, fmt: *const u8, arg: *const u8) = transmute(what);
        what(priority as i16, b"%s\x00".as_ptr(), error_bytes.as_ptr());
    }
}

#[repr(C)]
pub struct ConsoleColor {
    pub alpha: f32,
    pub red: f32,
    pub green: f32,
    pub blue: f32
}

/// Print the formatted string to the in-game console.
#[allow(unused_macros)]
macro_rules! console {
    ($($args:tt)*) => {{
        crate::console::console_put_args(None, format_args!($($args), *));
    }};
}

/// Print the formatted string to the in-game console with a given color.
///
/// The first argument must be a &ConsoleColor reference.
#[allow(unused_macros)]
macro_rules! console_color {
    ($color:expr, $($args:tt)*) => {{
        let color: &crate::console::ConsoleColor = $color;
        crate::console::console_put_args(Some(color), format_args!($($args), *));
    }};
}

pub fn console_put_args(color: Option<&ConsoleColor>, fmt: core::fmt::Arguments) {
    struct ConsoleBuffer {
        offset: usize,
        data: [u8; 0xFE]
    }
    impl core::fmt::Write for ConsoleBuffer {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            let max_len = self.data.len();
            let remainder = &mut self.data[self.offset..max_len-1];
            let bytes_to_add = s.as_bytes();
            let bytes = &bytes_to_add[..remainder.len().min(bytes_to_add.len())];
            if !bytes.is_empty() {
                remainder[..bytes.len()].copy_from_slice(bytes);
                self.offset += bytes.len();
            }
            Ok(())
        }
    }

    let mut message_bytes = ConsoleBuffer {
        offset: 0,
        data: [0u8; 0xFE]
    };

    // never fails!
    let _ = core::fmt::write(&mut message_bytes, fmt);

    console_put_message(color, &message_bytes.data);
}

fn console_put_message(color: Option<&ConsoleColor>, message_bytes: &[u8]) {
    const CONSOLE_PRINTF: VariableProvider<[u8; 0]> = VariableProvider {
        name: "CONSOLE_PRINTF",
        cache_address: 0x0040917E as *mut _,
        tags_address: 0x0040A844 as *mut _
    };

    assert!(message_bytes.last() == Some(&0u8), "should be null-terminated");

    // SAFETY: VariableProvider is probably right.
    unsafe {
        let what = CONSOLE_PRINTF.get() as *const _;
        let what: unsafe extern "C" fn(color: Option<&ConsoleColor>, fmt: *const u8, arg: *const u8) = transmute(what);
        what(color, b"%s\x00".as_ptr(), message_bytes.as_ptr());
    }
}

const TERMINAL_SALT: u16 = 0x6574;

#[repr(C)]
struct TerminalOutput {
    pub some_id: ID<TERMINAL_SALT>,
    pub unknown1: u32,
    pub unknown2: u8,
    pub text: [u8; 0xFF],
    pub unknown3: u32,
    pub color: ConsoleColor,
    pub timer: u32
}

type TerminalOutputTable = DataTable<TerminalOutput, TERMINAL_SALT>;

const TERMINAL_INITIALIZED: VariableProvider<u8> = VariableProvider {
    name: "TERMINAL_INITIALIZED",
    cache_address: 0x00C8AEE0 as *mut _,
    tags_address: 0x00D42490 as *mut _
};

const TERMINAL_OUTPUT_TABLE: VariableProvider<Option<&mut TerminalOutputTable>> = VariableProvider {
    name: "TERMINAL_OUTPUT_TABLE",
    cache_address: 0x00C8AEE4 as *mut _,
    tags_address: 0x00D42494 as *mut _
};

const LIMIT_TICKS: u32 = 150;
const CONSOLE_FADE_FRAME_RATE: f64 = 30.0;

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
            i.item.timer = (i.item.timer + 1).min(LIMIT_TICKS);
        }
    });
}

const CONSOLE_IS_ACTIVE: VariableProvider<u8> = VariableProvider {
    name: "CONSOLE_IS_ACTIVE",
    cache_address: 0x00C98AE0 as *mut _,
    tags_address: 0x00D500A0 as *mut _
};

#[c_mine]
pub extern "C" fn console_is_active() -> bool {
    // SAFETY: This is known to be valid
    unsafe { *CONSOLE_IS_ACTIVE.get() != 0 }
}

#[c_mine]
pub unsafe extern "C" fn terminal_update() {
    if *TERMINAL_INITIALIZED.get() == 0 {
        return
    }

    const GET_CONSOLE_INPUT: VariableProvider<[u8; 0]> = VariableProvider {
        name: "GET_CONSOLE_INPUT",
        cache_address: 0x00649720 as *mut _,
        tags_address: 0x00650F80 as *mut _
    };

    let get_console_input: extern "C" fn() = transmute(GET_CONSOLE_INPUT.get() as *const _);
    let get_console_input = get_console_input();

    let t = TERMINAL_OUTPUT_TABLE
        .get_mut()
        .as_mut()
        .expect("TERMINAL_OUTPUT_TABLE not initialized");

    if !console_is_active.get()() {
        fade_console_text(*t);
    }
}
