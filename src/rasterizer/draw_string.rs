use core::ptr::null;
use num_enum::TryFromPrimitive;
use c_mine::{c_mine, pointer_from_hook};
use crate::math::{ColorARGB, ColorRGB};
use crate::rasterizer::InterfaceCanvasBounds;
use crate::tag::{get_tag_info, TagGroup, TagID};
use crate::util::{PointerProvider, StaticStringBytes, VariableProvider};

pub const RASTERIZER_DRAW_UNICODE_STRING: PointerProvider<unsafe extern "C" fn(
    rectangle: *const [u32; 2],
    _unknown_null_1: *const u16,
    _unknown_null_2: *const u32,
    _zero_me_baby: u32,
    text: *const u16
)> = pointer_from_hook!("rasterizer_draw_unicode_string");

const DRAW_STRING_COLOR: VariableProvider<ColorARGB> = variable! {
    name: "DRAW_STRING_COLOR",
    cache_address: 0x00F457E8,
    tag_address: 0x00FFDD48
};

#[derive(Default, Copy, Clone, Debug)]
#[repr(C)]
pub struct DrawStringFlags(u32);

#[derive(TryFromPrimitive)]
#[derive(Default, Copy, Clone, Debug)]
#[repr(u16)]
pub enum DrawStringStyle {
    #[default]
    Plain = 0xFFFF,
    Bold = 0x0000,
    Italic = 0x0001,
    Condensed = 0x0002,
    Underline = 0x0003
}

const DRAW_STRING_STYLE: VariableProvider<DrawStringStyle> = variable! {
    name: "DRAW_STRING_STYLE",
    cache_address: 0x00F457E4,
    tag_address: 0x00FFDD44
};

// FIXME: Use definitions
#[derive(TryFromPrimitive)]
#[derive(Default, Copy, Clone, Debug)]
#[repr(u16)]
pub enum DrawStringJustification {
    #[default]
    Left,
    Right,
    Center
}

const DRAW_STRING_JUSTIFICATION: VariableProvider<DrawStringJustification> = variable! {
    name: "DRAW_STRING_JUSTIFICATION",
    cache_address: 0x00F457E6,
    tag_address: 0x00FFDD46
};

const DRAW_STRING_FLAGS: VariableProvider<u32> = variable! {
    name: "DRAW_STRING_FLAGS",
    cache_address: 0x00F457E0,
    tag_address: 0x00FFDD40
};

const DRAW_STRING_FONT: VariableProvider<TagID> = variable! {
    name: "DRAW_STRING_FONT",
    cache_address: 0x00F457DC,
    tag_address: 0x00FFDD3C
};

pub const DEFAULT_WHITE: ColorARGB = ColorARGB {
    alpha: 1.0,
    color: ColorRGB {
        r: 1.0,
        g: 1.0,
        b: 1.0
    }
};

pub struct DrawStringWriter {
    font_tag: TagID,
    style: DrawStringStyle,
    justification: DrawStringJustification,
    flags: DrawStringFlags,
    tab_stop_count: usize,
    tab_stops: [u16; MAXIMUM_NUMBER_OF_TAB_STOPS],
    color: ColorARGB
}
impl DrawStringWriter {
    pub fn new_simple(font_tag: TagID,
                      color: ColorARGB) -> Self {
        Self::new_full(
            font_tag,
            DrawStringStyle::Plain,
            DrawStringJustification::Left,
            DrawStringFlags::default(),
            &[],
            color
        )
    }
    pub fn new_full(font_tag: TagID,
                    style: DrawStringStyle,
                    justification: DrawStringJustification,
                    flags: DrawStringFlags,
                    tab_stops: &[u16],
                    color: ColorARGB) -> Self {
        let mut writer = Self {
            font_tag, style, justification, flags, tab_stops: Default::default(), tab_stop_count: 0, color
        };
        writer.set_tab_stops(tab_stops);
        writer
    }
    pub fn set_font_tag(&mut self, tag: TagID) {
        self.font_tag = tag;
    }
    pub fn set_style(&mut self, style: DrawStringStyle) {
        self.style = style;
    }
    pub fn set_justification(&mut self, justification: DrawStringJustification) {
        self.justification = justification;
    }
    pub fn set_color(&mut self, color: ColorARGB) {
        self.color = color;
    }
    pub fn set_flags(&mut self, flags: DrawStringFlags) {
        self.flags = flags;
    }
    pub fn set_tab_stops(&mut self, tab_stops: &[u16]) {
        assert!(tab_stops.len() <= MAXIMUM_NUMBER_OF_TAB_STOPS);
        self.tab_stops[..tab_stops.len()].copy_from_slice(tab_stops);
        self.tab_stop_count = tab_stops.len();
    }

