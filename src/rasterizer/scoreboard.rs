use crate::input::INPUT_GET_BUTTON_STATE;
use crate::math::{ColorARGB, ColorRGB};
use crate::multiplayer::game_engine::{get_game_engine_globals_mode, GameEngineGlobalsMode};
use crate::multiplayer::{get_connected_ip_address, get_server_info, get_player_score, Gametype, ServerInfo};
use crate::player::{get_local_player_index, local_player_get_player_index, PlayerControlsAction, PlayerID, MAXIMUM_LIVES, MAXIMUM_NUMBER_OF_PLAYERS, PLAYERS_TABLE};
use crate::rasterizer::draw_string::{DrawStringBounds, DrawStringJustification, DrawStringWriter, DEFAULT_WHITE};
use crate::string::get_unicode_string_list_string;
use crate::tag::{get_tag_data_checking_tag_group, lookup_tag, TagGroup, TagID};
use crate::timing::InterpolatedTimer;
use crate::util::{decode_utf16_inplace, fmt_to_byte_array, PointerProvider, VariableProvider};
use c_mine::{c_mine, pointer_from_hook};
use core::fmt::{Display, Formatter};
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use libm::powf;
use num_enum::TryFromPrimitive;
use crate::globals::get_interface_fonts;
use crate::rasterizer::draw_box;
use crate::rasterizer::player_colors::PLAYER_ICON_COLORS;

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
        draw_scoreboard_screen(player_index, powf(scoreboard_fade, 1.9))
    }
    else if game_rules_fade > 0.0 {
        DRAW_GAME_RULES_SCREEN.get()(powf(game_rules_fade, 1.9))
    }
}

#[derive(Copy, Clone, Default)]
struct SizedString32 {
    bytes: [u8; 32],
    size: usize
}
impl SizedString32 {
    pub const fn from_str(string: &str) -> Self {
        let bytes = string.as_bytes();
        let length = bytes.len();
        let mut into = SizedString32 {
            bytes: [0u8; 32],
            size: length
        };
        let mut i = 0;
        while i < length {
            into.bytes[i] = bytes[i];
            i += 1;
        }
        into
    }
    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.bytes[..self.size]).expect("not utf-8???")
    }
}
impl Display for SizedString32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

struct ScoreboardScreenText {
    place: SizedString32,
    name: SizedString32,
    score: SizedString32,
    captures: SizedString32,
    minutes: SizedString32,
    frags: SizedString32,
    time: SizedString32,
    laps: SizedString32,
    kills: SizedString32,
    assists: SizedString32,
    deaths: SizedString32,
    ping: SizedString32,
    quit: SizedString32,
    dead: SizedString32,
    lives: SizedString32,
    server_ip_address: SizedString32,
    placements: [SizedString32; 16],
}
impl ScoreboardScreenText {
    const FALLBACK: ScoreboardScreenText = Self {
        place: SizedString32::from_str("Place"),
        name: SizedString32::from_str("Name"),
        score: SizedString32::from_str("Score"),
        kills: SizedString32::from_str("Kills"),
        assists: SizedString32::from_str("Assists"),
        deaths: SizedString32::from_str("Deaths"),
        ping: SizedString32::from_str("Ping"),
        quit: SizedString32::from_str("Quit"),
        time: SizedString32::from_str("Time"),
        captures: SizedString32::from_str("Captures"),
        minutes: SizedString32::from_str("Minutes"),
        frags: SizedString32::from_str("Frags"),
        laps: SizedString32::from_str("Laps"),
        server_ip_address: SizedString32::from_str("Server IP Address - "),
        dead: SizedString32::from_str("Dead"),
        lives: SizedString32::from_str("Lives"),
        placements: [
            SizedString32::from_str("1st"),
            SizedString32::from_str("2nd"),
            SizedString32::from_str("3rd"),
            SizedString32::from_str("4th"),
            SizedString32::from_str("5th"),
            SizedString32::from_str("6th"),
            SizedString32::from_str("7th"),
            SizedString32::from_str("8th"),
            SizedString32::from_str("9th"),
            SizedString32::from_str("10th"),
            SizedString32::from_str("11th"),
            SizedString32::from_str("12th"),
            SizedString32::from_str("13th"),
            SizedString32::from_str("14th"),
            SizedString32::from_str("15th"),
            SizedString32::from_str("16th"),
        ]
    };

