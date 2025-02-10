use crate::tag::TagID;
use c_mine::c_mine;

#[c_mine]
pub unsafe extern "C" fn choose_item_collection_item(item_collection: TagID) -> TagID {
    super::item_collection::choose_item_collection_item(item_collection)
}
