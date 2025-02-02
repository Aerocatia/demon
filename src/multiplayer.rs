use core::ffi::CStr;
use core::sync::atomic::{AtomicU16, Ordering};
use num_enum::TryFromPrimitive;
use c_mine::{c_mine, pointer_from_hook};
use crate::player::{PlayerID, MAXIMUM_NUMBER_OF_PLAYERS, PLAYERS_TABLE};
use crate::util::{PointerProvider, VariableProvider};

pub mod game_engine;
pub mod server;

pub unsafe fn get_server_info() -> Option<&'static ServerInfo> {
    if get_game_connection_state.get()() == GameConnectionState::None {
        return None
    }
    Some(SERVER_INFO.get()())
}

#[repr(C)]
pub struct NetworkClientMemeryPlayer {
    pub name: [u16; 12],
    pub color: u16,
    pub _unknown_0x1a: u16,
    pub team: u16,
    pub _unknown_0x1e: u16
}
const _: () = assert!(size_of::<NetworkClientMemeryPlayer>() == 0x20);

pub const SERVER_INFO: PointerProvider<unsafe extern "C" fn() -> &'static ServerInfo> = pointer_from_hook!("get_meme_gametype_data");

pub const HOSTED_SERVER_IP_ADDRESS: VariableProvider<u32> = variable! {
    name: "HOSTED_SERVER_IP_ADDRESS",
    cache_address: 0x00A1B958,
    tag_address: 0x00AADF30
};

pub const HOSTED_SERVER_PORT: VariableProvider<u16> = variable! {
    name: "HOSTED_SERVER_PORT",
    cache_address: 0x00A33648,
    tag_address: 0x00AD5560
};

pub const CONNECTED_SERVER_IP_ADDRESS: VariableProvider<u32> = variable! {
    name: "CONNECTED_SERVER_IP_ADDRESS",
    cache_address: 0x00DD2354,
    tag_address: 0x00E89914
};

pub const CONNECTED_SERVER_PORT: VariableProvider<u16> = variable! {
    name: "CONNECTED_PORT_ADDRESS",
    cache_address: 0x00DD2366,
    tag_address: 0x00E89926
};

pub const ODDBALL_SCORES: VariableProvider<[i32; MAXIMUM_NUMBER_OF_PLAYERS]> = variable! {
    name: "ODDBALL_SCORES",
    cache_address: 0x00C5889C,
    tag_address: 0x00D0FE54
};

pub const SLAYER_SCORES: VariableProvider<[i32; MAXIMUM_NUMBER_OF_PLAYERS]> = variable! {
    name: "SLAYER_SCORES",
    cache_address: 0x00C58FF0,
    tag_address: 0x00D105A8
};

pub const SERVER_MOTD: VariableProvider<[u8; 0x100]> = variable! {
    name: "SERVER_MOTD",
    cache_address: 0x00DE91B0,
    tag_address: 0x00EA0870
};

pub unsafe fn get_server_motd() -> &'static str {
    CStr::from_bytes_until_nul(SERVER_MOTD.get())
        .ok()
        .and_then(|m| m.to_str().ok())
        .unwrap_or("")
}

#[derive(TryFromPrimitive, Copy, Clone, Debug)]
#[repr(u16)]
pub enum Gametype {
    CTF,
    Slayer,
    Oddball,
    King,
    Race
}

pub unsafe fn get_player_score(player_id: PlayerID, server_info: &ServerInfo) -> i32 {
    let player_getter = || PLAYERS_TABLE
        .get_mut()
        .as_mut()
        .expect("NO PLAYER TABLE?!")
        .get_element(player_id)
        .expect("Failed to get player!")
        .get();

    match server_info.get_gametype() {
        Gametype::Slayer => SLAYER_SCORES
            .get()
            .get(player_id.index().unwrap())
            .map(|i| *i)
            .unwrap_or(i32::MIN),
        Gametype::King => player_getter().score_data,
        Gametype::Race => player_getter().score_data >> 16,
        Gametype::CTF => player_getter().score_data_2,
        Gametype::Oddball => ODDBALL_SCORES
            .get()
            .get(player_id.index().unwrap())
            .map(|i| *i)
            .unwrap_or(i32::MIN)
    }
}

pub unsafe fn get_connected_ip_address() -> (u32, u16) {
    match get_game_connection_state.get()() {
        GameConnectionState::ConnectedToServer => (*CONNECTED_SERVER_IP_ADDRESS.get(), *CONNECTED_SERVER_PORT.get()),
        _ => (*HOSTED_SERVER_IP_ADDRESS.get(), *HOSTED_SERVER_PORT.get())
    }
}

#[repr(C)]
pub struct ServerInfo {
    pub server_name: [u16; 66],
    pub map_name: [u8; 32],
    pub _unknown_0x104: [u8; 0x164 - 0x104],
    pub gametype_name: [u16; 12],
    pub _unknown_0x17c: [u8; 0x194 - 0x17C],
    pub gametype: u16,
    pub _unknown_0x196: u16,
    pub is_team_game: u8,
    pub _unknown_0x199: [u8; 0x244 - 0x199],
    pub players: [NetworkClientMemeryPlayer; 16]
}
impl ServerInfo {
    pub fn is_team_game(&self) -> bool {
        self.is_team_game != 0
    }
    pub fn get_gametype(&self) -> Gametype {
        let gametype_index = self.gametype.wrapping_sub(1);
        let Ok(gametype) = Gametype::try_from(gametype_index) else {
            panic!("Invalid gametype index {gametype_index}")
        };
        gametype
    }
    pub fn scoring_uses_time(&self) -> bool {
        match self.get_gametype() {
            Gametype::King => true,
            // FIXME: replace _unknown_0x199, etc. with actual gametype structs
            // Juggernaut uses score. Reverse Tag and Normal Oddball use time.
            Gametype::Oddball => self._unknown_0x199[119] != 2,
            _ => false
        }
    }
}

const _: () = assert!(size_of::<ServerInfo>() == 0x3E4);

pub unsafe fn play_multiplayer_sound(what: u32) {
    const GAME_ENGINE_PLAY_MULTIPLAYER_SOUND: PointerProvider<unsafe extern "C" fn(index: u32, something: bool)> = pointer_from_hook!("game_engine_play_multiplayer_sound");
    GAME_ENGINE_PLAY_MULTIPLAYER_SOUND.get()(what, false)
}

#[derive(Copy, Clone, TryFromPrimitive, Debug, PartialEq)]
#[repr(u16)]
pub enum GameConnectionState {
    None,
    ConnectedToServer,
    Hosting
}

pub static GAME_CONNECTION_STATE: AtomicU16 = AtomicU16::new(GameConnectionState::None as u16);

#[c_mine]
pub extern "C" fn set_game_connection_state(state: u16) {
    let state = GameConnectionState::try_from(state).expect("set_game_connection_state with invalid state");
    GAME_CONNECTION_STATE.store(state as u16, Ordering::Relaxed);
}

#[c_mine]
pub extern "C" fn get_game_connection_state() -> GameConnectionState {
    GAME_CONNECTION_STATE.load(Ordering::Relaxed).try_into().expect("get_game_connection_state with invalid state")
}

#[c_mine]
pub unsafe extern "C" fn death() -> *const u8 {
    "putput\x00".as_ptr()
}