    pub unsafe fn load() -> Self {
        let mut current = Self::FALLBACK;
        let Some((_, multiplayer_game_text)) = lookup_tag("ui\\multiplayer_game_text", TagGroup::UnicodeStringList.into()) else {
            return current;
        };

        let copy_memes = |into: &mut SizedString32, index: u16| {
            let Some(t) = get_unicode_string_list_string(multiplayer_game_text, index) else {
                return
            };

            let mut writer = [0u8; 32];
            into.size = decode_utf16_inplace(t, &mut writer).len();
            into.bytes = writer;
        };

        copy_memes(&mut current.place, 67);
        copy_memes(&mut current.name, 68);
        copy_memes(&mut current.score, 154);
        copy_memes(&mut current.kills, 69);
        copy_memes(&mut current.assists, 70);
        copy_memes(&mut current.captures, 22);
        copy_memes(&mut current.minutes, 23);
        copy_memes(&mut current.frags, 24);
        copy_memes(&mut current.laps, 25);
        copy_memes(&mut current.deaths, 71);
        copy_memes(&mut current.time, 158);
        copy_memes(&mut current.quit, 139);
        copy_memes(&mut current.dead, 138);
        copy_memes(&mut current.server_ip_address, 190);
        copy_memes(&mut current.ping, 191); // only available in Custom Edition maps

        for i in 0..16 {
            copy_memes(&mut current.placements[i], 36 + i as u16);
        }

        current
    }
}

pub static mut SHOW_SERVER_INFO: u8 = 1;
pub static mut SCOREBOARD_STYLE: u16 = 0;
pub static mut USE_TERMINAL_FONT: u8 = 0;

#[derive(Copy, Clone, TryFromPrimitive, Default)]
#[repr(u16)]
pub enum ScoreboardStyle {
    #[default]
    Gearbox,
    // Xbox
    // Gearbox but it uses tag data
}

pub const STATUS_COLOR: ColorRGB = ColorRGB { r: 0.7, g: 0.7, b: 0.7 };
pub const HEADER_COLOR: ColorRGB = ColorRGB { r: 0.5, g: 0.5, b: 0.5 };
pub const HIGHLIGHT_BOOST: ColorRGB = ColorRGB { r: 0.4, g: 0.4, b: 0.4 };
pub const FFA_COLOR: ColorRGB = ColorRGB { r: 0.458824008703232, g: 0.729412019252777, b: 1.0 }; // TODO: read the icon color from hud globals instead
pub const RED_TEAM_COLOR: ColorRGB = ColorRGB { r: 0.6, g: 0.3, b: 0.3 };
pub const BLUE_TEAM_COLOR: ColorRGB = ColorRGB { r: 0.3, g: 0.3, b: 0.6 };
pub const GREEN_TEAM_COLOR: ColorRGB = ColorRGB { r: 0.3, g: 0.6, b: 0.3 };
pub const YELLOW_TEAM_COLOR: ColorRGB = ColorRGB { r: 0.6, g: 0.5, b: 0.3 };
pub static mut USE_PLAYER_COLORS: u8 = 0;

unsafe fn write_score_for_player<'a>(score: i32, score_buffer: &'a mut [u8], server_info: &ServerInfo) -> &'a str {
    let time = match server_info.get_gametype() {
        Gametype::King => true,
        // TODO: check if juggernaut
        Gametype::Oddball => true,
        _ => false
    };

    if time {
        // Sadly we can't show ms here because the server only syncs the time once per second.
        // Also it wouldn't fit the scoreboard.
        let seconds = score / 30;
        let minutes = seconds / 60;
        let seconds_trunc = seconds % 60;
        fmt_to_byte_array(score_buffer, format_args!("{minutes}:{seconds_trunc:02}")).unwrap()
    }
    else {
        fmt_to_byte_array(score_buffer, format_args!("{score}")).unwrap()
    }

}

