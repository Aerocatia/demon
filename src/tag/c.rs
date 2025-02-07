use core::ffi::c_char;
use c_mine::c_mine;
use crate::tag::{get_tag_data_checking_tag_group, lookup_tag, Reflexive, TagGroupUnsafe, TagID, GLOBAL_SCENARIO};
use crate::util::CStrPtr;

#[c_mine]
pub unsafe extern "C" fn tag_get(group: TagGroupUnsafe, id: TagID) -> *mut [u8; 0] {
    get_tag_data_checking_tag_group(id, group).expect("tag_get failed!")
}

#[c_mine]
pub unsafe extern "C" fn tag_block_get_address(reflexive: Option<&Reflexive<[u8; 0]>>) -> *mut [u8; 0] {
    reflexive.expect("tag_block_get_address with null reflexive").objects
}

#[c_mine]
pub unsafe extern "C" fn tag_block_get_element_with_size(
    reflexive: Option<&Reflexive<[u8; 0]>>,
    index: usize,
    element_size: usize
) -> *mut [u8; 0] {
    let reflexive = reflexive.expect("tag_block_get_element_with_size with null reflexive");
    assert!(
        index < reflexive.len(),
        "tag_block_get_element_with_size with out-of-bounds index {index} < {} @ 0x{:08X}",
        reflexive.len(),
        (reflexive as *const _) as usize
    );

    let offset = index.checked_mul(element_size)
        .and_then(|v| isize::try_from(v).ok())
        .expect("tag_block_get_element_with_size with invalid offset/element size");

    reflexive
        .objects
        .wrapping_byte_offset(offset)
}

#[c_mine]
pub unsafe extern "C" fn global_scenario_get() -> *mut u8 {
    let global_scenario = *GLOBAL_SCENARIO.get();
    assert!(!global_scenario.is_null(), "global_scenario_get(): global_scenario is null!");
    global_scenario
}

#[c_mine]
pub unsafe extern "C" fn tag_loaded(group: TagGroupUnsafe, path: CStrPtr) -> TagID {
    lookup_tag(path.as_str(), group).map(|t| t.1).unwrap_or(TagID::NULL)
}

#[c_mine]
pub unsafe extern "C" fn tag_name_strip_path(path: CStrPtr) -> CStrPtr {
    path.as_str()
        .as_bytes()
        .iter()
        .rev()
        .find(|b| **b == b'\\')
        .map(|b| (b as *const u8).wrapping_byte_add(1) as *const c_char)
        .map(CStrPtr)
        .unwrap_or(path)
}
