use core::ffi::c_int;
use utf16_lit::utf16_null;
use c_mine::c_mine;
use crate::string::get_unicode_string_list_string;
use crate::tag::TagID;
use crate::util::CStrPtr;

#[c_mine]
pub unsafe extern "C" fn unicode_string_list_get_string(tag_id: TagID, index: u16) -> *const u16 {
    const MISSING_STRING: &[u16] = &utf16_null!("<missing string>");
    get_unicode_string_list_string(tag_id, index).unwrap_or(MISSING_STRING).as_ptr()
}

#[c_mine]
pub unsafe extern "C" fn strcmp(a: CStrPtr, b: CStrPtr) -> c_int {
    a.as_cstr().to_bytes().cmp(b.as_cstr().to_bytes()) as c_int
}

#[c_mine]
pub unsafe extern "C" fn strlen(a: CStrPtr) -> usize {
    a.as_cstr().count_bytes()
}

#[c_mine]
pub unsafe extern "C" fn strcpy(to: *mut u8, from: CStrPtr) {
    let slice_from = from.as_cstr().to_bytes_with_nul();
    let slice_to = core::slice::from_raw_parts_mut(to, slice_from.len());
    slice_to.copy_from_slice(slice_from)
}
