use num_enum::TryFromPrimitive;
use c_mine::c_mine;
use crate::math::ColorARGB;
use crate::tag::{get_tag_info, TagGroup, TagID};
use crate::util::VariableProvider;

const DRAW_STRING_COLOR: VariableProvider<ColorARGB> = variable! {
    name: "DRAW_STRING_COLOR",
    cache_address: 0x00F457E8,
    tag_address: 0x00FFDD48
};

#[derive(TryFromPrimitive)]
#[repr(u16)]
pub enum DrawStringStyle {
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
#[repr(u16)]
pub enum DrawStringJustification {
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

/// Safety: Unsafe because we cannot determine if the color is being accessed concurrently.
#[c_mine]
pub unsafe extern "C" fn draw_string_set_color(color: Option<&ColorARGB>) {
    let color = color.expect("draw_string_set_color with a null color");
    assert!(color.is_valid(), "draw_string_set_color with an invalid color {color:?}");
    *DRAW_STRING_COLOR.get_mut() = *color;
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
