pub mod c;

use core::fmt::Display;
use num_enum::FromPrimitive;
use spin::RwLock;
use tag_structs::primitives::color::{ColorARGB, ColorRGB};
use tag_structs::primitives::vector::Rectangle;
use crate::globals::get_interface_fonts;
use crate::id::ID;
use crate::rasterizer::draw_string::{DrawStringJustification, DrawStringWriter};
use crate::rasterizer::font::get_font_tag_height;
use crate::rasterizer::{draw_box, get_global_interface_canvas_bounds};
use crate::scrollback::ScrollbackButton;
use crate::timing::InterpolatedTimer;
use crate::util::{CStrPtr, StaticStringBytes, VariableProvider};

const CONSOLE_FADE_START: f64 = 4.0;
const CONSOLE_FADE_TIME: f64 = 0.5;
const CONSOLE_MAX_TIME_VISIBLE: f64 = CONSOLE_FADE_START + CONSOLE_FADE_TIME;
const CONSOLE_CURSOR: char = '•';
const CONSOLE_PREFIX: &'static str = "halo( ";
const CONSOLE_DISPLAY_PADDING: i16 = 4;
const CONSOLE_ENTRY_MAX_SIZE: usize = 1024;
const CONSOLE_INPUT_MAX_SIZE: usize = 512;
const CONSOLE_MAX_SCROLLBACK: usize = 1024;
const CONSOLE_DEFAULT_TEXT_COLOR: ColorARGB = ColorARGB { a: 1.0, color: ColorRGB { r: 0.7, g: 0.7, b: 0.7 } };
const CONSOLE_BACKGROUND_OPACITY: f32 = 0.7;

const CONSOLE_INPUT_TEXT: VariableProvider<[u8; 256]> = variable! {
    name: "CONSOLE_INPUT_TEXT",
    cache_address: 0x00C98B98,
    tag_address: 0x00D50158
};

const CONSOLE_CURSOR_POSITION: VariableProvider<u16> = variable! {
    name: "CONSOLE_CURSOR_POSITION",
    cache_address: 0x00C98C9E,
    tag_address: 0x000D5025E
};

pub static mut CONSOLE_COLOR: ColorARGB = ColorARGB {
    a: 1.0,
    color: ColorRGB {
        r: 1.0,
        g: 0.3,
        b: 1.0
    }
};

#[derive(Copy, Clone, PartialEq, FromPrimitive)]
#[repr(u16)]
enum ConsoleStyle {
    #[num_enum(default)]
    Default,
    HighContrast
}
pub static mut CONSOLE_STYLE: u16 = ConsoleStyle::Default as u16;

pub static CONSOLE_BUFFER: RwLock<Console> = RwLock::new(Console::new());

pub struct Console {
    lines: [ConsoleEntry; CONSOLE_MAX_SCROLLBACK],
    number_of_lines: usize,
    cursor_timer: InterpolatedTimer,
    next: usize,
    scrollback: ScrollbackButton,
    scrollback_offset: usize,
    new_messages_while_scrolling_back: usize
}

impl Console {
    pub const fn new() -> Self {
        Self {
            number_of_lines: 0,
            next: 0,
            cursor_timer: InterpolatedTimer::new(0.5),
            lines: [const { ConsoleEntry::new() }; CONSOLE_MAX_SCROLLBACK],
            scrollback: ScrollbackButton::new(),
            scrollback_offset: 0,
            new_messages_while_scrolling_back: 0
        }
    }
    pub fn put(&mut self, color: impl AsRef<ColorARGB>, what: impl Display) {
        let color = color.as_ref();
        assert!(color.is_valid(), "Console::put with invalid color!");

        let latest = self.next;
        self.next = (latest + 1) % CONSOLE_MAX_SCROLLBACK;
        self.number_of_lines = (latest + 1).max(self.number_of_lines);

        let latest_line = &mut self.lines[latest];
        *latest_line = ConsoleEntry {
            text: StaticStringBytes::from_display(what),
            default_color: *color,
            timer_started: false,
            ..ConsoleEntry::new()
        };

        if self.scrollback_offset > 0 {
            self.scrollback_offset = (self.scrollback_offset + 1).min(self.number_of_lines.saturating_sub(1));
            self.new_messages_while_scrolling_back += 1;
        }
    }
    pub fn clear(&mut self) {
        self.next = 0;
        self.number_of_lines = 0;
        self.scrollback_offset = 0;
        self.new_messages_while_scrolling_back = 0;
    }
    pub fn poll_scrollback(&mut self, lines_per_page: usize) {
        let scroll_count = self.scrollback.poll();
        let mut value = self.scrollback_offset;

        if scroll_count != 0 {
            if scroll_count > 0 {
                value = value.saturating_add(lines_per_page.saturating_mul(scroll_count as usize));
            }
            else {
                value = value.saturating_sub(lines_per_page.saturating_mul(scroll_count.abs() as usize));
            }

            // Show the last line
            self.set_scrollback_offset(value);
        }
    }
    fn set_scrollback_offset(&mut self, offset: usize) {
        // Show the last line
        self.scrollback_offset = offset.clamp(0, self.number_of_lines.saturating_sub(1));

        // Clear this
        if self.scrollback_offset == 0 {
            self.new_messages_while_scrolling_back = 0;
        }
    }

