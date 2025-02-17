use crate::model::get_model_tag_data;
use crate::tag::TagID;
use crate::util::CStrPtr;
use c_mine::c_mine;
use tag_structs::primitives::data::Index;

#[c_mine]
pub unsafe extern "C" fn model_find_marker(model_tag: TagID, name: CStrPtr) -> usize {
    if name.is_null() || model_tag.is_null() {
        return usize::MAX;
    }

    get_model_tag_data(model_tag)
        .map(|m| m.get_marker(name.as_str()))
        .unwrap_or(Index::new_none())
        .get()
        .unwrap_or(0xFFFFFFFF)
}
