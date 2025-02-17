use core::ffi::c_char;
use c_mine::c_mine;
use tag_structs::primitives::data::{Data, Reflexive};
use tag_structs::Scenario;
use crate::tag::{get_tag_data_checking_tag_group, get_tag_info, lookup_tag, ReflexiveImpl, TagData, TagGroupUnsafe, TagID, UnknownType, GLOBAL_SCENARIO};
use crate::util::CStrPtr;

#[c_mine]
pub unsafe extern "C" fn tag_get(group: TagGroupUnsafe, id: TagID) -> *mut [u8; 0] {
    get_tag_data_checking_tag_group(id, group).expect("tag_get failed!")
}

#[c_mine]
pub unsafe extern "C" fn tag_block_get_address(reflexive: Option<&Reflexive<UnknownType>>) -> *mut [u8; 0] {
    reflexive.expect("tag_block_get_address with null reflexive").address.0 as *mut _
}

#[c_mine]
pub unsafe extern "C" fn tag_block_get_element_with_size(
    reflexive: Option<&Reflexive<UnknownType>>,
    index: usize,
    element_size: usize
) -> *mut UnknownType {
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

    let objects = reflexive.address.0 as *mut UnknownType;
    objects.wrapping_byte_offset(offset)
}

#[c_mine]
pub unsafe extern "C" fn global_scenario_get() -> &'static mut Scenario {
    GLOBAL_SCENARIO
        .get_copied()
        .expect("global_scenario_get(): global_scenario is null!")
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

#[c_mine]
pub unsafe extern "C" fn tag_get_name(tag_id: TagID) -> CStrPtr {
    let path = get_tag_info(tag_id)
        .expect("failed to get the tag name")
        .get_tag_path()
        .as_ptr() as *const c_char;
    CStrPtr(path)
}

#[c_mine]
pub unsafe extern "C" fn get_data_address(data: &mut Data) -> *mut u8 {
    data.as_mut_slice().as_mut_ptr()
}
