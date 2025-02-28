use crate::tag::TagID;
use c_mine::c_mine;
use crate::util::{utf16_to_slice, StaticStringBytes};

#[c_mine]
pub unsafe extern "C" fn choose_item_collection_item(item_collection: TagID) -> TagID {
    super::item_collection::choose_item_collection_item(item_collection)
}

#[c_mine]
pub unsafe extern "C" fn keystone_put(message: *const u16) {
    if message.is_null() {
        return
    }

    let string = utf16_to_slice(message);
    let string = StaticStringBytes::<256>::from_utf16(string);
    hud!("{string}")
}
