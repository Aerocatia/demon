use c_mine::c_mine;
use crate::bink::play_bink;
use crate::sound::main_menu_music::{ui_start_main_menu_music, ui_stop_main_menu_music};

#[c_mine]
pub extern "C" fn bink_playback_active() -> bool {
    false
}

#[c_mine]
pub unsafe extern "C" fn game_end_credits_start() {
    ui_stop_main_menu_music.get()();
    // TODO: Move all biks into a bik folder
    play_bink("credits.bik");
    ui_start_main_menu_music.get()();
}
