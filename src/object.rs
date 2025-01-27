use crate::id::ID;
use crate::math::Vector3D;
use crate::tag::TagID;
use crate::util::VariableProvider;

pub type ObjectID = ID<0x626F>;

#[repr(transparent)]
pub struct BaseDynamicObjectFlags(pub u32);

#[repr(transparent)]
pub struct BaseDynamicObjectHealthFlags(pub u16);

#[repr(C)]
pub struct ModelNode {
    pub scale: f32,
    pub rotation_matrix: [Vector3D; 3],
    pub position: Vector3D
}

const _: () = assert!(size_of::<ModelNode>() == 0x34);

// TODO: Use definitions
#[repr(u16)]
pub enum ObjectType {
    Biped,
    Vehicle,
    Weapon,
    Equipment,
    Garbage,
    Projectile,
    Scenery,
    DeviceMachine,
    DeviceControl,
    DeviceLightFixture,
    Placeholder,
    SoundScenery
}

pub const GLOBAL_OBJECT_MARKER: VariableProvider<u32> = variable! {
    name: "global_object_marker",
    cache_address: 0x00DED5B4,
    tag_address: 0x00EA4B74
};

#[repr(C)]
pub struct BaseObject {
    pub tag_id: TagID,
    pub object_type: ObjectType,
    pub _unknown_0x06: u16,
    pub _unknown_0x08: u32,
    pub _unknown_0x0c: u32,
    pub _unknown_0x10: u32,

    /// A copy of the `GLOBAL_OBJECT_MARKER` value
    pub global_object_marker: u32,

    pub position: Vector3D,
    pub velocity: Vector3D
}
