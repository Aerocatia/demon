use c_mine::c_mine;
use crate::timing::FixedTimer;
use crate::util::{PointerProvider, VariableProvider};

const GAME_ENGINE: VariableProvider<Option<&mut [u8; 0]>> = variable! {
    name: "game_engine",
    cache_address: 0x00C56FF4,
    tag_address: 0x00D0E5AC
};

// 1 = game_engine_mode_postgame_delay
const GAME_ENGINE_GLOBALS_MODE: VariableProvider<u32> = variable! {
    name: "game_engine_globals.mode",
    cache_address: 0x00C56FDC,
    tag_address: 0x00D0E594
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

#[c_mine]
pub unsafe extern "C" fn game_engine_post_rasterize() {
    static TIMER: FixedTimer = FixedTimer::new(1.0 / 30.0, 30);

    if GAME_ENGINE.get().is_none() {
        return
    }

    let old_scoreboard_value = *SCOREBOARD_FADE.get();
    let old_rules_value = *RULES_FADE.get();

    match *GAME_ENGINE_GLOBALS_MODE.get() {
        0 | 1 => {
            const A: PointerProvider<extern "C" fn()> = pointer! {
                name: "a",
                cache_address: 0x005A4360,
                tag_address: 0x005AA4C0
            };
            A.get()();
        },
        2 | 3 => {
            const B: PointerProvider<extern "C" fn()> = pointer! {
                name: "b",
                cache_address: 0x00404089,
                tag_address: 0x004032A6
            };
            B.get()();
        },
        n => panic!("game_engine_globals.mode is an unexpected value {n}")
    }

    // Evil hack to prevent tied to framerate memes that should be destroyed when this is decomped properly
    if !TIMER.test() {
        *SCOREBOARD_FADE.get_mut() = old_scoreboard_value;
        *RULES_FADE.get_mut() = old_rules_value;
    }
}
