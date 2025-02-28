use crate::tag::TagID;
use c_mine::c_mine;
use crate::util::StaticStringBytes;

#[c_mine]
pub unsafe extern "C" fn choose_item_collection_item(item_collection: TagID) -> TagID {
    super::item_collection::choose_item_collection_item(item_collection)
}

#[c_mine]
pub unsafe extern "C" fn keystone_put(message: *const u16) {
    if message.is_null() {
        return
    }

    let mut len = 0usize;
    let mut s = message;
    loop {
        if *s == 0 {
            break
        }
        len += 1;
        s = s.wrapping_add(1);
    }

    let string = core::slice::from_raw_parts(message, len);
    let string = StaticStringBytes::<256>::from_utf16(string);
    hud!("{string}")
}
