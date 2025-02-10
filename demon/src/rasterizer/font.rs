use crate::tag::{get_tag_data, TagID};
use tag_structs::Font;

/// Returns the render height and the leading height.
pub unsafe fn get_font_tag_height(font: TagID) -> (u16, u16) {
    let font = get_tag_data::<Font>(font).expect("no font tag ;-;");
    let total_height = (font.ascending_height + font.descending_height) as u16;
    let leading_height = font.leading_height as u16;

    (total_height, leading_height)
}
