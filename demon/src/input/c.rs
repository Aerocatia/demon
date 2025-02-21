use c_mine::{c_mine, pointer_from_hook};
use crate::console::{console_is_active, handle_win32_window_message};
use crate::input::mouse::scale_mouse_sensitivity;
use crate::util::PointerProvider;

#[c_mine]
pub unsafe extern "C" fn handle_key_input(message_code: u32, parameter: u32) -> u32 {
    let handled = handle_win32_window_message(message_code, parameter);
    if handled || console_is_active() {
        return 0
    }

    let original: PointerProvider<unsafe extern "C" fn(u32, u32) -> u32> = pointer_from_hook!("handle_key_input_inner");
    original.get()(message_code, parameter)
}

#[c_mine]
pub extern "C" fn mouse_sensitivity(sensitivity: f32, raw_input: i32) -> f32 {
    scale_mouse_sensitivity(sensitivity, raw_input)
}
