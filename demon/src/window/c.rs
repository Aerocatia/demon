use c_mine::c_mine;
use crate::window::on_window_focus_change;

#[c_mine]
pub unsafe extern "C" fn window_set_focus(unfocused: u8) {
    on_window_focus_change(unfocused == 0);
}
