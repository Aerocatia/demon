use crate::math::ColorRGB;
use crate::multiplayer::ServerInfo;
use crate::player::{PlayerID, PLAYERS_TABLE};
use crate::rasterizer::player_colors::PLAYER_ICON_COLORS;

pub static mut USE_PLAYER_COLORS: u8 = 0;

pub const HEADING_COLOR: ColorRGB = ColorRGB { r: 0.7, g: 0.7, b: 0.7 };
pub const HEADER_COLOR: ColorRGB = ColorRGB { r: 0.5, g: 0.5, b: 0.5 };
pub const HIGHLIGHT_BOOST: ColorRGB = ColorRGB { r: 0.4, g: 0.4, b: 0.4 };
pub const FFA_COLOR: ColorRGB = ColorRGB { r: 117.0 / 255.0, g: 186.0 / 255.0, b: 1.0 }; // TODO: read the icon color from hud globals instead
pub const RED_TEAM_COLOR: ColorRGB = ColorRGB { r: 0.6, g: 0.3, b: 0.3 };
pub const BLUE_TEAM_COLOR: ColorRGB = ColorRGB { r: 0.3, g: 0.3, b: 0.6 };
pub const GREEN_TEAM_COLOR: ColorRGB = ColorRGB { r: 0.3, g: 0.6, b: 0.3 };
pub const YELLOW_TEAM_COLOR: ColorRGB = ColorRGB { r: 0.6, g: 0.5, b: 0.3 };

pub unsafe fn get_scoreboard_color(player: PlayerID, server_info: &ServerInfo) -> ColorRGB {
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
