use crate::object::weapon::refill_weapon;
use crate::object::{get_dynamic_object, BaseObject, ObjectID, MAXIMUM_NUMBER_OF_HELD_WEAPONS};
use crate::player::PLAYERS_TABLE;
use crate::script::HS_RETURN;
use crate::tag::{get_tag_data_from_info, lookup_tag, ReflexiveImpl};
use tag_structs::primitives::tag_group::TagGroup;
use tag_structs::Globals;

pub unsafe extern "C" fn cheat_max_ammo_eval(a: u16, b: u32, c: u8) {
    let table = PLAYERS_TABLE.get_copied().unwrap();
    for i in table.iter() {
        // TODO: Convert to BaseUnit to get weapons of unit
        let Ok(unit) = get_dynamic_object::<BaseObject>(i.get().unit) else { continue };

        let frags = &mut *(unit as *mut _ as *mut u8).wrapping_add(0x2E2);
        let plasmas = &mut *(unit as *mut _ as *mut u8).wrapping_add(0x2E3);

        let (globals_tag_info, _) = lookup_tag("globals\\globals", TagGroup::Globals.into()).unwrap();
        let globals_tag = get_tag_data_from_info::<Globals>(globals_tag_info).unwrap();
        *frags = (*frags).max(globals_tag.grenades.get(0).map(|c| c.maximum_count as u8).unwrap_or(0));
        *plasmas = (*plasmas).max(globals_tag.grenades.get(1).map(|c| c.maximum_count as u8).unwrap_or(0));

        let weapons = (unit as *mut _ as *mut u8).wrapping_add(0x2BC) as *mut [ObjectID; MAXIMUM_NUMBER_OF_HELD_WEAPONS];
        for i in *weapons {
            refill_weapon(i);
        }

        let current_weapon = (unit as *mut _ as *mut u8).wrapping_add(0xD0) as *const ObjectID;
        refill_weapon(*current_weapon);
    }

    HS_RETURN.get()(b, 0);
}

