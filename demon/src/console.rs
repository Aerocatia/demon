pub mod c;

use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::fmt::Display;
use num_enum::FromPrimitive;
use spin::RwLock;
use windows_sys::Win32::UI::Input::KeyboardAndMouse;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{GetKeyState, VIRTUAL_KEY};
use windows_sys::Win32::UI::WindowsAndMessaging::{WM_CHAR, WM_KEYDOWN};
use tag_structs::primitives::color::{ColorARGB, ColorRGB};
use tag_structs::primitives::vector::Rectangle;
use crate::console::c::run_console_command;
use crate::globals::get_interface_fonts;
use crate::rasterizer::draw_string::{DrawStringJustification, DrawStringWriter};
use crate::rasterizer::font::get_font_tag_height;
use crate::rasterizer::{draw_box, get_global_interface_canvas_bounds};
use crate::timing::InterpolatedTimer;
use crate::util::{decode_win32_character, CStrPtr, StaticStringBytes, VariableProvider};

const CONSOLE_FADE_START: f64 = 4.0;
const CONSOLE_FADE_TIME: f64 = 0.5;
const CONSOLE_MAX_TIME_VISIBLE: f64 = CONSOLE_FADE_START + CONSOLE_FADE_TIME;
const CONSOLE_CURSOR: char = '•';
const CONSOLE_PREFIX: &'static str = "halo( ";
const CONSOLE_DISPLAY_PADDING: i16 = 4;
const CONSOLE_ENTRY_MAX_SIZE: usize = 1024;
const CONSOLE_INPUT_MAX_SIZE: usize = 512;
const CONSOLE_MAX_INPUT_HISTORY: usize = 2000;
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

static CONSOLE_COLOR: RwLock<ColorARGB> = RwLock::new(ColorARGB {
    a: 1.0,
    color: ColorRGB {
        r: 1.0,
        g: 0.3,
        b: 1.0
    }
});

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

    history: Vec<Arc<String>>,
    history_offset: usize,

    input_text: String,
    input_cursor_position: usize,
    input_cursor_timer: InterpolatedTimer,

    scrollback_queue: isize,
    scrollback_offset: usize,
    active: bool,

    number_of_lines: usize,
    next: usize,
    new_messages_while_scrolling_back: usize
}