    fn iterate_messages(&self) -> impl Iterator<Item = &ConsoleEntry> {
        let (front, back) = self
            .lines[..self.number_of_lines]
            .split_at(self.next);

        front
            .into_iter()
            .rev()
            .chain(back.into_iter().rev())
    }
    fn iterate_messages_mut(&mut self) -> impl Iterator<Item = &mut ConsoleEntry> {
        let (front, back) = self
            .lines[..self.number_of_lines]
            .split_at_mut(self.next);

        front
            .into_iter()
            .rev()
            .chain(back.into_iter().rev())
    }
}

// NOTE: It is always valid to have this be initialized with zeroed()
struct ConsoleEntry {
    default_color: ColorARGB,
    text: StaticStringBytes<CONSOLE_ENTRY_MAX_SIZE>,
    life_timer: InterpolatedTimer,
    last_read_timer_value: f64,
    timer_offset: f64,
    timer_started: bool
}
impl ConsoleEntry {
    pub const fn new() -> Self {
        ConsoleEntry {
            default_color: ColorARGB::zeroed(),
            text: StaticStringBytes::new(),
            life_timer: InterpolatedTimer::second_timer(),
            last_read_timer_value: 0.0,
            timer_offset: 0.0,
            timer_started: true
        }
    }
}

unsafe fn render_console() {
    let font = get_interface_fonts().terminal_font;
    if font.is_null() {
        // no console for you ;-;
        return
    }
    let (ascending, leading) = get_font_tag_height(font);
    let line_height = ascending + leading;

    if line_height <= 0 {
        // line height is bullshit; no console
        return
    }
    let console_active = CONSOLE_IS_ACTIVE.get_copied() != 0;
    let console_type: ConsoleStyle = CONSOLE_STYLE.into();

    let mut writer = DrawStringWriter::new_simple(
        font,
        CONSOLE_COLOR
    );

    let interface_bounds = get_global_interface_canvas_bounds();
    let mut bounds = interface_bounds;
    bounds.left += CONSOLE_DISPLAY_PADDING;
    bounds.top += CONSOLE_DISPLAY_PADDING;
    bounds.right -= CONSOLE_DISPLAY_PADDING;
    bounds.bottom -= CONSOLE_DISPLAY_PADDING + line_height;

    let mut console_buffer = CONSOLE_BUFFER.write();

    if console_active {
        let console_input_text_ptr = CStrPtr::from_bytes(CONSOLE_INPUT_TEXT.get());
        let console_input_text_bytes = console_input_text_ptr.as_cstr().to_bytes();
        let text_bounds = Rectangle {
            top: bounds.bottom,
            bottom: bounds.bottom + line_height,
            ..bounds
        };

        if console_type == ConsoleStyle::HighContrast {
            draw_box(ColorARGB { a: CONSOLE_BACKGROUND_OPACITY, color: ColorRGB::BLACK }, interface_bounds);
        }

        let show_cursor = (console_buffer.cursor_timer.value().0 % 2) == 0;
        let mut shown = false;

        let valid = console_input_text_bytes.is_empty() || console_input_text_bytes.iter().all(|b| b.is_ascii());
        if show_cursor {
            if let Ok(s) = console_input_text_ptr.as_cstr().to_str() {
                let cursor_position = *CONSOLE_CURSOR_POSITION.get() as usize;
                let character_count = s.char_indices().count();
                if cursor_position >= character_count {
                    writer.draw(format_args!("{CONSOLE_PREFIX}{s}{CONSOLE_CURSOR}"), text_bounds).expect(";-;");
                    shown = true;
                }
                else {
                    let end = s.char_indices()
                        .skip(cursor_position)
                        .next()
                        .expect("cursor_position is within character_count; this should work");

                    let (before, after) = s.split_at(end.0);
                    let actual_after = &after[end.1.len_utf8()..];
                    writer.draw(format_args!("{CONSOLE_PREFIX}{before}{CONSOLE_CURSOR}{actual_after}"), text_bounds).expect(";-;");
                    shown = true;
                }
            }
        }

        if !shown {
            writer.draw(format_args!("{CONSOLE_PREFIX}{}", console_input_text_ptr.display_lossy()), text_bounds).expect(";-;");
        }

        let unread_messages = console_buffer.new_messages_while_scrolling_back;
        if console_buffer.new_messages_while_scrolling_back > 0 {
            writer.set_justification(DrawStringJustification::Right);

            // halve the alpha to prevent writing over the input with something that might not be readable
            if !console_input_text_bytes.is_empty() {
                writer.set_color(ColorARGB {
                    a: CONSOLE_COLOR.a * 0.5,
                    color: CONSOLE_COLOR.color
                })
            }

            writer.draw(format_args!("(+{unread_messages})"), text_bounds).expect(";-;");
            writer.set_justification(DrawStringJustification::Left);
        }
    }

    // TODO: Don't modify scrollback_offset in this loop; do it when console_active is set to false
    if !console_active || console_buffer.number_of_lines == 0 {
        console_buffer.set_scrollback_offset(0);
    }
    else {
        console_buffer.poll_scrollback({
            let height = (bounds.bottom - bounds.top) as usize;
            height / (line_height as usize)
        });
    };

    let scrollback_offset = console_buffer.scrollback_offset;

    let mut print_line = |text: &str, color: ColorARGB, additional_padding: i16| -> bool {
        writer.set_color(color);

        if bounds.bottom - line_height < 0 {
            return false
        }
        bounds.bottom -= line_height;

        let draw_bounds = Rectangle {
            top: bounds.bottom,
            bottom: bounds.bottom + line_height,
            ..bounds
        };

        let buffer = StaticStringBytes::<CONSOLE_ENTRY_MAX_SIZE>::from_strs(
            text.split("|t").map(|t| [t, "\t"]).flatten()
        );

        let string_to_print = &buffer.as_str()[..buffer.as_str().len() - 1];

        if string_to_print.contains("\t") {
            writer.set_tab_stops(&[160, 320, 480]);
        }
        else {
            writer.set_tab_stops(&[]);
        }

        writer.draw(format_args!("{string_to_print}"), draw_bounds).expect(";-;");
        true
    };

    for (index, entry) in console_buffer
        .iterate_messages_mut()
        .enumerate()
        .skip(scrollback_offset) {
        let mut color = entry.default_color;

        if console_active {
            match console_type {
                ConsoleStyle::Default => {
                    // Text fades out on close
                    entry.last_read_timer_value = entry.last_read_timer_value.min(CONSOLE_FADE_START);
                    entry.timer_offset = entry.last_read_timer_value;
                    entry.life_timer.start();
                }
                ConsoleStyle::HighContrast => {
                    // Text instantly goes away on close
                    entry.timer_offset = CONSOLE_MAX_TIME_VISIBLE;
                    entry.last_read_timer_value = CONSOLE_MAX_TIME_VISIBLE;
                }
            }
        }

        if !console_active {
            if entry.last_read_timer_value >= CONSOLE_MAX_TIME_VISIBLE {
                break;
            }

            // Start the timer on the first frame the text is shown so that the player doesn't miss
            // it (if there's no spew of text following this)
            if !entry.timer_started {
                entry.life_timer.start();
                entry.timer_started = true;
            }

            if !console_active {
                let time = entry.life_timer.seconds() + entry.timer_offset;
                entry.last_read_timer_value = time;
            }

            // Apply fade to the alpha.
            let console_fade_offset = entry.last_read_timer_value - CONSOLE_FADE_START;
            if console_fade_offset > 0.0 {
                color.a = color.a * (1.0 - console_fade_offset / CONSOLE_FADE_TIME).clamp(0.0, 1.0) as f32;
            }
        }

        if !print_line(entry.text.as_str(), color, 0) {
            break
        }
    }
}

