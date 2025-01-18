use c_mine::c_mine;
use crate::id::ID;
use crate::math::{ColorARGB, ColorRGB};
use crate::table::DataTable;
use crate::timing::FixedTimer;
use crate::util::{PointerProvider, VariableProvider};

pub const ERROR_WAS_SET: VariableProvider<u8> = variable! {
    name: "ERROR_WAS_SET",
    cache_address: 0x00B016C8,
    tags_address: 0x00BB8C80
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
    // format
    let mut data = [0u8; 0xFFE];
    crate::util::fmt_to_byte_array(&mut data, fmt).expect("failed to write error");

    // null terminate
    *data.last_mut().expect("should be a last one???") = 0;

    // done
    error_put_message(priority, &data);
}

pub fn error_put_message(priority: ErrorPriority, error_bytes: &[u8]) {
    const ERROR: PointerProvider<unsafe extern "C" fn(priority: i16, fmt: *const u8, arg: *const u8)> = pointer! {
        name: "ERROR",
        cache_address: 0x00408607,
        tags_address: 0x0040785B
    };

    assert!(error_bytes.last() == Some(&0u8), "should be null-terminated");

    // SAFETY: PointerProvider is probably right.
    unsafe {
        ERROR.get()(priority as i16, b"%s\x00".as_ptr(), error_bytes.as_ptr());
    }
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
/// The first argument must be a [`ColorARGB`] reference.
#[allow(unused_macros)]
macro_rules! console_color {
    ($color:expr, $($args:tt)*) => {{
        let color: &crate::math::ColorARGB = $color;
        crate::console::console_put_args(Some(color), format_args!($($args)*));
    }};
}

pub fn console_put_args(color: Option<&ColorARGB>, fmt: core::fmt::Arguments) {
    // format
    let mut data = [0u8; 0xFE];
    crate::util::fmt_to_byte_array(&mut data, fmt).expect("failed to write console message");

    // null terminate
    *data.last_mut().expect("should be a last one???") = 0;

    // done
    console_put_message(color, &data);
}

fn console_put_message(color: Option<&ColorARGB>, message_bytes: &[u8]) {
    const CONSOLE_PRINTF: PointerProvider<unsafe extern "C" fn(color: Option<&ColorARGB>, fmt: *const u8, arg: *const u8)> = pointer! {
        name: "CONSOLE_PRINTF",
        cache_address: 0x0040917E,
        tags_address: 0x0040A844
    };

    assert!(message_bytes.last() == Some(&0u8), "should be null-terminated");

    // SAFETY: PointerProvider is probably right.
    unsafe {
        CONSOLE_PRINTF.get()(color, b"%s\x00".as_ptr(), message_bytes.as_ptr());
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
    pub color: ColorARGB,
    pub timer: u32
}

type TerminalOutputTable = DataTable<TerminalOutput, TERMINAL_SALT>;

const TERMINAL_INITIALIZED: VariableProvider<u8> = variable! {
    name: "TERMINAL_INITIALIZED",
    cache_address: 0x00C8AEE0,
    tags_address: 0x00D42490
};

const TERMINAL_OUTPUT_TABLE: VariableProvider<Option<&mut TerminalOutputTable>> = variable! {
    name: "TERMINAL_OUTPUT_TABLE",
    cache_address: 0x00C8AEE4,
    tags_address: 0x00D42494
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

const CONSOLE_IS_ACTIVE: VariableProvider<u8> = variable! {
    name: "CONSOLE_IS_ACTIVE",
    cache_address: 0x00C98AE0,
    tags_address: 0x00D500A0
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

    const GET_CONSOLE_INPUT: PointerProvider<extern "C" fn()> = pointer! {
        name: "GET_CONSOLE_INPUT",
        cache_address: 0x00649720,
        tags_address: 0x00650F80
    };

    GET_CONSOLE_INPUT.get()();

    let t = TERMINAL_OUTPUT_TABLE
        .get_mut()
        .as_mut()
        .expect("TERMINAL_OUTPUT_TABLE not initialized");

    if !console_is_active.get()() {
        fade_console_text(*t);
    }
}

const CONSOLE_COLOR: VariableProvider<ColorARGB> = variable! {
    name: "CONSOLE_COLOR",
    cache_address: 0x00C98B68,
    tags_address: 0x00D50128
};

const CONSOLE_PROMPT_TEXT: VariableProvider<[u8; 32]> = variable! {
    name: "CONSOLE_PROMPT_TEXT",
    cache_address: 0x00C98B78,
    tags_address: 0x00D50138
};

const CONSOLE_TEXT: VariableProvider<[u8; 256]> = variable! {
    name: "CONSOLE_TEXT",
    cache_address: 0x00C98B98,
    tags_address: 0x00D50158
};

const CONSOLE_HISTORY_LENGTH: VariableProvider<u16> = variable! {
    name: "CONSOLE_HISTORY_LENGTH",
    cache_address: 0x00C9949C,
    tags_address: 0x00D5015C
};

const CONSOLE_HISTORY_NEXT_INDEX: VariableProvider<u16> = variable! {
    name: "CONSOLE_HISTORY_NEXT_INDEX",
    cache_address: 0x00C9949E,
    tags_address: 0x00D5015E
};

const CONSOLE_HISTORY_SELECTED_INDEX: VariableProvider<u16> = variable! {
    name: "CONSOLE_HISTORY_SELECTED_INDEX",
    cache_address: 0x00C994A0,
    tags_address: 0x00D50160
};

const CONSOLE_ENABLED: VariableProvider<bool> = variable! {
    name: "CONSOLE_ENABLED",
    cache_address: 0x00C98AE1,
    tags_address: 0x00D500A1
};

const DEFAULT_CONSOLE_PROMPT_TEXT: &str = "halo( ";
const DEFAULT_CONSOLE_COLOR: ColorARGB = ColorARGB {
    alpha: 1.0,
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

#[c_mine]
pub unsafe extern "C" fn console_initialize() {
    set_console_prompt_text(DEFAULT_CONSOLE_PROMPT_TEXT);
    set_console_color_text(DEFAULT_CONSOLE_COLOR);
    CONSOLE_TEXT.get_mut().fill(0);
    *CONSOLE_HISTORY_SELECTED_INDEX.get_mut() = 0xFFFF;
    *CONSOLE_HISTORY_LENGTH.get_mut() = 0;
    *CONSOLE_HISTORY_SELECTED_INDEX.get_mut() = 0xFFFF;
    *CONSOLE_ENABLED.get_mut() = true;
}
