use tag_structs::primitives::color::{ColorARGB, ColorRGB};
use crate::memory::table::DataTable;
use crate::multiplayer::{get_connected_ip_address, Gametype, ServerInfo};
use crate::player::{Player, PlayerID, MAXIMUM_LIVES, PLAYERS_TABLE};
use crate::rasterizer::{draw_box, InterfaceCanvasBounds};
use crate::rasterizer::draw_string::{DrawStringJustification, DrawStringWriter, DEFAULT_WHITE};
use crate::rasterizer::font::get_font_tag_height;
use crate::rasterizer::scoreboard::color::{get_scoreboard_color, HEADER_COLOR, HEADING_COLOR, HIGHLIGHT_BOOST};
use crate::rasterizer::scoreboard::{format_score, SHOW_SERVER_INFO};
use crate::rasterizer::scoreboard::heading::fmt_scoreboard_heading;
use crate::rasterizer::scoreboard::sortable_score::SortableScore;
use crate::rasterizer::scoreboard::strings::ScoreboardScreenText;
use crate::tag::TagID;
use crate::util::StaticStringBytes;

pub unsafe fn draw_verbose_scoreboard(
    local_player: PlayerID,
    opacity: f32,
    scoreboard_text: &ScoreboardScreenText,
    large_ui: TagID,
    small_ui: TagID,
    ffa_color: &ColorARGB,
    all_players: &[SortableScore],
    server_info: &ServerInfo
) {
    let mut score_writer = DrawStringWriter::new_simple(
        small_ui,
        ColorARGB { a: opacity, color: HEADING_COLOR }
    );

    // originally hardcoded to 15, with an extra 2 pixels of leeway when doing the actual text draw
    let small_line_height = get_font_tag_height(small_ui).0;
    let large_line_height = get_font_tag_height(large_ui).0;

    let top = 60u16;
    let left = 10u16;
    let right = 630u16;
    // originally 390
    let bottom = (top + small_line_height * (16 + 2) + 2)
        // prevent the server name and IP from being overlapped
        .min(480 - large_line_height * 2);

    // originally top - 1
    let mut score_offset = top;
    let mut next_score_line = |line_height: u16| { score_offset += line_height; InterfaceCanvasBounds { top: score_offset - small_line_height, left: 8, right: 640 - 5, bottom: bottom.min(score_offset) }};

    draw_box(
        InterfaceCanvasBounds {
            top,
            left,
            right,
            bottom
        },
        ColorARGB {
            a: opacity * 0.69,
            color: ColorRGB {
                r: 0.125,
                g: 0.125,
                b: 0.125,
            }
        }
    );

    let maximum_lives = *MAXIMUM_LIVES.get();
    let local_player_score_data = all_players
        .iter()
        .find(|s| s.player_id == local_player);

    let score_heading = fmt_scoreboard_heading(
        scoreboard_text,
        local_player_score_data,
        server_info
    );

    score_writer
        .draw(format_args!("{score_heading}"), next_score_line(small_line_height))
        .expect(";-;");

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

    score_writer.set_color(ColorARGB { a: opacity, color: HEADER_COLOR });

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
                Gametype::Oddball if server_info.scoring_uses_time() => &scoreboard_text.time,
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

        let bounds = next_score_line(small_line_height);
        draw_player_score(
            local_player,
            opacity,
            scoreboard_text,
            server_info,
            &mut score_writer,
            small_line_height,
            bounds,
            maximum_lives,
            players,
            player_score_data,
            ffa_color
        );
    }

    draw_server_info(opacity, scoreboard_text, large_ui, server_info);
}

unsafe fn draw_player_score(
    local_player: PlayerID,
    opacity: f32,
    scoreboard_text:
    &ScoreboardScreenText,
    server_info: &ServerInfo,
    score_writer: &mut DrawStringWriter,
    small_line_height: u16,
    bounds: InterfaceCanvasBounds,
    maximum_lives: u32,
    players: &mut &mut DataTable<Player, 27760>,
    player_score_data: &SortableScore,
    ffa_color: &ColorARGB
) {
    let player = players.get_element(player_score_data.player_id).expect("player went away???").get();

    let mut color = get_scoreboard_color(player_score_data.player_id, server_info).unwrap_or(ffa_color.color);

    // highlight the local player
    if player_score_data.player_id == local_player {
        color.r = (color.r + HIGHLIGHT_BOOST.r).min(1.0);
        color.g = (color.g + HIGHLIGHT_BOOST.g).min(1.0);
        color.b = (color.b + HIGHLIGHT_BOOST.b).min(1.0);
    }

    score_writer.set_color(ColorARGB {
        a: opacity,
        color
    });

    let name = StaticStringBytes::<256>::from_utf16(&player.name);
    let score = format_score(player_score_data.score, server_info);
    let deaths: StaticStringBytes<16> = if player.quit != 0 {
        StaticStringBytes::from_display(scoreboard_text.quit.as_str())
    } else if maximum_lives == 0 {
        StaticStringBytes::from_display(player.deaths)
    } else if player.out_of_lives() {
        StaticStringBytes::from_display(scoreboard_text.dead.as_str())
    } else {
        StaticStringBytes::from_display(maximum_lives - player.deaths as u32)
    };

    score_writer.draw(
        format_args!(
            "{has_objective}\t{place}\t{name}\t{score}\t{kills}\t{assists}\t{deaths}\t{ping}",
            has_objective = if player_score_data.has_objective { "*" } else { "" },
            place = scoreboard_text.placements[player_score_data.placement],
            ping = player.ping,
            kills = player_score_data.kills,
            assists = player_score_data.assists
        ),
        bounds
    ).expect(";-;");
}

unsafe fn draw_server_info(opacity: f32, scoreboard_text: &ScoreboardScreenText, large_font: TagID, server_info: &ServerInfo) {
    if SHOW_SERVER_INFO == 0 {
        return
    }

    let large_line_height = get_font_tag_height(large_font).0;
    let mut footer_writer = DrawStringWriter::new_simple(
        large_font,
        ColorARGB { a: opacity, color: DEFAULT_WHITE.color }
    );
    footer_writer.set_justification(DrawStringJustification::Right);
    let mut footer_offset = 480 - large_line_height * 2;
    let mut next_footer_line = |line_height: u16| { footer_offset += line_height; InterfaceCanvasBounds { top: footer_offset - large_line_height, left: 8, right: 640 - 5, bottom: 480.min(footer_offset) }};

    let server_name = StaticStringBytes::<66>::from_utf16(&server_info.server_name);
    let server_ip = format_connected_server_ip();

    footer_writer.draw(format_args!("{server_name}"), next_footer_line(large_line_height)).expect(";-;");
    footer_writer.draw(format_args!("{}{server_ip}", scoreboard_text.server_ip_address), next_footer_line(large_line_height)).expect(";-;");
}

unsafe fn format_connected_server_ip() -> StaticStringBytes<66> {
    let (ip,port) = get_connected_ip_address();
    StaticStringBytes::from_fmt(
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
