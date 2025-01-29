use num_enum::TryFromPrimitive;
use c_mine::c_mine;
use crate::multiplayer::get_game_engine_globals_mode;
use crate::timing::{FixedTimer, TICK_RATE};
use crate::util::{PointerProvider, VariableProvider};

const GAME_ENGINE: VariableProvider<Option<&mut [u8; 0]>> = variable! {
    name: "game_engine",
    cache_address: 0x00C56FF4,
    tag_address: 0x00D0E5AC
};

const SCOREBOARD_FADE: VariableProvider<f32> = variable! {
    name: "scoreboard_fade",
    cache_address: 0x00C56FE0,
    tag_address: 0x00D0E598
};

const RULES_FADE: VariableProvider<f32> = variable! {
    name: "rules_fade",
    cache_address: 0x00C56FE4,
    tag_address: 0x00D0E59C
};

#[derive(Copy, Clone, TryFromPrimitive, Debug)]
#[repr(u32)]
pub enum GameEngineGlobalsMode {
    Active = 0,
    PostgameDelay = 1,
    PostgameRasterizeDelay = 2,

    // unknown what the original name was
    PostgameRasterize = 3
}

#[c_mine]
pub unsafe extern "C" fn game_engine_post_rasterize() {
    static TIMER: FixedTimer = FixedTimer::new(1.0 / TICK_RATE, 30);

    if GAME_ENGINE.get().is_none() {
        return
    }

    let old_scoreboard_value = *SCOREBOARD_FADE.get();
    let old_rules_value = *RULES_FADE.get();

    match get_game_engine_globals_mode() {
        GameEngineGlobalsMode::Active | GameEngineGlobalsMode::PostgameDelay => {
            const A: PointerProvider<extern "C" fn()> = pointer! {
                name: "a",
                cache_address: 0x005A4360,
                tag_address: 0x005AA4C0
            };
            A.get()();
        },
        GameEngineGlobalsMode::PostgameRasterizeDelay | GameEngineGlobalsMode::PostgameRasterize => {
            const B: PointerProvider<extern "C" fn()> = pointer! {
                name: "b",
                cache_address: 0x00404089,
                tag_address: 0x004032A6
            };
            B.get()();
        }
    }

    // Evil hack to prevent tied to framerate memes that should be destroyed when this is decomped properly
    if !TIMER.test() {
        *SCOREBOARD_FADE.get_mut() = old_scoreboard_value;
        *RULES_FADE.get_mut() = old_rules_value;
    }
}
