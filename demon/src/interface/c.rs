use core::ffi::c_int;
use c_mine::c_mine;

#[c_mine]
pub extern "C" fn hud_chat_open(chat_type: c_int) {
    match chat_type {
        0 => console!("TODO: hud_chat_open (ALL)"),
        1 => console!("TODO: hud_chat_open (TEAM)"),
        2 => console!("TODO: hud_chat_open (VEHICLE)"),
        _ => panic!("hud_chat_open cannot open {chat_type}-type chat")
    }
}
