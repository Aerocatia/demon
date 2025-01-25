use c_mine::c_mine;
use crate::id::ID;
use crate::math::{Euler2D, Vector2D};
use crate::object::ObjectID;
use crate::util::VariableProvider;

pub type PlayerID = ID<0x6C70>;

pub const MAXIMUM_NUMBER_OF_LOCAL_PLAYERS: usize = 1;

pub const PLAYER_CONTROLS: VariableProvider<*mut PlayerControlTable> = variable! {
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
    let controls = &mut **PLAYER_CONTROLS.get();
    let Some(c) = controls.player_controls.get_mut(player_index as usize) else {
        panic!("get_player_control tried to get player index {player_index} when only {MAXIMUM_NUMBER_OF_LOCAL_PLAYERS} local players are supported")
    };
    c
}

#[c_mine]
pub unsafe extern "C" fn player_control_get(index: u16) -> &'static mut PlayerControl {
    get_player_control(index)
}