// TODO: USE DEFINITIONS
unsafe fn get_font_tag_height(font: TagID) -> u16 {
    let data = get_tag_data_checking_tag_group(font, TagGroup::Font.into())
        .expect("no font tag ;-;")
        .wrapping_byte_add(4);
    *(data as *const u16) + *(data.wrapping_byte_add(2) as *const u16)
}

struct SortableScore {
    score: i32,
    kills: u16,
    deaths: u16,
    assists: u16,
    team: u16,
    player_id: PlayerID,
    placement: usize
}

unsafe fn sort_players_by_score(local_player: PlayerID, local_player_team: Option<u16>, players: &[PlayerID; MAXIMUM_NUMBER_OF_PLAYERS], server_info: &ServerInfo) -> [SortableScore; MAXIMUM_NUMBER_OF_PLAYERS] {
    let is_team_game = server_info.is_team_game();

    let mut scores: [SortableScore; MAXIMUM_NUMBER_OF_PLAYERS] = core::array::from_fn(|index| {
        let player = players[index];
        let Ok(player_data) = PLAYERS_TABLE
            .get_mut()
            .as_mut()
            .unwrap()
            .get_element(player)
            .map(|g| g.get()) else {
            return SortableScore {
                score: i32::MIN,
                player_id: player,
                kills: 0,
                deaths: 0,
                assists: 0,
                team: u16::MAX,
                placement: 0
            }
        };

        SortableScore {
            score: get_player_score(player, server_info),
            player_id: player,
            kills: player_data.kills,
            deaths: player_data.deaths,
            assists: player_data.assists,
            team: player_data.team,
            placement: 0
        }
    });

    // reversed sort order; higher scores go on the top
    scores.sort_by(|b, a| {
        if a.score != b.score {
            return a.score.cmp(&b.score)
        };
        if a.kills != b.kills {
            return a.kills.cmp(&b.kills)
        };
        if a.assists != b.assists {
            return a.assists.cmp(&b.assists)
        };
        if a.deaths != b.deaths {
            // dying is bad
            return a.deaths.cmp(&b.deaths).reverse()
        };
        if a.player_id != local_player {
            if a.player_id == local_player {
                return core::cmp::Ordering::Greater
            }
            else if b.player_id == local_player {
                return core::cmp::Ordering::Less
            }
            else {
                return b.player_id.index().cmp(&a.player_id.index())
            }
        }
        return core::cmp::Ordering::Equal
    });

    // Set placement by index
    for i in scores.iter_mut().enumerate() {
        i.1.placement = i.0
    }

    // If some players are tied, fixup the placement
    for i in 1..MAXIMUM_NUMBER_OF_PLAYERS {
        let (previous, this) = scores[i-1..=i].split_at_mut(1);
        let previous = &previous[0];
        let this = &mut this[0];

        if previous.assists == this.assists && previous.score == this.score && previous.kills == this.kills && previous.deaths == this.deaths {
            this.placement = previous.placement;
        }
    }

    // Now sort by teams
    if is_team_game {
        scores.sort_by(|b, a| {
            if a.team != b.team {
                return if local_player_team.is_some_and(|local_player_team| a.team != local_player_team && b.team == local_player_team) {
                    core::cmp::Ordering::Less
                }
                else if local_player_team.is_some_and(|local_player_team| b.team != local_player_team && a.team == local_player_team) {
                    core::cmp::Ordering::Greater
                }
                else {
                    // reverse; red team goes above blue team
                    a.team.cmp(&b.team).reverse()
                }
            }
            else {
                core::cmp::Ordering::Equal
            }
        });
    }

    scores
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
    let sorted = sort_players_by_score(local_player, local_player_team, &mut player_ids, server_info);

    let style = ScoreboardStyle::try_from(SCOREBOARD_STYLE).unwrap_or_default();
    SCOREBOARD_STYLE = style as u16;
    let scoreboard_text = ScoreboardScreenText::load();
    match style {
        ScoreboardStyle::Gearbox => draw_gearbox_scoreboard(local_player, opacity, &scoreboard_text, large_font, small_font, &sorted, server_info)
    }
}

