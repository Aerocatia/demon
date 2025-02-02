use crate::multiplayer::game_engine::{get_game_engine_globals_mode, GameEngineGlobalsMode};
use crate::multiplayer::ServerInfo;
use crate::player::{MAXIMUM_LIVES, PLAYERS_TABLE};
use crate::rasterizer::scoreboard::format_score;
use crate::rasterizer::scoreboard::sortable_score::SortableScore;
use crate::rasterizer::scoreboard::strings::ScoreboardScreenText;
use crate::util::{fmt_to_byte_array, PrintfFormatter};

pub unsafe fn fmt_scoreboard_heading<'a>(
    score_header_buffer: &'a mut [u8],
    scoreboard_text: &'a ScoreboardScreenText,
    local_player_score_data: Option<&SortableScore>,
    server_info: &ServerInfo
) -> &'a str {
    if let Some(local_score) = local_player_score_data {
        if unsafe { get_game_engine_globals_mode() } == GameEngineGlobalsMode::Active {
            fmt_scoreboard_heading_game_in_progress(
                score_header_buffer,
                scoreboard_text,
                local_score,
                server_info
            )
        }
        else {
            if server_info.show_red_blue_team_names() {
                let your_team_score = server_info.get_team_score(local_score.team);
                let enemy_team_score = server_info.get_team_score((local_score.team + 1) & 1);
                match your_team_score.cmp(&enemy_team_score) {
                    core::cmp::Ordering::Equal => scoreboard_text.game_ends_in_a_draw.as_str(),
                    core::cmp::Ordering::Greater => scoreboard_text.your_team_won.as_str(),
                    core::cmp::Ordering::Less => scoreboard_text.your_team_lost.as_str(),
                }
            }
            else if !server_info.is_team_game() {
                if local_score.placement > 0 {
                    scoreboard_text.you_lost.as_str()
                }
                else if local_score.is_tied {
                    scoreboard_text.game_ends_in_a_draw.as_str()
                }
                else {
                    scoreboard_text.you_won.as_str()
                }
            }
            else {
                // multi-team support not implemented
                ""
            }
        }
    }
    else {
        ""
    }
}

unsafe fn fmt_scoreboard_heading_game_in_progress<'a>(
    buffer: &'a mut [u8],
    scoreboard_text: &ScoreboardScreenText,
    score_data: &SortableScore,
    server_info: &ServerInfo
) -> &'a str {
    let mut lives_buffer = [0u8; 32];
    let lives = match *MAXIMUM_LIVES.get() {
        0 => "",
        lives => {
            let remaining_lives = lives.saturating_sub(score_data.deaths as u32);
            match remaining_lives {
                0 => scoreboard_text.no_lives.as_str(),
                1 => scoreboard_text.one_life.as_str(),
                n => {
                    let formatter = PrintfFormatter {
                        printf_string: scoreboard_text.n_lives.as_str(),
                        items: &[&n]
                    };
                    fmt_to_byte_array(&mut lives_buffer, format_args!("{formatter}")).expect(";-;")
                }
            }
        }
    };

    if server_info.show_red_blue_team_names() {
        let mut red_team_score_buffer = [0u8; 32];
        let mut blue_team_score_buffer = [0u8; 32];

        let red_team_score = server_info.get_team_score(0);
        let blue_team_score = server_info.get_team_score(1);

        let red_team_score_str = format_score(red_team_score, &mut red_team_score_buffer, server_info);
        let blue_team_score_str = format_score(blue_team_score, &mut blue_team_score_buffer, server_info);

        // Tied
        if red_team_score == blue_team_score {
            let formatter = PrintfFormatter {
                printf_string: scoreboard_text.teams_tied.as_str(),
                items: &[
                    &red_team_score_str,
                    &lives
                ]
            };
            return fmt_to_byte_array(buffer, format_args!("{formatter}")).expect(";-;");
        }

        let printf_string;
        let winning_score;
        let losing_score;

        if red_team_score > blue_team_score {
            printf_string = scoreboard_text.red_leads.as_str();
            winning_score = red_team_score_str;
            losing_score = blue_team_score_str;
        }
        else {
            printf_string = scoreboard_text.blue_leads.as_str();
            winning_score = blue_team_score_str;
            losing_score = red_team_score_str;
        };

        let formatter = PrintfFormatter {
            printf_string,
            items: &[
                &winning_score,
                &losing_score,
                &lives
            ]
        };

        fmt_to_byte_array(buffer, format_args!("{formatter}")).expect(";-;")
    }
    else if server_info.is_team_game() {
        // TODO: multiple teams
        write_nth_place(
            buffer,
            server_info.get_team_score(PLAYERS_TABLE.get_mut().as_mut().unwrap().get_element(score_data.player_id).unwrap().get().team),
            false,
            usize::MAX,
            lives,
            server_info,
            scoreboard_text
        )
    }
    else {
        write_nth_place(
            buffer,
            score_data.score,
            score_data.is_tied,
            score_data.placement,
            lives,
            server_info,
            scoreboard_text
        )
    }
}

unsafe fn write_nth_place<'a>(
    buffer: &'a mut [u8],
    score: i32,
    is_tied: bool,
    placement: usize,
    lives: &str,
    server_info: &ServerInfo,
    scoreboard_text: &ScoreboardScreenText,
) -> &'a str {
    let placement = scoreboard_text
        .placements
        .get(placement)
        .map(|s| s.as_str())
        .unwrap_or("?th");

    let mut score_buffer = [0u8; 32];
    let score = format_score(score, &mut score_buffer, server_info);

    let scoreboard = PrintfFormatter {
        printf_string: if is_tied {
            scoreboard_text.tied_for_nth_place_with_n.as_str()
        }
        else {
            scoreboard_text.in_nth_place_with_n.as_str()
        },
        items: &[
            &placement,
            &score,
            &lives
        ]
    };
    fmt_to_byte_array(buffer, format_args!("{scoreboard}")).expect(";-;")
}
