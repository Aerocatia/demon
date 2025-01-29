use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use c_mine::{c_mine, pointer_from_hook};
use crate::input::INPUT_GET_BUTTON_STATE;
use crate::multiplayer::{get_game_engine_globals_mode, GameEngineGlobalsMode};
use crate::player::{get_local_player_index, local_player_get_player_index, PlayerID};
use crate::timing::InterpolatedTimer;
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

const GAME_RULES_FADE: VariableProvider<f32> = variable! {
    name: "rules_fade",
    cache_address: 0x00C56FE4,
    tag_address: 0x00D0E59C
};

pub static SCOREBOARD_FADE_TIMER: InterpolatedTimer = InterpolatedTimer::new(1.0);
pub static RULES_FADE_TIMER: InterpolatedTimer = InterpolatedTimer::new(1.0);
pub static SCOREBOARD_FADE_IN: AtomicBool = AtomicBool::new(false);
pub static RULES_FADE_IN: AtomicBool = AtomicBool::new(false);
pub static SCOREBOARD_FADE_INITIAL: AtomicU32 = AtomicU32::new(0f32.to_bits());
pub static RULES_FADE_INITIAL: AtomicU32 = AtomicU32::new(0f32.to_bits());

const GAME_ENGINE_POST_RASTERIZE_SCOREBOARD: PointerProvider<extern "C" fn()> = pointer_from_hook!("game_engine_post_rasterize_scoreboard");
const GAME_ENGINE_POST_RASTERIZE_POST_GAME: PointerProvider<extern "C" fn()> = pointer_from_hook!("game_engine_post_rasterize_post_game");
const DRAW_SCOREBOARD_SCREEN: PointerProvider<unsafe extern "C" fn(player_id: PlayerID, opacity: f32)> = pointer_from_hook!("draw_scoreboard_screen");
const DRAW_GAME_RULES_SCREEN: PointerProvider<unsafe extern "C" fn(opacity: f32)> = pointer_from_hook!("draw_game_rules_screen");

const FADE_SPEED: f32 = 0.5;

fn handle_fade(button: bool, fade_in: &AtomicBool, initial: &AtomicU32, timer: &InterpolatedTimer, fade: &mut f32) -> f32 {
    let current_value = (*fade).clamp(0.0, 1.0);

    if fade_in.swap(button, Ordering::Relaxed) != button {
        timer.start();
        initial.store(current_value.to_bits(), Ordering::Relaxed);
    }

    let target = if button { 1.0 } else { 0.0 };
    if current_value == target {
        *fade = current_value;
        return current_value;
    }

    let initial = f32::from_bits(initial.load(Ordering::Relaxed));
    let elapsed = (timer.seconds() as f32) / FADE_SPEED.clamp(0.01, 10.0);
    *fade = (initial + elapsed * if button { 1.0 } else { -1.0 }).clamp(0.0, 1.0);

    *fade
}

unsafe fn game_engine_post_rasterize_scoreboard() {
    let local_player = get_local_player_index();
    let player_index = local_player_get_player_index.get()(local_player);

    // Acts as though the scoreboard button is pressed and game rules is not pressed
    let force_show_scoreboard = unsafe { get_game_engine_globals_mode() } == GameEngineGlobalsMode::PostgameDelay;

    let current_scoreboard_fade_value = *SCOREBOARD_FADE.get();
    let current_game_rules_fade_value = *GAME_RULES_FADE.get();

    // TODO: make INPUT_GET_BUTTON_STATE use enums
    let scoreboard = (force_show_scoreboard || INPUT_GET_BUTTON_STATE.get()(local_player, 0xC) != 0) && current_game_rules_fade_value == 0.0;
    let rules = !force_show_scoreboard && INPUT_GET_BUTTON_STATE.get()(local_player, 0x1B) != 0 && current_scoreboard_fade_value == 0.0;

    let scoreboard_fade = handle_fade(scoreboard, &SCOREBOARD_FADE_IN, &SCOREBOARD_FADE_INITIAL, &SCOREBOARD_FADE_TIMER, SCOREBOARD_FADE.get_mut());
    let game_rules_fade = handle_fade(rules, &RULES_FADE_IN, &RULES_FADE_INITIAL, &RULES_FADE_TIMER, GAME_RULES_FADE.get_mut());

    // These screens are mutually exclusive and should not be drawn together
    if scoreboard_fade > 0.0 {
        DRAW_SCOREBOARD_SCREEN.get()(player_index, scoreboard_fade)
    }
    else if game_rules_fade > 0.0 {
        DRAW_GAME_RULES_SCREEN.get()(game_rules_fade)
    }
}

#[c_mine]
pub unsafe extern "C" fn game_engine_post_rasterize() {
    if GAME_ENGINE.get().is_none() {
        return
    }

    match get_game_engine_globals_mode() {
        GameEngineGlobalsMode::Active | GameEngineGlobalsMode::PostgameDelay => {
            game_engine_post_rasterize_scoreboard();
        },
        GameEngineGlobalsMode::PostgameRasterizeDelay | GameEngineGlobalsMode::PostgameRasterize => {
            // This branch will never be hit.
            GAME_ENGINE_POST_RASTERIZE_POST_GAME.get()();
        }
    }
}

#[c_mine]
pub unsafe extern "C" fn game_engine_nonplayer_post_rasterize() {
    match get_game_engine_globals_mode() {
        GameEngineGlobalsMode::Active | GameEngineGlobalsMode::PostgameDelay => {},
        GameEngineGlobalsMode::PostgameRasterizeDelay | GameEngineGlobalsMode::PostgameRasterize => {
            GAME_ENGINE_POST_RASTERIZE_POST_GAME.get()();
        }
    }
}
