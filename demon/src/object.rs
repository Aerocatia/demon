use core::fmt::{Debug, Formatter};
use tag_structs::{ModelMarkerInstance, ObjectType};
use tag_structs::primitives::vector::{Matrix4x3, Vector3D};
use crate::id::ID;
use crate::memory::table::DataTable;
use crate::tag::TagID;
use crate::util::VariableProvider;

pub mod weapon;
pub mod c;

const OBJECT_SALT: u16 = 0x626F;

pub type ObjectID = ID<OBJECT_SALT>;

#[repr(C)]
pub struct ObjectIndex {
    pub identifier: u16,
    pub _unknown_0x2: u8,
    pub object_type: u8,
    pub _unknown_0x4: u32,
    pub object_data: *mut [u8; 0]
}

impl ObjectIndex {
    pub fn try_get_object_type(&self) -> Option<ObjectType> {
        ObjectType::try_from(self.object_type as u16).ok()
    }
}

pub const OBJECT_TABLE: VariableProvider<Option<&mut DataTable<ObjectIndex, OBJECT_SALT>>> = variable! {
    name: "OBJECT_TABLE",
    cache_address: 0x00DED5A8,
    tag_address: 0x00EA4B68
};

#[repr(transparent)]
pub struct BaseDynamicObjectFlags(pub u32);

#[repr(transparent)]
pub struct BaseDynamicObjectHealthFlags(pub u16);

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct ObjectMarker {
    /// Refers to the model node the marker is attached to.
    pub node_index: u16,

    /// Padding probably.
    pub _unknown_0x2: [u8; 2],

    /// Derived from a model tag's marker instance (translation and rotation).
    ///
    /// Appears to only be used in the `model_get_marker_by_name` function, where it's written to
    /// and then never read once the function returns...
    pub marker_instance_matrix_tmp: Matrix4x3,

    /// Object node matrix * model tag marker
    pub matrix: Matrix4x3
}

impl ObjectMarker {
    pub fn new(node_index: u16, node: &Matrix4x3, marker_instance: &ModelMarkerInstance, flip: bool) -> Self {
        let marker_instance_matrix_tmp = Matrix4x3::from_point_and_quaternion(
            marker_instance.translation,
            marker_instance.rotation
        );
        let mut matrix = *node * marker_instance_matrix_tmp;
        if flip {
            matrix.rotation_matrix.b = -matrix.rotation_matrix.b;
        }
        ObjectMarker {
            node_index, marker_instance_matrix_tmp, matrix, _unknown_0x2: [0u8; 2]
        }
    }
}

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
        Self(1u32.wrapping_shl(object_type as u32))
    }
    pub const fn contains(&self, what: ObjectType) -> bool {
        (Self::from_single(what).0 & self.0) != 0
    }
}

impl Debug for ObjectTypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str("ObjectTypes [ ")?;
        if self.contains(ObjectType::Biped) {
            f.write_fmt(format_args!(" {:?}", ObjectType::Biped))?;
        }
        if self.contains(ObjectType::Vehicle) {
            f.write_fmt(format_args!(" {:?}", ObjectType::Vehicle))?;
        }
        if self.contains(ObjectType::Weapon) {
            f.write_fmt(format_args!(" {:?}", ObjectType::Weapon))?;
        }
        if self.contains(ObjectType::Equipment) {
            f.write_fmt(format_args!(" {:?}", ObjectType::Equipment))?;
        }
        if self.contains(ObjectType::Garbage) {
            f.write_fmt(format_args!(" {:?}", ObjectType::Garbage))?;
        }
        if self.contains(ObjectType::Projectile) {
            f.write_fmt(format_args!(" {:?}", ObjectType::Projectile))?;
        }
        if self.contains(ObjectType::Scenery) {
            f.write_fmt(format_args!(" {:?}", ObjectType::Scenery))?;
        }
        if self.contains(ObjectType::DeviceMachine) {
            f.write_fmt(format_args!(" {:?}", ObjectType::DeviceMachine))?;
        }
        if self.contains(ObjectType::DeviceControl) {
            f.write_fmt(format_args!(" {:?}", ObjectType::DeviceControl))?;
        }
        if self.contains(ObjectType::DeviceLightFixture) {
            f.write_fmt(format_args!(" {:?}", ObjectType::DeviceLightFixture))?;
        }
        if self.contains(ObjectType::Placeholder) {
            f.write_fmt(format_args!(" {:?}", ObjectType::Placeholder))?;
        }
        if self.contains(ObjectType::SoundScenery) {
            f.write_fmt(format_args!(" {:?}", ObjectType::SoundScenery))?;
        }
        f.write_str(" ]")?;
        Ok(())
    }
}

impl From<ObjectType> for ObjectTypes {
    fn from(value: ObjectType) -> Self {
        Self::from_single(value)
    }
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
