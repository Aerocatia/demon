use c_mine::{c_mine, get_hs_global};
use crate::object::{object_get_and_verify_type, ObjectID, ObjectType};

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
