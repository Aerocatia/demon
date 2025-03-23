use tag_structs::Weapon;
use crate::object::{get_dynamic_object, BaseObject, ObjectID};
use crate::tag::{get_tag_info_typed, ReflexiveImpl};

pub mod c;

pub unsafe fn refill_weapon(object_id: ObjectID) {
    // TODO: Convert to DynamicWeapon to get weapon
    let Ok(weapon) = get_dynamic_object::<BaseObject>(object_id) else { return };

    // TODO: multiple magazines
    let unloaded = &mut *((weapon as *mut _ as *mut u8).wrapping_add(0x26E) as *mut u16);
    let loaded = &mut *((weapon as *mut _ as *mut u8).wrapping_add(0x270) as *mut u16);
    let age = (weapon as *mut _ as *mut u8).wrapping_add(0x200) as *mut f32;

    *age = 0.0;

    let Ok((_, weapon_tag_data)) = get_tag_info_typed::<Weapon>(weapon.tag_id) else { return };
    let Some(m) = weapon_tag_data.magazines.get(0) else { return };
    *unloaded = (*unloaded).max(m.rounds_reserved_maximum as u16);
    *loaded = (*loaded).max(m.rounds_loaded_maximum as u16);
}

pub unsafe fn empty_weapon(object_id: ObjectID) {
    // TODO: Convert to DynamicWeapon to get weapon
    let Ok(weapon) = get_dynamic_object::<BaseObject>(object_id) else { return };

    // TODO: multiple magazines
    let unloaded = &mut *((weapon as *mut _ as *mut u8).wrapping_add(0x26E) as *mut u16);
    let loaded = &mut *((weapon as *mut _ as *mut u8).wrapping_add(0x270) as *mut u16);
    let age = (weapon as *mut _ as *mut u8).wrapping_add(0x200) as *mut f32;

    *age = 1.0;
    *unloaded = 0;
    *loaded = 0;
}
