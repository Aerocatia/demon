use core::mem::zeroed;
use core::ptr::null;
use c_mine::c_mine;
use crate::memory::game_state_malloc;
use crate::memory::table::game_state_data_new;
use crate::player::{get_player_control, local_player_index_to_id, PlayerControl, PlayerControlTable, PlayerGlobals, PlayerID, MAXIMUM_NUMBER_OF_PLAYERS, PLAYERS_TABLE, PLAYER_CONTROLS, PLAYER_GLOBALS};
use crate::player::player_effect::{PlayerEffect, PLAYER_EFFECT_GLOBALS};
use crate::util::CStrPtr;

#[c_mine]
pub unsafe extern "C" fn player_control_get(index: u16) -> &'static mut PlayerControl {
    get_player_control(index)
}

#[c_mine]
pub unsafe extern "C" fn local_player_get_player_index(player_index: u16) -> PlayerID {
    local_player_index_to_id(player_index)
}

#[c_mine]
pub unsafe extern "C" fn players_initialize() {
    *PLAYERS_TABLE.get_mut() = Some(&mut *(game_state_data_new.get()(
        CStrPtr::from_cstr(c"players"),
        MAXIMUM_NUMBER_OF_PLAYERS as u16,
        0x1F8
    ) as *mut _));
    *PLAYER_GLOBALS.get_mut() = Some(&mut *(game_state_malloc.get()(
        CStrPtr::from_cstr(c"players globals"),
        null(),
        0x98
    ) as *mut PlayerGlobals));

    let globals = PLAYER_GLOBALS.get_copied().unwrap();
    globals.player_indices.fill(PlayerID::NULL);
    globals._unknown_0x0c = 0;
    globals._unknown_0x00 = 0xFFFFFFFF;

    *PLAYER_CONTROLS.get_mut() = Some(&mut *(game_state_malloc.get()(
        CStrPtr::from_cstr(c"player control globals"),
        null(),
        size_of::<PlayerControlTable>()
    ) as *mut _));
}

#[c_mine]
pub unsafe extern "C" fn player_effect_get(local_player: u16) -> &'static mut PlayerEffect {
    PLAYER_EFFECT_GLOBALS
        .get_copied()
        .get_mut(local_player as usize)
        .expect("player_effect_get with invalid local player index")
}

#[c_mine]
pub unsafe extern "C" fn player_effect_clear_damage_indicators(local_player: u16) {
    PLAYER_EFFECT_GLOBALS
        .get_copied()
        .get_mut(local_player as usize)
        .expect("player_effect_clear_damage_indicators with invalid local player index")
        .damage_indicator_data = zeroed();
}
