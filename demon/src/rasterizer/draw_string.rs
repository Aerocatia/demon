pub mod c;
use core::ptr::null;
use num_enum::TryFromPrimitive;
use c_mine::pointer_from_hook;
use tag_structs::primitives::color::{ColorARGB, ColorRGB, Pixel32};
use tag_structs::primitives::rectangle::Rectangle;
use crate::rasterizer::draw_string::c::{draw_string_set_color, draw_string_set_font, draw_string_set_format, rasterizer_text_set_shadow_color, set_tab_stops};
use crate::tag::TagID;
use crate::util::{PointerProvider, StaticStringBytes, VariableProvider};

pub const RASTERIZER_DRAW_UNICODE_STRING: PointerProvider<unsafe extern "C" fn(
    rectangle: &Rectangle,
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
    a: 1.0,
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
    shadow_color: Option<ColorARGB>,
    tab_stop_count: usize,
    tab_stops: [i16; MAXIMUM_NUMBER_OF_TAB_STOPS],
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
            None,
            &[],
            color
        )
    }
    pub fn new_full(font_tag: TagID,
                    style: DrawStringStyle,
                    justification: DrawStringJustification,
                    flags: DrawStringFlags,
                    shadow_color: Option<ColorARGB>,
                    tab_stops: &[i16],
                    color: ColorARGB) -> Self {
        let mut writer = Self {
            font_tag, style, justification, flags, tab_stops: Default::default(), tab_stop_count: 0, color, shadow_color
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
    pub fn set_shadow_color(&mut self, color: Option<ColorARGB>) {
        self.shadow_color = color;
    }
    pub fn set_flags(&mut self, flags: DrawStringFlags) {
        self.flags = flags;
    }
    pub fn set_tab_stops(&mut self, tab_stops: &[i16]) {
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
    pub unsafe fn draw(&self, fmt: core::fmt::Arguments, bounds: Rectangle) -> core::fmt::Result {
        // Skip processing
        if self.color.a == 0.0 {
            return Ok(())
        }

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

        if let Some(n) = self.shadow_color {
            rasterizer_text_set_shadow_color.get()(n.to_pixel32());
        }
        draw_string_set_font.get()(self.font_tag);
        draw_string_set_format.get()(self.style as u16, self.justification as u16, self.flags.0);
        draw_string_set_color.get()(Some(&self.color));
        set_tab_stops(&self.tab_stops[..self.tab_stop_count]);

        RASTERIZER_DRAW_UNICODE_STRING.get()(&bounds, null(), null(), 0, doubled_up_buffer.as_ptr());

        // prevent subsequent calls from using a possibly broken color, tab stops, etc.
        draw_string_set_color.get()(Some(&DEFAULT_WHITE));
        set_tab_stops(&[]);
        if self.shadow_color.is_some() {
            rasterizer_text_set_shadow_color.get()(Pixel32::default())
        }

        Ok(())
    }
}

pub const MAXIMUM_NUMBER_OF_TAB_STOPS: usize = 16;

const TAB_STOP_COUNT: VariableProvider<u16> = variable! {
    name: "TAB_STOP_COUNT",
    cache_address: 0x00F457F8,
    tag_address: 0x00FFDD58
};

const TAB_STOPS: VariableProvider<[i16; 0x10]> = variable! {
    name: "TAB_STOPS",
    cache_address: 0x00F457FA,
    tag_address: 0x00FFDD5A
};


#[derive(Default)]
#[repr(C)]
struct RasterizerTextRenderingGlobals {
    pub horizontal_scale: f32,
    pub unknown_0x04: f32,
    pub unknown_0x08: f32,
    pub unknown_0x0c: f32,
    pub unknown_0x10: f32,
    pub vertical_scale_neg: f32,
    pub unknown_0x18: f32,
    pub vertical_offset: f32,
    pub unknown_0x20: f32,
    pub unknown_0x24: f32,
    pub unknown_0x28: f32,
    pub unknown_0x2c: f32,
    pub unknown_0x30: f32,
    pub unknown_0x34: f32,
    pub unknown_0x38: f32,
    pub text_scaling: f32,
    pub unknown_0x40: f32,
    pub unknown_0x44: f32,
    pub unknown_0x48: f32,
    pub unknown_0x4c: f32,
}

const RASTERIZER_TEXT_RENDERING_GLOBALS: VariableProvider<RasterizerTextRenderingGlobals> = variable! {
    name: "rasterizer_text_rendering_globals",
    cache_address: 0x00E10900,
    tag_address: 0x00EC7EC0
};

pub unsafe fn set_rasterizer_text_rendering_scaling_to_canvas(canvas: Rectangle) {
    let width = canvas.width() as f32;
    let height = canvas.height() as f32;

    *RASTERIZER_TEXT_RENDERING_GLOBALS.get_mut() = RasterizerTextRenderingGlobals {
        horizontal_scale: 2.0 / width,
        unknown_0x0c: -(1.0 + 1.0 / width),
        vertical_scale_neg: -2.0 / height,
        vertical_offset: 1.0 + 1.0 / height,
        unknown_0x2c: 0.5,
        text_scaling: 1.0,
        unknown_0x4c: 1.0,
        ..RasterizerTextRenderingGlobals::default()
    }
}