impl Console {
    pub const fn new() -> Self {
        Self {
            number_of_lines: 0,
            next: 0,
            input_text: String::new(),
            input_cursor_timer: InterpolatedTimer::new(0.5),
            input_cursor_position: 0,
            history_offset: 0,
            history: Vec::new(),
            scrollback_queue: 0,
            scrollback_offset: 0,
            active: false,
            lines: [const { ConsoleEntry::new() }; CONSOLE_MAX_SCROLLBACK],
            new_messages_while_scrolling_back: 0
        }
    }
    pub fn put_message(&mut self, color: impl AsRef<ColorARGB>, what: impl Display) {
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
    fn insert_input_char(&mut self, character: char) {
        let mut bytes = [0u8; 4];
        let string = character.encode_utf8(&mut bytes);
        self.input_text.insert_str(self.input_cursor_position, string);
        self.move_input_cursor(self.input_cursor_position + string.len());
    }
    fn pop_input_char(&mut self, forward: bool) -> Option<char> {
        self.input_cursor_timer.start();
        if forward {
            if self.input_cursor_position < self.input_text.len() {
                Some(self.input_text.remove(self.input_cursor_position))
            }
            else {
                None
            }
        }
        else {
            if self.input_cursor_position == 0 {
                None
            }
            else {
                self.char_shift_input_cursor(-1);
                Some(self.input_text.remove(self.input_cursor_position))
            }
        }
    }
    fn move_input_cursor(&mut self, new_position: usize) {
        if !self.input_text.is_char_boundary(new_position) {
            panic!("trying to move the input cursor to a non-char buffer");
        }
        self.input_cursor_position = new_position.clamp(0, self.input_text.len());
        self.input_cursor_timer.start();
    }
    fn char_shift_input_cursor(&mut self, chars: isize) {
        if chars == 0 {
            return
        }

        let forward = |position: &mut usize, len: usize| {
            *position = (*position + 1).clamp(0, len);
        };

        let backward = |position: &mut usize, len: usize| {
            *position = position.saturating_sub(1).clamp(0, len);
        };

        let advance: fn(&mut usize, usize) = if chars < 0 { backward } else { forward };
        let chars = chars.abs() as usize;

        for _ in 0..chars {
            advance(&mut self.input_cursor_position, self.input_text.len());
            while !self.input_text.is_char_boundary(self.input_cursor_position) {
                advance(&mut self.input_cursor_position, self.input_text.len());
            }
        }

        self.input_cursor_timer.start();
    }
    fn word_shift_input_cursor(&mut self, words: isize) {
        if words == 0 { return };

        self.input_cursor_position = self.input_cursor_position.clamp(0, self.input_text.len());
        let input_cursor_address = self.input_text.as_bytes()[self.input_cursor_position..].as_ptr();

        let direction = if words > 0 { 1 } else { 0 };
        let words = words.abs() as usize;

        let word_iter = self.input_text.split_whitespace();

        self.input_cursor_position = if direction > 0 {
            let mut word_iter = word_iter.peekable();
            while let Some(w) = word_iter.peek() {
                if w.as_bytes().as_ptr() >= input_cursor_address {
                    break
                }
                word_iter.next();
            }
            for _ in 1..words {
                word_iter.next();
            }
            match word_iter.next() {
                Some(q) => {
                    // this is definitely within range, and there's no way we can allocate a String more than isize per its requirements
                    let end = q.as_bytes().as_ptr_range().end;
                    unsafe { end.byte_offset_from(self.input_text.as_ptr()) as usize }
                },
                None => self.input_text.len()
            }
        }
        else {
            let mut word_iter = word_iter.rev().peekable();
            while let Some(w) = word_iter.peek() {
                if w.as_bytes().as_ptr() < input_cursor_address {
                    break
                }
                word_iter.next();
            }
            for _ in 1..words {
                word_iter.next();
            }
            match word_iter.next() {
                Some(q) => {
                    // this is definitely within range, and there's no way we can allocate a String more than isize per its requirements
                    unsafe { q.as_ptr().byte_offset_from(self.input_text.as_ptr()) as usize }
                },
                None => 0
            }
        };

        self.input_cursor_timer.start();
    }
    pub const fn clear_messages(&mut self) {
        self.next = 0;
        self.number_of_lines = 0;
        self.scrollback_offset = 0;
        self.new_messages_while_scrolling_back = 0;
    }
    pub fn finalize_input(&mut self) -> Arc<String> {
        // ensure the buffer is null-terminated
        self.input_text.push(0 as char);
        self.input_text.pop();
        self.input_cursor_timer.start();
        let input = Arc::new(core::mem::take(&mut self.input_text));
        self.history.push(input.clone());
        while self.history.len() > CONSOLE_MAX_INPUT_HISTORY {
            self.history.remove(0);
        }
        self.history_offset = self.history.len();
        self.clear_input();
        input
    }
    pub fn clear_input(&mut self) {
        self.input_cursor_position = 0;
        self.input_text.clear();
        self.input_cursor_timer.start();
    }
    fn poll_scrollback(&mut self, lines_per_page: usize) {
        let scroll_count = self.scrollback_queue;
        self.scrollback_queue = 0;
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
    let console_type: ConsoleStyle = CONSOLE_STYLE.into();

    let console_color = CONSOLE_COLOR.read();
    let mut writer = DrawStringWriter::new_simple(
        font,
        *console_color
    );

    let interface_bounds = get_global_interface_canvas_bounds();
    let mut bounds = interface_bounds;
    bounds.left += CONSOLE_DISPLAY_PADDING;
    bounds.top += CONSOLE_DISPLAY_PADDING;
    bounds.right -= CONSOLE_DISPLAY_PADDING;
    bounds.bottom -= CONSOLE_DISPLAY_PADDING + line_height;

    let mut console_buffer = CONSOLE_BUFFER.write();
    let console_active = console_buffer.active;
    *CONSOLE_IS_ACTIVE_HALO.get_mut() = console_active as u8;

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

        let show_cursor = (console_buffer.input_cursor_timer.value().0 % 2) == 0;
        if show_cursor {
            let (start, end) = console_buffer.input_text.split_at(console_buffer.input_cursor_position);
            let mut chars_after_cursor = end.char_indices();
            let _ = chars_after_cursor.next(); // skip the first one since the cursor replaces it
            let actual_end = if let Some((c, _)) = chars_after_cursor.next() {
                &end[c..]
            }
            else {
                ""
            };
            writer.draw(format_args!("{CONSOLE_PREFIX}{start}{CONSOLE_CURSOR}{actual_end}"), text_bounds).expect(";-;");
        }
        else {
            writer.draw(format_args!("{CONSOLE_PREFIX}{}", console_buffer.input_text), text_bounds).expect(";-;");
        }

        let unread_messages = console_buffer.new_messages_while_scrolling_back;
        if console_buffer.new_messages_while_scrolling_back > 0 {
            writer.set_justification(DrawStringJustification::Right);

            // halve the alpha to prevent writing over the input with something that might not be readable
            if !console_input_text_bytes.is_empty() {
                writer.set_color(ColorARGB {
                    a: console_color.a * 0.5,
                    color: console_color.color
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
        .put_message(color.unwrap_or(&CONSOLE_DEFAULT_TEXT_COLOR), text.display_lossy());
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

pub fn console_put_args(color: Option<&ColorARGB>, fmt: core::fmt::Arguments) {
    CONSOLE_BUFFER.write().put_message(color.unwrap_or(&CONSOLE_DEFAULT_TEXT_COLOR), fmt);
}

pub(crate) const CONSOLE_IS_ACTIVE_HALO: VariableProvider<u8> = variable! {
    name: "CONSOLE_IS_ACTIVE",
    cache_address: 0x00C98AE0,
    tag_address: 0x00D500A0
};

pub fn console_is_active() -> bool {
    CONSOLE_BUFFER.read().active
}

pub(crate) unsafe fn handle_win32_window_message(message: u32, parameter: u32) -> bool {
    let mut console = CONSOLE_BUFFER.write();

    if message == WM_CHAR && parameter == (b'`' as u32) {
        console.active = !console.active;
        console.input_cursor_timer.start();
        console.clear_input();
        return true
    }

    if !console.active {
        return false
    }

    let ctrl_held_down = unsafe {
        GetKeyState(KeyboardAndMouse::VK_LCONTROL as i32) | GetKeyState(KeyboardAndMouse::VK_RCONTROL as i32)
    } & 0x80 != 0;

    if message == WM_CHAR && !ctrl_held_down {
        let character = decode_win32_character(parameter as u8);
        if !character.is_control() {
            console.insert_input_char(character);
        }
    }
    else if message == WM_KEYDOWN {
        match parameter as VIRTUAL_KEY {
            // ctrl-c clears the console
            KeyboardAndMouse::VK_C if ctrl_held_down => {
                console.clear_input();
                console.history_offset = console.history.len();
            }
            // left/right advances 1 character (holding ctrl advances by word instead)
            KeyboardAndMouse::VK_LEFT => {
                if ctrl_held_down {
                    console.word_shift_input_cursor(-1);
                }
                else {
                    console.char_shift_input_cursor(-1);
                }
            },
            KeyboardAndMouse::VK_RIGHT => {
                if ctrl_held_down {
                    console.word_shift_input_cursor(1);
                }
                else {
                    console.char_shift_input_cursor(1);
                }
            },
            // up/down iterates history
            KeyboardAndMouse::VK_UP => {
                console.history_offset = console.history_offset.saturating_sub(1);
                if let Some(n) = console.history.get(console.history_offset) {
                    console.input_text = String::clone(n);
                    console.input_cursor_position = console.input_text.len();
                }
            },
            KeyboardAndMouse::VK_DOWN => {
                console.history_offset = (console.history_offset + 1).clamp(0, console.history.len());
                if let Some(n) = console.history.get(console.history_offset) {
                    console.input_text = String::clone(n);
                    console.input_cursor_position = console.input_text.len();
                }
                else {
                    console.clear_input();
                }
            },
            // pgup/down iterates scrollback
            KeyboardAndMouse::VK_PRIOR => {
                console.scrollback_queue = console.scrollback_queue.saturating_add(1);
            },
            KeyboardAndMouse::VK_NEXT => {
                console.scrollback_queue = console.scrollback_queue.saturating_sub(1);
            },
            // return enters a command
            KeyboardAndMouse::VK_RETURN => {
                let input_text = console.finalize_input();
                drop(console);
                run_console_command.get()(CStrPtr(input_text.as_ptr() as *const _));
            },
            // backspace/del remove characters
            KeyboardAndMouse::VK_BACK => {
                console.pop_input_char(false);
            },
            KeyboardAndMouse::VK_DELETE => {
                console.pop_input_char(true);
            },
            // tab completion
            KeyboardAndMouse::VK_TAB => {
                console.put_message(ColorARGB::from(ColorRGB::WHITE), "TODO: Tab completion!");
            }
            _ => ()
        }
    }

    true
}

fn set_console_color(color: ColorRGB) {
    *CONSOLE_COLOR.write() = color.into()
}
