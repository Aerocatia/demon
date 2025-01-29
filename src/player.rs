use core::ptr::null;
use c_mine::c_mine;
use crate::id::ID;
use crate::math::{Euler2D, Vector2D};
use crate::memory::game_state_malloc;
use crate::memory::table::{game_state_data_new, DataTable};
use crate::object::ObjectID;
use crate::util::VariableProvider;

pub const PLAYER_ID_SALT: u16 = 0x6C70;
pub type PlayerID = ID<PLAYER_ID_SALT>;

pub const MAXIMUM_NUMBER_OF_LOCAL_PLAYERS: usize = 1;
pub const MAXIMUM_NUMBER_OF_PLAYERS: usize = 16;

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

const PLAYERS_TABLE: VariableProvider<Option<&mut DataTable<[u8; 0x1F4], PLAYER_ID_SALT>>> = variable! {
    name: "PLAYERS_TABLE",
    cache_address: 0x00C59150,
    tag_address: 0x00D1070C
};

const PLAYER_GLOBALS: VariableProvider<Option<&mut PlayerGlobals>> = variable! {
    name: "player_globals",
    cache_address: 0x00C59158,
    tag_address: 0x00D10708
};

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
        b"players\x00".as_ptr() as *const _,
        MAXIMUM_NUMBER_OF_PLAYERS as u16,
        0x1F8
    ) as *mut _));
    *PLAYER_GLOBALS.get_mut() = Some(&mut *(game_state_malloc.get()(
        b"players globals\x00".as_ptr() as *const _,
        null(),
        0x98
    ) as *mut PlayerGlobals));

    let globals = PLAYER_GLOBALS.get_mut().as_mut().unwrap();
    globals.player_indices.fill(PlayerID::NULL);
    globals._unknown_0x0c = 0;
    globals._unknown_0x00 = 0xFFFFFFFF;

    *PLAYER_CONTROLS.get_mut() = Some(&mut *(game_state_malloc.get()(
        b"player control globals\x00".as_ptr() as *const _,
        null(),
        size_of::<PlayerControlTable>()
    ) as *mut _));
}
