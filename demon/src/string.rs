use crate::tag::{get_tag_info_typed, ReflexiveImpl, TagData, TagID};
use c_mine::c_mine;
use tag_structs::UnicodeStringList;
use utf16_lit::utf16_null;

pub unsafe fn get_unicode_string_list_string(tag_id: TagID, index: u16) -> Option<&'static [u16]> {
    let index = index as usize;

    let (unicode_string_list_info, unicode_string_list) = get_tag_info_typed::<UnicodeStringList>(tag_id)
        .expect("unicode_string_list_get_string failed to get a tag");
    let string = unicode_string_list.strings.get(index)?;
    let string_data = string.string;
    let data = string_data.as_slice();
    let len = data.len();
    if len % size_of::<u16>() != 0 {
        panic!("unicode_string_list_get_string tried to get a string that doesn't divide into u16s (tag={unicode_string_list_info}, index={index})")
    }

    let u16 = core::slice::from_raw_parts(data.as_ptr() as *const u16, len / size_of::<u16>());
    if u16.last() != Some(&0x00u16) {
        panic!("unicode_string_list_get_string tried to get a non-null terminated string (tag={unicode_string_list_info}, index={index})")
    }

    Some(u16)
}

#[c_mine]
pub unsafe extern "C" fn unicode_string_list_get_string(tag_id: TagID, index: u16) -> *const u16 {
    const MISSING_STRING: &[u16] = &utf16_null!("<missing string>");
    get_unicode_string_list_string(tag_id, index).unwrap_or(MISSING_STRING).as_ptr()
}
