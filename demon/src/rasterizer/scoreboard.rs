use crate::globals::get_interface_fonts;
use crate::input::INPUT_GET_BUTTON_STATE;
use crate::multiplayer::game_engine::{get_game_engine_globals_mode, GameEngineGlobalsMode};
use crate::multiplayer::{get_server_info, ServerInfo};
use crate::player::{get_local_player_index, local_player_get_player_index, PlayerControlsAction, PlayerID, MAXIMUM_NUMBER_OF_PLAYERS, PLAYERS_TABLE};
use crate::timing::InterpolatedTimer;
use crate::util::{PointerProvider, StaticStringBytes, VariableProvider};
use c_mine::{c_mine, pointer_from_hook};
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use num_enum::TryFromPrimitive;

mod strings;
mod sortable_score;
mod verbose;
mod heading;
mod color;

pub use color::USE_PLAYER_COLORS;

use crate::rasterizer::scoreboard::sortable_score::SortableScore;
use crate::rasterizer::scoreboard::verbose::draw_verbose_scoreboard;
use strings::ScoreboardScreenText;
use tag_structs::primitives::float::FloatFunctions;

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
        draw_scoreboard_screen(player_index, scoreboard_fade.powf(1.9))
    }
    else if game_rules_fade > 0.0 {
        DRAW_GAME_RULES_SCREEN.get()(game_rules_fade.powf(1.9))
    }
}

pub static mut SHOW_SERVER_INFO: u8 = 1;
pub static mut SCOREBOARD_STYLE: u16 = 0;
pub static mut USE_TERMINAL_FONT: u8 = 0;

#[derive(Copy, Clone, TryFromPrimitive, Default)]
#[repr(u16)]
pub enum ScoreboardStyle {
    #[default]
    Verbose,
    // Simple
}

unsafe fn format_score<'a>(score: i32, server_info: &ServerInfo) -> StaticStringBytes<32> {
    if server_info.scoring_uses_time() {
        // Sadly we can't show ms here because the server only syncs the time once per second.
        // Also it wouldn't fit the scoreboard.
        let seconds = score / 30;
        let minutes = seconds / 60;
        let seconds_trunc = seconds % 60;
        StaticStringBytes::from_fmt(format_args!("{minutes}:{seconds_trunc:02}")).expect(";-;")
    }
    else {
        StaticStringBytes::from_fmt(format_args!("{score}")).expect(";-;")
    }
}

unsafe fn draw_scoreboard_screen(local_player: PlayerID, opacity: f32) {
    let Some(server_info) = get_server_info() else {
        return
    };

    let fonts = get_interface_fonts();
    let large_font = fonts.full_screen_font;
    let small_font = if USE_TERMINAL_FONT != 0 { fonts.terminal_font } else { fonts.split_screen_font };

    let mut player_ids = [PlayerID::NULL; MAXIMUM_NUMBER_OF_PLAYERS];
    let mut index = 0;
    let player_table = PLAYERS_TABLE.get_mut().as_mut().expect("where is the player table???");
    let mut player_iterator = player_table.iter();
    while index < player_ids.len() && player_iterator.next().is_some() {
        player_ids[index] = player_iterator.id();
        index += 1;
    }
    let local_player_team = player_table
        .get_element(local_player)
        .map(|e| e.get().team)
        .ok();
    let sorted = SortableScore::sort_players_by_score(local_player, local_player_team, &mut player_ids, server_info);
    let style = ScoreboardStyle::try_from(SCOREBOARD_STYLE).unwrap_or_default();
    SCOREBOARD_STYLE = style as u16;
    let scoreboard_text = ScoreboardScreenText::load();
    match style {
        ScoreboardStyle::Verbose => draw_verbose_scoreboard(
            local_player,
            opacity,
            &scoreboard_text,
            large_font,
            small_font,
            &fonts.hud_icon_color,
            &sorted,
            server_info
        )
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
