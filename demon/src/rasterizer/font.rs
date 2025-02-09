use tag_structs::Font;
use crate::tag::{get_tag_data_checking_tag_group, TagGroup, TagID};

/// Returns the render height and the leading height.
pub unsafe fn get_font_tag_height(font: TagID) -> (u16, u16) {
    let data = get_tag_data_checking_tag_group(font, TagGroup::Font.into())
        .expect("no font tag ;-;")
        as *const Font;
    let font = &*data;

    let total_height = (font.ascending_height + font.descending_height) as u16;
    let leading_height = font.leading_height as u16;

    (total_height, leading_height)
}
