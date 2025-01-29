use crate::input::INPUT_GET_BUTTON_STATE;
use crate::multiplayer::{get_game_engine_globals_mode, GameEngineGlobalsMode};
use crate::player::{get_local_player_index, local_player_get_player_index, PlayerControlsAction, PlayerID};
use crate::timing::InterpolatedTimer;
use crate::util::{PointerProvider, VariableProvider};
use c_mine::{c_mine, pointer_from_hook};
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use libm::powf;

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

#[derive(Default)]
struct FadingInterface {
    timer: InterpolatedTimer,
    fade_in: AtomicBool,
    initial: AtomicU32
}
impl FadingInterface {
    pub const fn new() -> Self {
        Self {
            timer: InterpolatedTimer::second_timer(),
            fade_in: AtomicBool::new(false),
            initial: AtomicU32::new(0)
        }
    }
    pub fn handle_fade(&self, pressed: bool, fade: &mut f32) -> f32 {
        let current_value = (*fade).clamp(0.0, 1.0);

        if self.fade_in.swap(pressed, Ordering::Relaxed) != pressed {
            self.timer.start();
            self.initial.store(current_value.to_bits(), Ordering::Relaxed);
        }

        let target = if pressed { 1.0 } else { 0.0 };
        if current_value == target {
            *fade = current_value;
            return current_value;
        }

        let initial = f32::from_bits(self.initial.load(Ordering::Relaxed));
        let elapsed = (self.timer.seconds() as f32) / FADE_SPEED.clamp(0.01, 10.0);
        *fade = (initial + elapsed * if pressed { 1.0 } else { -1.0 }).clamp(0.0, 1.0);

        *fade
    }
}

#[derive(Default)]
pub struct FadingScoreboardRules {
    scoreboard: FadingInterface,
    game_rules: FadingInterface
}
pub static FADING_SCOREBOARD_RULES: FadingScoreboardRules = FadingScoreboardRules {
    scoreboard: FadingInterface::new(),
    game_rules: FadingInterface::new()
};

const GAME_ENGINE_POST_RASTERIZE_SCOREBOARD: PointerProvider<extern "C" fn()> = pointer_from_hook!("game_engine_post_rasterize_scoreboard");
const GAME_ENGINE_POST_RASTERIZE_POST_GAME: PointerProvider<extern "C" fn()> = pointer_from_hook!("game_engine_post_rasterize_post_game");
const DRAW_SCOREBOARD_SCREEN: PointerProvider<unsafe extern "C" fn(player_id: PlayerID, opacity: f32)> = pointer_from_hook!("draw_scoreboard_screen");
const DRAW_GAME_RULES_SCREEN: PointerProvider<unsafe extern "C" fn(opacity: f32)> = pointer_from_hook!("draw_game_rules_screen");

const FADE_SPEED: f32 = 0.5;

unsafe fn game_engine_post_rasterize_scoreboard() {
    let local_player = get_local_player_index();
    let player_index = local_player_get_player_index.get()(local_player);

    // Acts as though the scoreboard button is pressed and game rules is not pressed
    let force_show_scoreboard = unsafe { get_game_engine_globals_mode() } == GameEngineGlobalsMode::PostgameDelay;
    let show_scores_button_pressed = force_show_scoreboard || INPUT_GET_BUTTON_STATE.get()(local_player, PlayerControlsAction::ShowScores) != 0;
    let show_rules_button_pressed = !force_show_scoreboard && INPUT_GET_BUTTON_STATE.get()(local_player, PlayerControlsAction::ShowRules) != 0;

    // We don't want to start fading in one thing if another thing is fully in view
    let current_scoreboard_fade_value = *SCOREBOARD_FADE.get();
    let current_game_rules_fade_value = *GAME_RULES_FADE.get();

    // allow some small lee-way to prevent the HUD text from flashing momentarily when transitioning from scoreboard to game rules (or vice versa)
    let scoreboard = show_scores_button_pressed && current_game_rules_fade_value < 0.05;
    let rules = show_rules_button_pressed && current_scoreboard_fade_value < 0.05;

    let fader = &FADING_SCOREBOARD_RULES;
    let scoreboard_fade = fader.scoreboard.handle_fade(scoreboard, SCOREBOARD_FADE.get_mut());
    let game_rules_fade = fader.game_rules.handle_fade(rules, GAME_RULES_FADE.get_mut());

    // These screens are mutually exclusive and should not be drawn together
    // Note that 1.9 is a magic number from the game...
    if scoreboard_fade > 0.0 {
        DRAW_SCOREBOARD_SCREEN.get()(player_index, powf(scoreboard_fade, 1.9))
    }
    else if game_rules_fade > 0.0 {
        DRAW_GAME_RULES_SCREEN.get()(powf(game_rules_fade, 1.9))
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
