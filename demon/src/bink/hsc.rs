use crate::bink::play_bink;
use crate::multiplayer::{get_game_connection_state, GameConnectionState};
use crate::script::{HS_MACRO_FUNCTION_EVALUATE, HS_RETURN};
use crate::util::CStrPtr;
use core::ffi::{c_char, CStr};
use core::mem::transmute;

pub unsafe extern "C" fn play_bink_eval(a: u16, b: u32, c: u8) {
    let path: *const *const c_char = transmute(HS_MACRO_FUNCTION_EVALUATE.get()(a, b, c));
    if !path.is_null() {
        if get_game_connection_state() == GameConnectionState::None {
            play_bink(CStrPtr::from_cstr(CStr::from_ptr(*path)).expect_str())
        }
        else {
            error!("Cannot play binks in multiplayer")
        }
        HS_RETURN.get()(b, 0);
    }
}