    /// Draws the string with the given bounds box.
    ///
    /// Returns `Err` if formatting failed, with no text being drawn.
    ///
    /// # Panics
    ///
    /// Panics if the current font tag ID is not a font tag.
    ///
    /// # Safety
    ///
    /// This function is not thread-safe, and no guarantees are made that the state of the draw
    /// string parameters are not being done somewhere else concurrently.
    pub unsafe fn draw(&self, fmt: core::fmt::Arguments, bounds: InterfaceCanvasBounds) -> core::fmt::Result {
        // Rust uses UTF-8
        let buffer = StaticStringBytes::<512>::from_fmt(fmt)?;

        // Note that Halo's "unicode", while 16 bits wide, is NOT actual UTF-16 but UCS-2.
        //
        // Basically, each 16-bit word looks up a single character in the font tag to be drawn, with
        // no support for multi-word characters. Thus not all characters are available, such as most
        // emojis.
        //
        // We are intending to add proper UTF-8 support later along with support for TTF/OTF fonts.

        let mut doubled_up_buffer = [0u16; 512];
        let mut encoder = buffer.as_str().encode_utf16();
        doubled_up_buffer.fill_with(|| encoder.next().unwrap_or(0));
        *doubled_up_buffer.last_mut().expect("should be a last character") = 0;

        draw_string_set_font.get()(self.font_tag);
        draw_string_set_format.get()(self.style as u16, self.justification as u16, self.flags.0);
        draw_string_set_color.get()(Some(&self.color));
        set_tab_stops(&self.tab_stops[..self.tab_stop_count]);

        let top_left = (bounds.left as u32) << 16 | (bounds.top as u32);
        let bottom_right = (bounds.right as u32) << 16 | (bounds.bottom as u32);

        let rectangle = [top_left, bottom_right];
        RASTERIZER_DRAW_UNICODE_STRING.get()(&rectangle, null(), null(), 0, doubled_up_buffer.as_ptr());

        // prevent subsequent calls from using a possibly broken color, tab stops, etc.
        draw_string_set_color.get()(Some(&DEFAULT_WHITE));
        set_tab_stops(&[]);

        Ok(())
    }
}

pub const MAXIMUM_NUMBER_OF_TAB_STOPS: usize = 16;

const TAB_STOP_COUNT: VariableProvider<u16> = variable! {
    name: "TAB_STOP_COUNT",
    cache_address: 0x00F457F8,
    tag_address: 0x00FFDD58
};

const TAB_STOPS: VariableProvider<[u16; 0x10]> = variable! {
    name: "TAB_STOPS",
    cache_address: 0x00F457FA,
    tag_address: 0x00FFDD5A
};

/// Safety: Unsafe because we cannot determine if the color is being accessed concurrently.
#[c_mine]
pub unsafe extern "C" fn draw_string_set_color(color: Option<&ColorARGB>) {
    let color = color.expect("draw_string_set_color with a null color");
    assert!(color.is_valid(), "draw_string_set_color with an invalid color {color:?}");
    *DRAW_STRING_COLOR.get_mut() = *color;
}

pub unsafe fn set_tab_stops(tab_stops: &[u16]) {
    let count = tab_stops.len();
    assert!(count <= MAXIMUM_NUMBER_OF_TAB_STOPS, "set_tab_stops with {count} tab stops which is > {MAXIMUM_NUMBER_OF_TAB_STOPS}");

    *TAB_STOP_COUNT.get_mut() = tab_stops.len() as u16;
    if !tab_stops.is_empty() {
        TAB_STOPS.get_mut()[..tab_stops.len()].copy_from_slice(tab_stops);
    }
}

/// Safety: Unsafe because we cannot determine if this is being accessed concurrently.
///
/// Also `new_tab_stops` might not be a valid pointer which is required if count > 0.
///
/// # Panics
///
/// Panics if `count > MAXIMUM_NUMBER_OF_TAB_STOPS`
#[c_mine]
pub unsafe extern "C" fn draw_string_set_tab_stops(new_tab_stops: *const u16, count: u16) {
    if count == 0 {
        *TAB_STOP_COUNT.get_mut() = 0;
        return;
    }

    if count == 0 {
        set_tab_stops(&[]);
    }
    else {
        let new_tab_stops = core::slice::from_raw_parts(new_tab_stops, count as usize);
        set_tab_stops(new_tab_stops);
    }
}

/// Safety: Unsafe because we cannot determine if these values are being accessed concurrently.
#[c_mine]
pub unsafe extern "C" fn draw_string_set_format(style: u16, justification: u16, flags: u32) {
    let style = DrawStringStyle::try_from(style).expect("draw_string_set_format with invalid style");
    let justification = DrawStringJustification::try_from(justification).expect("draw_string_set_format with invalid justification");
    if (flags & 0xFFFFFFF0) != 0 {
        panic!("draw_string_set_format with invalid flags 0x{flags:08X}")
    }
    *DRAW_STRING_FLAGS.get_mut() = flags;
    *DRAW_STRING_STYLE.get_mut() = style;
    *DRAW_STRING_JUSTIFICATION.get_mut() = justification;
}

/// Safety: Unsafe because we cannot determine if these values are being accessed concurrently.
#[c_mine]
pub unsafe extern "C" fn draw_string_set_font(font: TagID) {
    let Some(font_tag) = get_tag_info(font) else {
        panic!("draw_string_set_font with invalid tag ID: {font:?}")
    };
    match font_tag.verify_tag_group(TagGroup::Font.into()) {
        Ok(()) => (),
        Err(e) => panic!("draw_string_set_font got tag {font:?} which is not a font tag {e:?}")
    }
    *DRAW_STRING_FONT.get_mut() = font;
}

/// Safety: Unsafe because we cannot determine if these values are being accessed concurrently.
#[c_mine]
pub unsafe extern "C" fn draw_string_setup(font_tag: TagID, style: u16, justification: u16, flags: u32, color_argb: Option<&ColorARGB>) {
    draw_string_set_font.get()(font_tag);
    draw_string_set_format.get()(style, justification, flags);
    draw_string_set_color.get()(color_argb);
}
