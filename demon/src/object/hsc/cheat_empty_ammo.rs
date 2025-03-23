use crate::object::weapon::empty_weapon;
use crate::object::{get_dynamic_object, BaseObject, ObjectID, MAXIMUM_NUMBER_OF_HELD_WEAPONS};
use crate::player::PLAYERS_TABLE;
use crate::script::HS_RETURN;

pub unsafe extern "C" fn cheat_empty_ammo_eval(a: u16, b: u32, c: u8) {
    let table = PLAYERS_TABLE.get_copied().unwrap();
    for i in table.iter() {
        // TODO: Convert to BaseUnit to get weapons of unit
        let Ok(unit) = get_dynamic_object::<BaseObject>(i.get().unit) else { continue };

        let frags = &mut *(unit as *mut _ as *mut u8).wrapping_add(0x2E2);
        let plasmas = &mut *(unit as *mut _ as *mut u8).wrapping_add(0x2E3);

        *frags = 0;
        *plasmas = 0;

        let weapons = (unit as *mut _ as *mut u8).wrapping_add(0x2BC) as *mut [ObjectID; MAXIMUM_NUMBER_OF_HELD_WEAPONS];
        for i in *weapons {
            empty_weapon(i);
        }

        let current_weapon = (unit as *mut _ as *mut u8).wrapping_add(0xD0) as *const ObjectID;
        empty_weapon(*current_weapon);
    }

    HS_RETURN.get()(b, 0);
}