extern "C" {
    fn printf(fmt: CStrPtr, ...) -> i32;
}

#[no_mangle]
unsafe extern "C" fn demon_terminal_put(color: Option<&ColorARGB>, text: CStrPtr) {
    printf(CStrPtr::from_bytes(b"[CONSOLE] %s\n\x00"), text);
    CONSOLE_BUFFER
        .write()
        .put(color.unwrap_or(&CONSOLE_DEFAULT_TEXT_COLOR), text.display_lossy());
}


pub static mut SHOW_DEBUG_MESSAGES: u8 = 1;

pub fn show_debug_messages() -> bool {
    // SAFETY: This is probably going to cause UB on systems that aren't x86 Windows, and thus it
    //         should be changed to an atomic when globals are reworked to use atomics.
    //
    //         In this case, the risk is acceptable in the interim.
    //
    unsafe { SHOW_DEBUG_MESSAGES != 0 }
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
    CONSOLE_BUFFER.write().put(color.unwrap_or(&CONSOLE_DEFAULT_TEXT_COLOR), fmt);
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

const CONSOLE_IS_ACTIVE: VariableProvider<u8> = variable! {
    name: "CONSOLE_IS_ACTIVE",
    cache_address: 0x00C98AE0,
    tag_address: 0x00D500A0
};
