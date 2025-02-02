use crate::tag::{get_tag_data_checking_tag_group, TagGroup, TagID};

/// Returns the render height and the leading height.
pub unsafe fn get_font_tag_height(font: TagID) -> (u16, u16) {
    // TODO: use definitions!
    let data = get_tag_data_checking_tag_group(font, TagGroup::Font.into())
        .expect("no font tag ;-;")
        .wrapping_byte_add(4);
    (*(data as *const u16) + *(data.wrapping_byte_add(2) as *const u16), *(data.wrapping_byte_add(4) as *const u16))
}
