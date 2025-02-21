use c_mine::pointer_from_hook;
use crate::player::PlayerControlsAction;
use crate::util::PointerProvider;

pub mod mouse;
pub mod c;

pub const INPUT_GET_BUTTON_STATE: PointerProvider<unsafe extern "C" fn(local_player_index: u16, button: PlayerControlsAction) -> u8> = pointer_from_hook!("input_get_button_state");

