use core::mem::transmute;
use c_mine::c_mine;
use crate::timing::FixedTimer;
use crate::util::VariableProvider;

const GAME_ENGINE: VariableProvider<Option<&mut [u8; 0]>> = VariableProvider {
    name: "game_engine",
    cache_address: 0x00C56FF4 as *mut _,
    tags_address: 0x00D0E5AC as *mut _,
};

// 1 = game_engine_mode_postgame_delay
const GAME_ENGINE_GLOBALS_MODE: VariableProvider<u32> = VariableProvider {
    name: "game_engine_globals.mode",
    cache_address: 0x00C56FDC as *mut _,
    tags_address: 0x00D0E594 as *mut _,
};

const SCOREBOARD_FADE: VariableProvider<f32> = VariableProvider {
    name: "scoreboard_fade",
    cache_address: 0x00C56FE0 as *mut _,
    tags_address: 0x00D0E598 as *mut _,
};

const RULES_FADE: VariableProvider<f32> = VariableProvider {
    name: "rules_fade",
    cache_address: 0x00C56FE4 as *mut _,
    tags_address: 0x00D0E59C as *mut _,
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
            const A: VariableProvider<[u8; 0]> = VariableProvider {
                name: "a",
                cache_address: 0x005A4360 as *mut _,
                tags_address: 0x005AA4C0 as *mut _,
            };
            let function: extern "C" fn() = transmute(A.get() as *const _);
            function();
        },
        2 | 3 => {
            const B: VariableProvider<[u8; 0]> = VariableProvider {
                name: "a",
                cache_address: 0x00404089 as *mut _,
                tags_address: 0x004032A6 as *mut _,
            };
            let function: extern "C" fn() = transmute(B.get() as *const _);
            function();
        },
        n => panic!("game_engine_globals.mode is an unexpected value {n}")
    }

    // Evil hack to prevent tied to framerate memes that should be destroyed when this is decomped properly
    if !TIMER.test() {
        *SCOREBOARD_FADE.get_mut() = old_scoreboard_value;
        *RULES_FADE.get_mut() = old_rules_value;
    }
}
