use core::ffi::{c_char, CStr};
use core::mem::transmute;
use c_mine::pointer_from_hook;
use crate::multiplayer::{get_game_connection_state, GameConnectionState};
use crate::script::{HS_MACRO_FUNCTION_EVALUATE, HS_RETURN};
use crate::util::{CStrPtr, PointerProvider};

const PLAY_BINK: PointerProvider<unsafe extern "C" fn(CStrPtr)> = pointer_from_hook!("play_bink");

pub unsafe extern "C" fn play_bink_eval(a: u16, b: u32, c: u8) {
    let path: *const *const c_char = transmute(HS_MACRO_FUNCTION_EVALUATE.get()(a, b, c));
    if !path.is_null() {
        if get_game_connection_state() == GameConnectionState::None {
            PLAY_BINK.get()(CStrPtr::from_cstr(CStr::from_ptr(*path)));
        }
        else {
            error!("Cannot play binks in multiplayer")
        }
        HS_RETURN.get()(b, 0);
    }
}
