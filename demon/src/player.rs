use core::ptr::null;
use c_mine::c_mine;
use tag_structs::primitives::vector::{Euler2D, Vector2D};
use crate::id::ID;
use crate::memory::game_state_malloc;
use crate::memory::table::{game_state_data_new, DataTable};
use crate::object::ObjectID;
use crate::util::{CStrPtr, VariableProvider};

pub const PLAYER_ID_SALT: u16 = 0x6C70;
pub type PlayerID = ID<PLAYER_ID_SALT>;

pub const MAXIMUM_NUMBER_OF_LOCAL_PLAYERS: usize = 1;
pub const MAXIMUM_NUMBER_OF_PLAYERS: usize = 16;

pub const MAXIMUM_LIVES: VariableProvider<u32> = variable! {
    name: "MAXIMUM_LIVES",
    cache_address: 0x00C56E28,
    tag_address: 0x00D0E3E0
};

#[repr(u16)]
pub enum PlayerControlsAction {
    Jump,
    SwitchGrenade,
    Action,
    SwitchWeapon,
    MeleeAttack,
    Flashlight,
    ThrowGrenade,
    FireWeapon,
    MenuAccept,
    MenuBack,
    Crouch,
    ScopeZoom,
    ShowScores,
    Reload,
    ExchangeWeapon,
    Say,
    SayToTeam,
    SayToVehicle,
    Screenshot,
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    LookUp,
    LookDown,
    LookLeft,
    LookRight,
    ShowRules,
    ShowPlayerNames,
}

pub const PLAYER_CONTROLS: VariableProvider<Option<&mut PlayerControlTable>> = variable! {
    name: "PLAYER_CONTROLS",
    cache_address: 0x00C59620,
    tag_address: 0x00D10BD8
};

#[repr(C)]
pub struct PlayerControlTable {
    pub unknown: [u8; 0x10],
    pub player_controls: [PlayerControl; MAXIMUM_NUMBER_OF_LOCAL_PLAYERS]
}

#[repr(C)]
pub struct PlayerControl {
    pub object_id: ObjectID,
    pub buttons: u32,
    pub unknown_0x8: u32,

    pub facing: Vector2D,
    pub movement: Vector2D,

    pub unknown_0x1c: u32,

    pub current_weapon_index: u16,
    pub current_grenade_index: u16,
    pub zoom_magnification: u16,
    pub unknown_0x26: u16,

    pub unknown_0x28: u32,
    pub unknown_0x2c: u32,

    pub unknown_0x30: u32,
    pub unknown_0x34: u32,

    pub desired_angles: Euler2D,
}

const _: () = assert!(size_of::<PlayerControl>() == 0x40);
const _: () = assert!(size_of::<PlayerControlTable>() == 0x10 + MAXIMUM_NUMBER_OF_LOCAL_PLAYERS * size_of::<PlayerControl>());

/// Unsafe because this is a static mutable reference, and no guarantees this is the only one!
///
/// # Panics
///
/// Panics if `index` >= [`MAXIMUM_NUMBER_OF_LOCAL_PLAYERS`]
pub unsafe fn get_player_control(player_index: u16) -> &'static mut PlayerControl {
    let controls = PLAYER_CONTROLS.get_mut().as_mut().expect("get_player_control with null PLAYER_CONTROLS");
    let Some(c) = controls.player_controls.get_mut(player_index as usize) else {
        panic!("get_player_control tried to get player index {player_index} when only {MAXIMUM_NUMBER_OF_LOCAL_PLAYERS} local players are supported")
    };
    c
}

pub unsafe fn get_player_id(player_index: u16) -> PlayerID {
    let Some(c) = PLAYER_GLOBALS
        .get()
        .as_ref()
        .expect("get_player_id called when player_globals is null")
        .player_indices
        .get(player_index as usize)
    else {
        panic!("get_player_id tried to get player index {player_index} when only {MAXIMUM_NUMBER_OF_LOCAL_PLAYERS} local players are supported")
    };
    *c
}

#[repr(C)]
struct PlayerGlobals {
    _unknown_0x00: u32,
    player_indices: [PlayerID; MAXIMUM_NUMBER_OF_LOCAL_PLAYERS],
    _unknown_0x08: u32,
    _unknown_0x0c: u16,
    _unknown_0x0e: u16,
    _unknown_0x10: [u8; 0x88]
}
const _: () = assert!(size_of::<PlayerGlobals>() == 0x98);

// everything is what it is + 0x4 because of the ID
#[repr(C)]
pub struct Player {
    pub salt: u16,
    pub local_player_index: u16,

    /// Name (11 characters + null terminator)
    pub name: [u16; 12],