unsafe fn get_scoreboard_color(player: PlayerID, server_info: &ServerInfo) -> ColorRGB {
    if server_info.is_team_game() {
        let player = PLAYERS_TABLE
            .get_mut()
            .as_mut()
            .unwrap()
            .get_element(player)
            .unwrap();

        match player.get().team {
            0 => RED_TEAM_COLOR,
            1 => BLUE_TEAM_COLOR,
            2 => GREEN_TEAM_COLOR,
            3 => YELLOW_TEAM_COLOR,
            _ => FFA_COLOR
        }
    }
    else if USE_PLAYER_COLORS == 0 {
        FFA_COLOR
    }
    else {
        let player = PLAYERS_TABLE
            .get_mut()
            .as_mut()
            .unwrap()
            .get_element(player)
            .unwrap();

        let mut color = *PLAYER_ICON_COLORS.get(player.get().color as usize).unwrap_or(&FFA_COLOR);
        color.r *= 0.7;
        color.g *= 0.7;
        color.b *= 0.7;
        color
    }
}

unsafe fn draw_gearbox_scoreboard(
    local_player: PlayerID,
    opacity: f32,
    scoreboard_text: &ScoreboardScreenText,
    large_ui: TagID,
    small_ui: TagID,
    all_players: &[SortableScore],
    server_info: &ServerInfo
) {
    let mut score_writer = DrawStringWriter::new_simple(
        small_ui,
        ColorARGB { alpha: opacity, color: STATUS_COLOR }
    );

    // the Gearbox scoreboard bypasses tag data and uses a hardcoded value
    let small_line_height = 15;
    let small_line_margin = 2;

    let top = 60u16;
    let left = 10u16;
    let right = 630u16;
    let bottom = 390u16;

    let mut score_offset = top - 1;
    let mut next_score_line = |line_height: u16| { score_offset += line_height; DrawStringBounds { top: score_offset - small_line_height, left: 8, right: 640 - 5, bottom: bottom.min(score_offset + small_line_margin) }};

    draw_box(
        DrawStringBounds {
            top,
            left,
            right,
            bottom
        },
        ColorARGB {
            alpha: opacity * 0.69,
            color: ColorRGB {
                r: 0.125,
                g: 0.125,
                b: 0.125,
            }
        }
    );

    score_writer.draw(
        format_args!(
            "(Butterfree says: TODO!)"
        ),
        next_score_line(small_line_height)
    ).expect(";-;");

    score_writer.set_tab_stops(&[
        25,
        90,

        // each column is 65 pixels
        300,
        365,
        430,
        495,
        560
    ]);

    score_writer.set_color(ColorARGB { alpha: opacity, color: HEADER_COLOR });

    let maximum_lives = *MAXIMUM_LIVES.get();

    score_writer.draw(
        format_args!(
            "\t{place}\t{name}\t{score}\t{kills}\t{assists}\t{deaths}\t{ping}",
            place = scoreboard_text.place,
            ping = scoreboard_text.ping,
            name = scoreboard_text.name,
            kills = scoreboard_text.kills,
            assists = scoreboard_text.assists,
            deaths = if maximum_lives > 0 { scoreboard_text.lives } else { scoreboard_text.deaths },
            score = match server_info.get_gametype() {
                Gametype::Race => &scoreboard_text.laps,
                Gametype::King => &scoreboard_text.time,
                // TODO: Change to "Score" if on juggernaut
                Gametype::Oddball => &scoreboard_text.time,
                _ => &scoreboard_text.score
            }
        ),
        next_score_line(small_line_height)
    ).expect(";-;");

    let players = PLAYERS_TABLE.get_mut().as_mut().expect("you can't draw the scoreboard without players!");
    for player_score_data in all_players.iter() {
        if player_score_data.player_id.is_null() {
            continue;
        }

        let player = players.get_element(player_score_data.player_id).expect("player went away???").get();

        let mut color = get_scoreboard_color(player_score_data.player_id, server_info);

        // highlight the local player
        if player_score_data.player_id == local_player {
            color.r = (color.r + HIGHLIGHT_BOOST.r).min(1.0);
            color.g = (color.g + HIGHLIGHT_BOOST.g).min(1.0);
            color.b = (color.b + HIGHLIGHT_BOOST.b).min(1.0);
        }

        score_writer.set_color(ColorARGB {
            alpha: opacity,
            color
        });

        let mut name_buffer = [0u8; 256];
        let name = decode_utf16_inplace(&player.name, &mut name_buffer);

        let mut score_buffer = [0u8; 32];
        let score: &str = write_score_for_player(player_score_data.score, &mut score_buffer, server_info);

        let mut deaths_buffer = [0u8; 32];
        let deaths: &str;
        if player.quit != 0 {
            deaths = scoreboard_text.quit.as_str();
        }
        else if maximum_lives == 0 {
            deaths = fmt_to_byte_array(&mut deaths_buffer, format_args!("{}", player.deaths)).expect(";-;")
        }
        else if player.out_of_lives() {
            deaths = scoreboard_text.dead.as_str();
        }
        else {
            deaths = fmt_to_byte_array(&mut deaths_buffer, format_args!("{}", maximum_lives - player.deaths as u32)).expect(";-;")
        };

        score_writer.draw(
            format_args!(
                "\t{place}\t{name}\t{score}\t{kills}\t{assists}\t{deaths}\t{ping}",
                place = scoreboard_text.placements[player_score_data.placement],
                ping = player.ping,
                kills = player_score_data.kills,
                assists = player_score_data.assists
            ),
            next_score_line(small_line_height)
        ).expect(";-;");
    }

    draw_server_info_gearbox(opacity, scoreboard_text, large_ui, server_info);
}

