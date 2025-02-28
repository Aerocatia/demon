use tag_structs::primitives::tag_group::TagGroup;
use crate::string::get_unicode_string_list_string;
use crate::tag::lookup_tag;
use crate::util::StaticStringBytes;

pub type SizedString32 = StaticStringBytes<32>;

pub struct ScoreboardScreenText {
    pub place: SizedString32,
    pub name: SizedString32,
    pub score: SizedString32,
    pub captures: SizedString32,
    pub minutes: SizedString32,
    pub frags: SizedString32,
    pub time: SizedString32,
    pub laps: SizedString32,
    pub kills: SizedString32,
    pub assists: SizedString32,
    pub deaths: SizedString32,
    pub ping: SizedString32,
    pub quit: SizedString32,
    pub dead: SizedString32,
    pub lives: SizedString32,
    pub you_won: SizedString32,
    pub your_team_won: SizedString32,
    pub you_lost: SizedString32,
    pub your_team_lost: SizedString32,
    pub in_nth_place_with_n: SizedString32,
    pub tied_for_nth_place_with_n: SizedString32,
    pub n_lives: SizedString32,
    pub red_leads: SizedString32,
    pub blue_leads: SizedString32,
    pub teams_tied: SizedString32,
    pub game_ends_in_a_draw: SizedString32,
    pub one_life: SizedString32,
    pub no_lives: SizedString32,
    pub server_ip_address: SizedString32,
    pub placements: [SizedString32; 16],
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
        in_nth_place_with_n: SizedString32::from_str("In %s place with %s %s"),
        tied_for_nth_place_with_n: SizedString32::from_str("Tied for %s place with %s %s"),
        n_lives: SizedString32::from_str("(%s lives)"),
        one_life: SizedString32::from_str("(1 life)"),
        no_lives: SizedString32::from_str("(no lives)"),
        red_leads: SizedString32::from_str("Red leads Blue %s to %s %s"),
        blue_leads: SizedString32::from_str("Blue leads Red %s to %s %s"),
        teams_tied: SizedString32::from_str("Teams tied at %s to %s"),
        game_ends_in_a_draw: SizedString32::from_str("Game ends in a draw"),
        you_won: SizedString32::from_str("You won"),
        your_team_won: SizedString32::from_str("Your team won"),
        you_lost: SizedString32::from_str("You lost"),
        your_team_lost: SizedString32::from_str("Your team lost"),
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
            *into = SizedString32::from_utf16(t)
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
        copy_memes(&mut current.red_leads, 60);
        copy_memes(&mut current.blue_leads, 61);
        copy_memes(&mut current.teams_tied, 62);
        copy_memes(&mut current.in_nth_place_with_n, 64);
        copy_memes(&mut current.tied_for_nth_place_with_n, 63);
        copy_memes(&mut current.no_lives, 52);
        copy_memes(&mut current.one_life, 53);
        copy_memes(&mut current.n_lives, 54);
        copy_memes(&mut current.game_ends_in_a_draw, 55);
        copy_memes(&mut current.your_team_lost, 56);
        copy_memes(&mut current.you_lost, 57);
        copy_memes(&mut current.your_team_won, 58);
        copy_memes(&mut current.you_won, 59);
        copy_memes(&mut current.server_ip_address, 190);
        copy_memes(&mut current.ping, 191); // only available in Custom Edition maps

        for i in 0..16 {
            copy_memes(&mut current.placements[i], 36 + i as u16);
        }

        current
    }
}