    /// Team (0 = red, 1 = blue...)
    pub team: u16,

    /// ????
    pub _unknown_0x1e: u16,

    /// ????
    pub _unknown_0x20: [u8; 0x30 - 0x20],

    /// Unit of the player
    pub unit: ObjectID,

    /// ????
    pub _unknown_0x34: [u8; 0x44 - 0x34],

    /// Another copy of the name (11 characters + null terminator)
    pub name_again: [u16; 12],

    /// Armor color (FFA)
    pub color: u16,

    /// 0xFFFF
    pub _unknown_0x5e: u16,

    /// ????
    pub _unknown_0x60: [u8; 0x98 - 0x60],

    /// Number of kills the player has
    pub kills: u16,

    /// ????
    pub _unknown_0x9a: u16,

    /// ????
    pub _unknown_0x9c: u32,

    /// Number of assists the player has
    pub assists: u16,

    /// ????
    pub _unknown_0xa2: u16,

    /// ????
    pub _unknown_0xa4: [u8; 0xAA - 0xA4],

    /// Number of deaths the player has
    pub deaths: u16,

    /// ????
    pub _unknown_0xac: [u8; 0xC0 - 0xAC],

    /// In King, this is time on the hill in ticks.
    ///
    /// In CTF, the upper 16 bits are number of times it's been returned, and the lower 16 bits are the number of times it's been picked up.
    ///
    /// In Race, the upper 16 bits are laps and the lower 16 bits are fuck you.
    pub score_data: i32,

    /// In CTF, this is the number of times the flag has been captured.
    pub score_data_2: i32,

    /// ????
    pub _unknown_0xc8: [u8; 0xD0 - 0xC8],

    /// ????
    pub _unknown_0xd0: u8,

    /// If non-zero, the player has quit
    pub quit: u8,

    /// ????
    pub _unknown_0xd2: u16,

    /// Ping in milliseconds
    pub ping: u16,

    /// ????
    pub _unknown_0xd6: [u8; 0x1F8 - 0xD6]
}

impl Player {
    pub unsafe fn out_of_lives(&self) -> bool {
        let lives = *MAXIMUM_LIVES.get();
        self.unit.is_null() && lives > 0 && (self.deaths as u32) >= lives
    }
}

const _: () = assert!(size_of::<Player>() == 0x1F8);

pub const PLAYERS_TABLE: VariableProvider<Option<&mut DataTable<Player, PLAYER_ID_SALT>>> = variable! {
    name: "PLAYERS_TABLE",
    cache_address: 0x00C59150,
    tag_address: 0x00D1070C
};

const PLAYER_GLOBALS: VariableProvider<Option<&mut PlayerGlobals>> = variable! {
    name: "player_globals",
    cache_address: 0x00C59158,
    tag_address: 0x00D10708
};

const LOCAL_PLAYER_INDEX: VariableProvider<u16> = variable! {
    name: "LOCAL_PLAYER_INDEX",
    cache_address: 0x00ECFE00,
    tag_address: 0x00F873C0
};

pub unsafe fn get_local_player_index() -> u16 {
    *LOCAL_PLAYER_INDEX.get()
}

#[c_mine]
pub unsafe extern "C" fn player_control_get(index: u16) -> &'static mut PlayerControl {
    get_player_control(index)
}

#[c_mine]
pub unsafe extern "C" fn local_player_get_player_index(player_index: u16) -> PlayerID {
    if player_index == 0xFFFF {
        return PlayerID::NULL
    }
    get_player_id(player_index)
}

#[c_mine]
pub unsafe extern "C" fn players_initialize() {
    *PLAYERS_TABLE.get_mut() = Some(&mut *(game_state_data_new.get()(
        CStrPtr::from_bytes(b"players\x00"),
        MAXIMUM_NUMBER_OF_PLAYERS as u16,
        0x1F8
    ) as *mut _));
    *PLAYER_GLOBALS.get_mut() = Some(&mut *(game_state_malloc.get()(
        CStrPtr::from_bytes(b"players globals\x00"),
        null(),
        0x98
    ) as *mut PlayerGlobals));

    let globals = PLAYER_GLOBALS.get_mut().as_mut().unwrap();
    globals.player_indices.fill(PlayerID::NULL);
    globals._unknown_0x0c = 0;
    globals._unknown_0x00 = 0xFFFFFFFF;

    *PLAYER_CONTROLS.get_mut() = Some(&mut *(game_state_malloc.get()(
        CStrPtr::from_bytes(b"player control globals\x00"),
        null(),
        size_of::<PlayerControlTable>()
    ) as *mut _));
}
