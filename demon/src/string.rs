use utf16_lit::utf16_null;
use c_mine::c_mine;
use crate::tag::{get_tag_data_checking_tag_group, TagData, Reflexive, TagID, get_tag_info, TagGroup};

// TODO: use generated structs
#[repr(C)]
pub struct UnicodeStringList {
    pub strings: Reflexive<UnicodeStringListString>,
}

#[repr(C)]
pub struct UnicodeStringListString {
    pub data: TagData,
}

pub unsafe fn get_unicode_string_list_string(tag_id: TagID, index: u16) -> Option<&'static [u16]> {
    let index = index as usize;

    let tag_data = get_tag_data_checking_tag_group(tag_id, TagGroup::UnicodeStringList.into())
        .expect("unicode_string_list_get_string failed to get a tag") as *const UnicodeStringList;

    let tag_data = &*tag_data;

    let string = tag_data.strings.get(index)?;

    let data = string.data.as_slice();
    let len = data.len();
    if len % size_of::<u16>() != 0 {
        let tag = get_tag_info(tag_id).expect("we got the tag earlier");
        panic!("unicode_string_list_get_string tried to get a string that doesn't divide into u16s (tag={}, index={index})", tag.get_tag_path())
    }

    let u16 = core::slice::from_raw_parts(data.as_ptr() as *const u16, len / size_of::<u16>());
    if u16.last() != Some(&0x00u16) {
        let tag = get_tag_info(tag_id).expect("we got the tag earlier");
        panic!("unicode_string_list_get_string tried to get a non-null terminated string (tag={}, index={index})", tag.get_tag_path())
    }

    Some(u16)
}

#[c_mine]
pub unsafe extern "C" fn unicode_string_list_get_string(tag_id: TagID, index: u16) -> *const u16 {
    const MISSING_STRING: &[u16] = &utf16_null!("<missing string>");
    get_unicode_string_list_string(tag_id, index).unwrap_or(MISSING_STRING).as_ptr()
}