unsafe fn draw_server_info_gearbox(opacity: f32, scoreboard_text: &ScoreboardScreenText, large_font: TagID, server_info: &ServerInfo) {
    if SHOW_SERVER_INFO == 0 {
        return
    }

    let large_line_height = get_font_tag_height(large_font);
    let mut footer_writer = DrawStringWriter::new_simple(
        large_font,
        ColorARGB { alpha: opacity, color: DEFAULT_WHITE.color }
    );
    footer_writer.set_justification(DrawStringJustification::Right);
    let mut footer_offset = 480 - large_line_height * 2;
    let mut next_footer_line = |line_height: u16| { footer_offset += line_height; DrawStringBounds { top: footer_offset - large_line_height, left: 8, right: 640 - 5, bottom: 480.min(footer_offset) }};

    let mut server_name_buffer = [0u8; 66];
    let server_name = decode_utf16_inplace(&server_info.server_name, &mut server_name_buffer);

    let mut server_ip_buffer = [0u8; 66];
    let server_ip = format_connected_server_ip(&mut server_ip_buffer);

    footer_writer.draw(format_args!("{server_name}"), next_footer_line(large_line_height)).expect(";-;");
    footer_writer.draw(format_args!("{}{server_ip}", scoreboard_text.server_ip_address), next_footer_line(large_line_height)).expect(";-;");
}

unsafe fn format_connected_server_ip(buffer: &mut [u8]) -> &str {
    let (ip,port) = get_connected_ip_address();
    fmt_to_byte_array(
        buffer,
        format_args!(
            "{}.{}.{}.{}:{}",
            (ip >> 24) & 0xFF,
            (ip >> 16) & 0xFF,
            (ip >> 08) & 0xFF,
            (ip >> 00) & 0xFF,
            port
        )
    ).expect("but that should have worked!")
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
