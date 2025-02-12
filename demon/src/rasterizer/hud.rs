use core::sync::atomic::{AtomicU32, Ordering};
use c_mine::{get_hs_global, pointer_from_hook};
use tag_structs::primitives::color::{ColorARGB, ColorRGB};
use crate::game_engine::GAME_ENGINE_RUNNING;
use crate::globals::get_interface_fonts;
use crate::player::{get_local_player_index, local_player_get_player_index, Player, PlayerID, PLAYERS_TABLE};
use crate::rasterizer::draw_string::{DrawStringJustification, DrawStringWriter};
use crate::rasterizer::font::get_font_tag_height;
use crate::rasterizer::motion_sensor::update_motion_sensor;
use crate::rasterizer::{get_global_interface_canvas_bounds, InterfaceCanvasBounds, Perspective};
use crate::timing::InterpolatedTimer;
use crate::util::{PointerProvider, VariableProvider};

pub mod c;

pub static mut SHOW_FPS: bool = false;

const RASTERIZER_HUD_BEGIN: PointerProvider<extern "C" fn()> = pointer_from_hook!("rasterizer_hud_begin");
const RASTERIZER_HUD_END: PointerProvider<extern "C" fn()> = pointer_from_hook!("rasterizer_hud_end");
const DRAW_TEMPORARY_HUD: PointerProvider<extern "C" fn()> = pointer_from_hook!("draw_temporary_hud");
const CINEMATIC_IN_PROGRESS: PointerProvider<extern "C" fn() -> bool> = pointer_from_hook!("cinematic_in_progress");
const HUD_UNIT_PLAY_SOUNDS: PointerProvider<extern "C" fn(player: &mut Player, hud_scripted_globals: u8) -> bool> = pointer_from_hook!("hud_unit_play_sounds");
const HUD_DRAW_SCREEN_UNKNOWN_0: PointerProvider<extern "C" fn() -> bool> = pointer_from_hook!("hud_draw_screen_unknown_0");
const HUD_DRAW_SCREEN_UNKNOWN_1: PointerProvider<extern "C" fn()> = pointer_from_hook!("hud_draw_screen_unknown_1");
const HUD_DRAW_SCREEN_UNKNOWN_2: PointerProvider<extern "C" fn(PlayerID)> = pointer_from_hook!("hud_draw_screen_unknown_2");
const HUD_RENDER_WEAPON_INTERFACE: PointerProvider<extern "C" fn(&mut Player)> = pointer_from_hook!("hud_render_weapon_interface");
const HUD_RENDER_UNIT_INTERFACE: PointerProvider<extern "C" fn(&mut Player)> = pointer_from_hook!("hud_render_unit_interface");
const HUD_RENDER_DAMAGE_INDICATORS: PointerProvider<extern "C" fn(u16)> = pointer_from_hook!("hud_render_damage_indicators");
const HUD_RENDER_NAV_POINTS: PointerProvider<extern "C" fn(u16)> = pointer_from_hook!("hud_render_nav_points");
const HUD_MESSAGING_UPDATE: PointerProvider<extern "C" fn(u16)> = pointer_from_hook!("hud_messaging_update");

pub const HUD_SCRIPTED_GLOBALS: VariableProvider<Option<&u8>> = variable! {
    name: "hud_scripted_globals",
    cache_address: 0x00C8348C,
    tag_address: 0x00D3AA44
};

pub unsafe fn draw_hud() {
    RASTERIZER_HUD_BEGIN.get()();
    draw_hud_for_local_player(get_local_player_index());
    RASTERIZER_HUD_END.get()();
    if *get_hs_global!("temporary_hud") != 0 {
        DRAW_TEMPORARY_HUD.get()();
    }

    if SHOW_FPS {
        show_fps();
    }
}

unsafe fn draw_hud_for_local_player(local_player_index: u16) {
    let player_id = local_player_get_player_index.get()(local_player_index);
    let Ok(player) = PLAYERS_TABLE
        .get_copied()
        .expect("draw_hud_for_player")
        .get_element(player_id) else {
        error!("Can't draw hud for player {player_id:?} because that ID is not valid!");
        return;
    };

    let player = player.get_mut();
    let perspective = Perspective::from_local_player(local_player_index);

    let game_engine_running = GAME_ENGINE_RUNNING.get()();
    if (!game_engine_running || HUD_DRAW_SCREEN_UNKNOWN_0.get()()) && !CINEMATIC_IN_PROGRESS.get()() {
        HUD_DRAW_SCREEN_UNKNOWN_1.get()();
    }

    if local_player_index == 0 {
        update_motion_sensor();
    }

    let should_show_hud = *HUD_SCRIPTED_GLOBALS.get().unwrap_or(&0);

    if should_show_hud != 0 {
        HUD_DRAW_SCREEN_UNKNOWN_2.get()(player_id);
        if !player.unit.is_null() && perspective.player_has_camera_control() {
            HUD_RENDER_WEAPON_INTERFACE.get()(player);
            HUD_RENDER_UNIT_INTERFACE.get()(player);
            HUD_RENDER_NAV_POINTS.get()(local_player_index);
            HUD_RENDER_DAMAGE_INDICATORS.get()(local_player_index);
        }
    }

    HUD_UNIT_PLAY_SOUNDS.get()(player, should_show_hud);
    HUD_MESSAGING_UPDATE.get()(local_player_index);
}

pub unsafe fn show_fps() {
    pub static TIMER: InterpolatedTimer = InterpolatedTimer::second_timer();
    pub static FPS_SHOWN: AtomicU32 = AtomicU32::new(0);
    pub static FRAMES_IN_LAST_INTERVAL: AtomicU32 = AtomicU32::new(0);
    pub static CURRENT_TIMER_INT: AtomicU32 = AtomicU32::new(u32::MAX);

    let mut current_timer_int_value = CURRENT_TIMER_INT.load(Ordering::Relaxed);
    if current_timer_int_value == u32::MAX {
        TIMER.start();
        current_timer_int_value = 0;
        CURRENT_TIMER_INT.swap(0, Ordering::Relaxed);
    }

    let intervals = TIMER.value().0 as u32;
    if intervals != current_timer_int_value {
        CURRENT_TIMER_INT.swap(intervals, Ordering::Relaxed);
        FPS_SHOWN.store(FRAMES_IN_LAST_INTERVAL.swap(0, Ordering::Relaxed), Ordering::Relaxed);
    }

    FRAMES_IN_LAST_INTERVAL.fetch_add(1, Ordering::Relaxed);

    let fonts = get_interface_fonts();
    let terminal_font = fonts.terminal_font;
    if terminal_font.is_null() {
        return
    }

    let mut writer = DrawStringWriter::new_simple(terminal_font, fonts.hud_text_color);
    writer.set_justification(DrawStringJustification::Right);

    let fps = FPS_SHOWN.load(Ordering::Relaxed);
    let (height, leading) = get_font_tag_height(terminal_font);
    let height = height + leading;
    writer.draw(format_args!("{fps}"), InterfaceCanvasBounds {
        bottom: height,
        ..get_global_interface_canvas_bounds()
    }).expect(";-;");
}
