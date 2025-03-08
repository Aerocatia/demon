use core::sync::atomic::Ordering;
use crate::tag::TagID;
use c_mine::{c_mine, pointer_from_hook};
use crate::multiplayer::{GameConnectionState, GAME_CONNECTION_STATE};
use crate::util::{utf16_to_slice, PointerProvider, StaticStringBytes};

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

#[c_mine]
pub extern "C" fn set_game_connection_state(state: u16) {
    let state = GameConnectionState::try_from(state).expect("set_game_connection_state with invalid state");
    GAME_CONNECTION_STATE.store(state as u16, Ordering::Relaxed);
}

#[c_mine]
pub extern "C" fn get_game_connection_state() -> GameConnectionState {
    GAME_CONNECTION_STATE.load(Ordering::Relaxed).try_into().expect("get_game_connection_state with invalid state")
}

const DECODE_PACKET_INNER: PointerProvider<unsafe extern "C" fn(*mut u8, *mut u8) -> u32> = pointer_from_hook!("decode_packet_inner");

#[c_mine]
pub unsafe extern "C" fn decode_packet(destination_data: *mut u8, header: *mut u8) -> u32 {
    let r = DECODE_PACKET_INNER.get()(destination_data, header);
    if *((*(header as *mut *mut u8)).add(4)) == 0xF {
        // FIXME: There seems to be a bug with these packets in this build
        *(destination_data as *mut u32) += 256;
    }
    r
}
