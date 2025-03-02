use crate::tag::{get_tag_info_typed, TagID};
use tag_structs::Font;

/// Returns the render height and the leading height.
pub unsafe fn get_font_tag_height(font: TagID) -> (i16, i16) {
    let font = get_tag_info_typed::<Font>(font).unwrap().1;
    let total_height = font.ascending_height + font.descending_height;
    let leading_height = font.leading_height;

    (total_height, leading_height)
}
