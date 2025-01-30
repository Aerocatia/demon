use c_mine::pointer_from_hook;
use crate::id::ID;
use crate::math::Vector3D;
use crate::tag::TagID;
use crate::util::{PointerProvider, VariableProvider};

pub mod weapon;

pub const OBJECT_GET_AND_VERIFY_TYPE: PointerProvider<unsafe extern "C" fn(object_id: ObjectID, object_type_bitfield: ObjectTypes) -> *const [u8; 0]> = pointer_from_hook!("object_get_and_verify_type");

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

#[derive(Copy, Clone, PartialEq)]
#[repr(transparent)]
pub struct ObjectTypes(u32);
impl ObjectTypes {
    pub const fn from_slice(object_types: &[ObjectType]) -> Self {
        let mut index = 0usize;
        let mut value = 0u32;
        while index < object_types.len() {
            let o = Self::from_single(object_types[index]);
            value |= o.0;
            index += 1;
        }
        Self(value)
    }
    pub const fn from_single(object_type: ObjectType) -> Self {
        Self(1 << (object_type as u32))
    }
    pub const fn contains(&self, what: ObjectType) -> bool {
        (Self::from_single(what).0 & self.0) != 0
    }
}

impl From<ObjectType> for ObjectTypes {
    fn from(value: ObjectType) -> Self {
        Self::from_single(value)
    }
}

// TODO: Use definitions
#[derive(Copy, Clone, PartialEq, Debug)]
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
