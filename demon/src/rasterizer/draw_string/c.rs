use c_mine::c_mine;
use tag_structs::primitives::color::{ColorARGB, Pixel32};
use tag_structs::primitives::tag_group::TagGroup;
use crate::rasterizer::draw_string::{DrawStringJustification, DrawStringStyle, DRAW_STRING_COLOR, DRAW_STRING_FLAGS, DRAW_STRING_FONT, DRAW_STRING_JUSTIFICATION, DRAW_STRING_STYLE, MAXIMUM_NUMBER_OF_TAB_STOPS, TAB_STOPS, TAB_STOP_COUNT};
use crate::tag::{get_tag_info, TagID};
use crate::util::VariableProvider;

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

const TEXT_SHADOW_COLOR: VariableProvider<Pixel32> = variable! {
    name: "TEXT_SHADOW_COLOR",
    cache_address: 0x00ECEA28,
    tag_address: 0x00F85FE8
};

/// Safety: Unsafe because we cannot determine if this is being accessed concurrently.
#[c_mine]
pub unsafe extern "C" fn rasterizer_text_set_shadow_color(color: Pixel32) {
    *TEXT_SHADOW_COLOR.get_mut() = color
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
