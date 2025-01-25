use crate::id::ID;
use crate::math::{ColorRGB, Vector3D};
use crate::player::PlayerID;
use crate::tag::TagID;

pub type ObjectID = ID<0x626F>;

#[repr(transparent)]
pub struct BaseDynamicObjectFlags(pub u32);

#[repr(transparent)]
pub struct BaseDynamicObjectHealthFlags(pub u16);

#[repr(u16)]
pub enum ObjectType {
    Biped,
    Vehicle,
    Weapon,
    Etc
}

/// WARNING: Some of this is probably bullshit, as much of this information comes from a bygone era
/// of Halo modding.
#[repr(C)]
pub struct BaseDynamicObject {
    pub tag_id: TagID,
    pub _unknown_0x4: u32,
    pub _unknown_0x8: u32,
    pub existence_type: u32,
    pub flags: BaseDynamicObjectFlags,
    pub _unknown_0x14: [u8; 0x48],
    pub position: Vector3D,
    pub velocity: Vector3D,
    pub orientation: [Vector3D; 2],
    pub rotation_velocity: Vector3D,
    pub _unknown_0x98: u32,
    pub _unknown_0x9c: u32,
    pub center: Vector3D,
    pub _unknown_0xac: u32,
    pub scale: f32,
    pub object_type: ObjectType,
    pub _unknown_0xb6: u16,
    pub _unknown_0xb8: [u8; 0x14],
    pub animation_tag_id: TagID,
    pub animation_index: u16,
    pub animation_frame: u16,
    pub _unknown_0xd4: u32,
    pub base_health: f32,
    pub base_shield: f32,
    pub health: f32,
    pub shield: f32,
    pub current_shield_damage: f32,
    pub current_health_damage: f32,
    pub _unknown_0xf0: u32,

    // 0x94
    pub recent_shield_damage: f32,
    pub recent_health_damage: f32,
    pub recent_shield_damage_time: u32,
    pub recent_health_damage_time: u32,
    pub shield_stun_time: u16,
    pub health_flags: BaseDynamicObjectHealthFlags,
    pub _unknown_0x108: [u8; 0x10],
    pub weapon: ObjectID,
    pub parent: ObjectID,
    pub parent_seat_index: u16,
    pub _unknown_0x122: u16,
    pub _unknown_0x124: [u8; 0x64],
    pub color_change: [ColorRGB; 4],
    pub color_change_2: [ColorRGB; 4],
    pub _unknown_0x1e8: [u8; 0xC]
}

const _: () = assert!(size_of::<BaseDynamicObject>() == 0x1F4);

#[repr(C)]
pub struct UnitRecentDamager {
    pub last_damage_time: u32,
    pub total_damage: f32,
    pub object: ObjectID,
    pub player: PlayerID
}

/// WARNING: Some of this is probably bullshit, as much of this information comes from a bygone era
/// of Halo modding.
#[repr(C)]
pub struct BaseDynamicUnit {
    pub object_data: BaseDynamicObject,
    pub _unknown_0x1f4: [u8; 0x10],
    pub flags: u32,
    pub control_flags: u16,
    pub _unknown_0x20a: u16,
    pub _unknown_0x20c: [u8; 0xC],
    pub player_id: PlayerID,
    pub _unknown_0x21c: u32,
    pub last_bullet_time: u32,
    pub facing: Vector3D,
    pub desired_aim: Vector3D,
    pub aim: Vector3D,
    pub wtf: [Vector3D; 3],
    pub _unknown_0x26c: [u8; 0xC],
    pub run: f32,
    pub strafe: f32,
    pub ascend: f32,
    pub shooting: f32,
    pub _unknown_0x288: [u8; 0xC],
    pub thrown_grenade_id: ObjectID,
    pub _unknown_0x298: [u8; 0x58],
    pub vehicle_seat: u16,
    pub weapon_slot: u16,
    pub _unknown_0x2f4: [u8; 4],
    pub weapons: [ObjectID; 4],
    pub _unknown_0x308: [u8; 0x14],
    pub current_grenade_slot: u8,
    pub _unknown_0x31d: u8,
    pub grenade_counts: [u8; 2],
    pub _unknown_0x320: [u8; 4],
    pub controller: ObjectID,
    pub gunner: ObjectID,
    pub _unknown_0x32c: [u8; 0x50],
    pub invisibility: f32,
    pub _unknown_0x380: [u8; 0xB0],
    pub recent_damagers: [UnitRecentDamager; 4],
    pub _unknown_0x470: [u8; 0x5c],
}

const _: () = assert!(size_of::<BaseDynamicUnit>() == 0x4CC);
