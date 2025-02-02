use core::sync::atomic::{AtomicU16, Ordering};
use num_enum::TryFromPrimitive;
use c_mine::{c_mine, pointer_from_hook};
use crate::player::{PlayerID, MAXIMUM_NUMBER_OF_PLAYERS, PLAYERS_TABLE};
use crate::util::{PointerProvider, VariableProvider};

pub mod game_engine;
pub mod server;

pub unsafe fn get_network_client_memery() -> &'static NetworkClientMemery {
    match get_game_connection_state.get()() {
        GameConnectionState::ConnectedToServer => NETWORK_CLIENT_MEMERY_CLIENT.get(),
        _ => NETWORK_CLIENT_MEMERY_SERVER.get()
    }

}

const NETWORK_CLIENT_MEMERY_SERVER: VariableProvider<NetworkClientMemery> = variable! {
    name: "NETWORK_CLIENT_MEMERY_SERVER",
    cache_address: 0x00DE82A8,
    tag_address: 0x00E9F860
};

const NETWORK_CLIENT_MEMERY_CLIENT: VariableProvider<NetworkClientMemery> = variable! {
    name: "NETWORK_CLIENT_MEMERY_CLIENT",
    cache_address: 0x00DD18A0 + 0xB14,
    tag_address: 0x00E88E60 + 0xB14
};

#[repr(C)]
pub struct NetworkClientMemeryPlayer {
    pub name: [u16; 12],
    pub color: u16,
    pub _unknown_0x1a: u16,
    pub team: u16,
    pub _unknown_0x1e: u16
}
const _: () = assert!(size_of::<NetworkClientMemeryPlayer>() == 0x20);

pub const MEME_GAMETYPE_DATA: PointerProvider<unsafe extern "C" fn() -> *mut [u8; 0]> = pointer_from_hook!("get_meme_gametype_data");

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

#[derive(TryFromPrimitive, Copy, Clone, Debug)]
#[repr(u16)]
pub enum Gametype {
    CTF,
    Slayer,
    Oddball,
    King,
    Race
}

pub unsafe fn is_team_game() -> bool {
    let gametype = MEME_GAMETYPE_DATA.get()();
    if gametype.is_null() {
        return false
    }
    let is_team = gametype.wrapping_byte_add(0x138) as *const u8;
    *is_team != 0
}

pub unsafe fn get_gametype() -> Option<Gametype> {
    let gametype = MEME_GAMETYPE_DATA.get()();
    if gametype.is_null() {
        return None
    }
    let gametype = gametype.wrapping_byte_add(0x104);
    Gametype::try_from((*(gametype.wrapping_byte_add(0x30) as *const u16)).wrapping_sub(1)).ok()
}

pub unsafe fn get_player_score(player_id: PlayerID) -> i32 {
    let player_getter = || PLAYERS_TABLE
        .get_mut()
        .as_mut()
        .expect("NO PLAYER TABLE?!")
        .get_element(player_id)
        .expect("Failed to get player!")
        .get();

    let Some(gametype) = get_gametype() else {
        return i32::MIN
    };

    match gametype {
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
pub struct NetworkClientMemery {
    pub server_name: [u16; 66],
    pub map_name: [u8; 32],
    pub _unknown_0x104: [u8; 0x164 - 0x104],
    pub gametype_name: [u16; 12],
    pub _unknown_0x11c: [u8; 0x244 - 0x17C],
    pub players: [NetworkClientMemeryPlayer; 16]
}

const _: () = assert!(size_of::<NetworkClientMemery>() == 0x444 - 0x60);

pub unsafe fn play_multiplayer_sound(what: u32) {
    const GAME_ENGINE_PLAY_MULTIPLAYER_SOUND: PointerProvider<unsafe extern "C" fn(index: u32, something: bool)> = pointer_from_hook!("game_engine_play_multiplayer_sound");
    GAME_ENGINE_PLAY_MULTIPLAYER_SOUND.get()(what, false)
}

#[derive(Copy, Clone, TryFromPrimitive, Debug)]
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
