use c_mine::pointer_from_hook;
use crate::util::PointerProvider;

pub mod mouse;

pub const INPUT_GET_BUTTON_STATE: PointerProvider<unsafe extern "C" fn(local_player_index: u16, button: u16) -> u8> = pointer_from_hook!("input_get_button_state");
