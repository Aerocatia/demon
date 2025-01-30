use c_mine::pointer_from_hook;
use crate::util::PointerProvider;

pub mod game_engine;
pub mod server;

pub unsafe fn play_multiplayer_sound(what: u32) {
    const GAME_ENGINE_PLAY_MULTIPLAYER_SOUND: PointerProvider<unsafe extern "C" fn(index: u32, something: bool)> = pointer_from_hook!("game_engine_play_multiplayer_sound");
    GAME_ENGINE_PLAY_MULTIPLAYER_SOUND.get()(what, false)
}
