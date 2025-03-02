use crate::player::MAXIMUM_NUMBER_OF_LOCAL_PLAYERS;
use crate::util::VariableProvider;

#[repr(C)]
pub struct PlayerEffect {
    pub _unknown_0x00: [u8; 0xE4],
    pub damage_indicator_data: u32,
    pub _unknown_0xe8: [u8; 4]
}

pub const PLAYER_EFFECT_GLOBALS: VariableProvider<&mut [PlayerEffect; MAXIMUM_NUMBER_OF_LOCAL_PLAYERS]> = variable! {
    name: "player_effect_globals",
    cache_address: 0x00C550D8,
    tag_address: 0x00D0C690
};
