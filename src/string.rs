use c_mine::c_mine;
use crate::tag::{get_tag_data_checking_tag_group, TagData, Reflexive, TagGroupUnsafe, TagID, get_tag_info};

// TODO: use generated structs
#[repr(C)]
pub struct UnicodeStringList {
    pub strings: Reflexive<UnicodeStringListString>,
}

#[repr(C)]
pub struct UnicodeStringListString {
    pub data: TagData,
}

#[c_mine]
pub unsafe extern "C" fn unicode_string_list_get_string(tag_id: TagID, index: u16) -> *const u16 {
    let index = index as usize;

    // TODO: use an enum for the unicode_string_list tag
    let tag_data = get_tag_data_checking_tag_group(TagGroupUnsafe(0x75737472), tag_id)
        .expect("unicode_string_list_get_string failed to get a tag") as *const UnicodeStringList;

    let tag_data = &*tag_data;

    let Some(string) = tag_data.strings.get(index) else {
        const MISSING_STRING: &[u16] = &[
            0x3C, 0x6D, 0x69, 0x73, 0x73, 0x69, 0x6E, 0x67, 0x20, 0x73, 0x74, 0x72, 0x69, 0x6E, 0x67, 0x3E, 0x00
        ];
        return MISSING_STRING.as_ptr();
    };

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

    u16.as_ptr()
}
