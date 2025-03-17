use core::intrinsics::transmute;
use c_mine::pointer_from_hook;
use crate::multiplayer::chat::{send_chat_message, MessageChannel};
use crate::script::{HS_MACRO_FUNCTION_EVALUATE, HS_RETURN};
use crate::util::{CStrPtr, PointerProvider};

pub unsafe extern "C" fn sv_map_restart_eval(a: u32, b: u32, c: u32) {
    // TODO: Determine if `begin_end_game` advances the map cycle. If so, we need to make it not do
    //       this once sv_mapcycle_* is added.
    const BEGIN_END_GAME: PointerProvider<unsafe extern "C" fn()> = pointer_from_hook!("begin_end_game");
    BEGIN_END_GAME.get()();
    HS_RETURN.get()(b, 0);
}

pub unsafe extern "C" fn demon_send_chat_message_eval(a: u16, b: u32, c: u8) {
    #[repr(C)]
    struct DemonChatMessageStruct {
        channel: u16,
        _padding: u16,
        string: CStrPtr
    }

    let v: Option<&DemonChatMessageStruct> = transmute(HS_MACRO_FUNCTION_EVALUATE.get()(a,b,c));
    if let Some(message) = v {
        if let Ok(channel) = MessageChannel::try_from(message.channel) {
            if let Some(message) = message.string.to_str_lossless() {
                send_chat_message(channel, message);
            }
            else {
                error!("Cannot send non-UTF-8 chat message.");
            }
        }
        else {
            error!("Invalid channel {} (must be 0-2)", message.channel);
        }
        HS_RETURN.get()(b, 0);
    }
}
