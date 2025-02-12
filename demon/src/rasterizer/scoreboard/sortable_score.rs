use crate::multiplayer::{get_player_score, ServerInfo};
use crate::player::{PlayerID, MAXIMUM_NUMBER_OF_PLAYERS, PLAYERS_TABLE};

pub struct SortableScore {
    pub score: i32,
    pub kills: u16,
    pub deaths: u16,
    pub assists: u16,
    pub team: u16,
    pub has_objective: bool,
    pub is_tied: bool,
    pub player_id: PlayerID,
    pub placement: usize
}

impl SortableScore {
    pub unsafe fn sort_players_by_score(local_player: PlayerID, local_player_team: Option<u16>, players: &[PlayerID; MAXIMUM_NUMBER_OF_PLAYERS], server_info: &ServerInfo) -> [SortableScore; MAXIMUM_NUMBER_OF_PLAYERS] {
        let is_team_game = server_info.is_team_game();

        let mut scores: [SortableScore; MAXIMUM_NUMBER_OF_PLAYERS] = core::array::from_fn(|index| {
            let player = players[index];
            let Ok(player_data) = PLAYERS_TABLE
                .get_copied()
                .unwrap()
                .get_element(player)
                .map(|g| g.get()) else {
                return SortableScore {
                    score: i32::MIN,
                    player_id: player,
                    kills: 0,
                    deaths: 0,
                    assists: 0,
                    has_objective: false,
                    team: u16::MAX,
                    is_tied: false,
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
                has_objective: false,
                is_tied: false,
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
            let previous = &mut previous[0];
            let this = &mut this[0];

            if previous.assists == this.assists && previous.score == this.score && previous.kills == this.kills && previous.deaths == this.deaths {
                this.placement = previous.placement;
                this.is_tied = true;
                previous.is_tied = true;
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
}
