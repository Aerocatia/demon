use num_enum::TryFromPrimitive;
use c_mine::pointer_from_hook;
use crate::multiplayer::get_game_connection_state;
use crate::util::{encode_utf16, PointerProvider};

#[derive(Copy, Clone, PartialEq, TryFromPrimitive)]
#[repr(u16)]
pub enum MessageChannel {
    All = 0,
    Team = 1,
    Vehicle = 2
}

const SEND_CHAT_MESSAGE: PointerProvider<unsafe extern "C" fn(u32, u8, *const u16)> = pointer_from_hook!("send_chat_message");

pub unsafe fn send_chat_message(channel: MessageChannel, message: &str) {
    if !get_game_connection_state().is_connected() {
        error!("Chat message not delivered; you are offline");
        return;
    }

    let string = encode_utf16::<256>(message);
    SEND_CHAT_MESSAGE.get()(channel as u32, 0, string.as_ptr());
}
