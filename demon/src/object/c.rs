use core::ptr::null_mut;
use c_mine::{c_mine, get_hs_global};
use tag_structs::ObjectType;
use crate::object::{ObjectID, ObjectTypes, OBJECT_TABLE};

#[c_mine]
pub unsafe extern "C" fn object_try_and_get_verify_type(object_id: ObjectID, object_types: ObjectTypes) -> *mut [u8; 0] {
    let object_table = OBJECT_TABLE
        .get_copied()
        .expect("object_try_and_get_verify_type called with null object table");

    let Ok(object) = object_table.get_element(object_id) else {
        return null_mut()
    };

    let object = object.get();
    let Some(object_type) = object.try_get_object_type() else {
        return null_mut()
    };

    if object_types.contains(object_type) {
        object.object_data
    }
    else {
        null_mut()
    }
}

#[c_mine]
pub unsafe extern "C" fn object_get_and_verify_type(object_id: ObjectID, object_types: ObjectTypes) -> *mut [u8; 0] {
    let object = OBJECT_TABLE
        .get_copied()
        .expect("object_get_and_verify_type called with null object table")
        .get_element(object_id)
        .expect("object_get_and_verify_type called with invalid object ID")
        .get();

    let object_type = object.try_get_object_type().expect("object has invalid type");
    assert!(object_types.contains(object_type), "object_get_and_verified_type called with object type {object_type} but expecting {object_types:?}");
    object.object_data
}

#[c_mine]
pub unsafe extern "C" fn get_weapon_age(object_id: ObjectID) -> f32 {
    let infinite_ammo = get_cheat_infinite_ammo.get()();
    if infinite_ammo {
        return 0.0
    }

    let object = object_get_and_verify_type.get()(object_id, ObjectType::Weapon.into());

    // FIXME: define in a struct
    *(object.wrapping_byte_add(0x200) as *mut f32)
}

#[c_mine]
pub unsafe extern "C" fn get_cheat_infinite_ammo() -> bool {
    *get_hs_global!("cheat_infinite_ammo") != 0
}
