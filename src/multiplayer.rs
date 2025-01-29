use num_enum::TryFromPrimitive;
use c_mine::{c_mine, pointer_from_hook};
use crate::ui::close_all_ui_widgets;
use crate::util::{PointerProvider, VariableProvider};

#[derive(Copy, Clone, TryFromPrimitive, Debug, PartialEq)]
#[repr(u32)]
pub enum GameEngineGlobalsMode {
    /// Game is in progress
    Active = 0,

    /// Game ended; scoreboard is shown
    PostgameDelay = 1,

    /// Postgame Carnage Report without buttons
    PostgameRasterizeDelay = 2,

    // unknown what the original name was
    /// Postgame Carnage Report with buttons
    PostgameRasterize = 3
}

#[c_mine]
pub unsafe extern "C" fn game_engine_end_game() {
    *GAME_ENGINE_GLOBALS_MODE.get_mut() = GameEngineGlobalsMode::PostgameDelay as u32;
    *GAME_ENGINE_GLOBALS_MODE_SWITCH_DELAY.get_mut() = 7.0;
    play_multiplayer_sound(1);
    close_all_ui_widgets();
}

pub const GAME_ENGINE_GLOBALS_MODE: VariableProvider<u32> = variable! {
    name: "game_engine_globals.mode",
    cache_address: 0x00C56FDC,
    tag_address: 0x00D0E594
};
pub const GAME_ENGINE_GLOBALS_MODE_SWITCH_DELAY: VariableProvider<f32> = variable! {
    name: "game_engine_globals.mode_switch_delay", // idk what the original name was
    cache_address: 0x00C56FD4,
    tag_address: 0x00D0E58C
};

pub unsafe fn get_game_engine_globals_mode() -> GameEngineGlobalsMode {
    GameEngineGlobalsMode::try_from(*GAME_ENGINE_GLOBALS_MODE.get()).expect("invalid game engine globals mode")
}

pub unsafe fn play_multiplayer_sound(what: u32) {
    const GAME_ENGINE_PLAY_MULTIPLAYER_SOUND: PointerProvider<unsafe extern "C" fn(index: u32, something: bool)> = pointer_from_hook!("game_engine_play_multiplayer_sound");
    GAME_ENGINE_PLAY_MULTIPLAYER_SOUND.get()(what, false)
}
