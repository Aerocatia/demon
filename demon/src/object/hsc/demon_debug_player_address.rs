use crate::object::{get_dynamic_object, BaseObject};
use crate::player::PLAYERS_TABLE;
use crate::script::HS_RETURN;
use crate::util::display_utf16;

pub unsafe extern "C" fn demon_debug_player_address_eval(a: u16, b: u32, c: u8) {
    let table = PLAYERS_TABLE.get_copied().unwrap();
    for i in table.iter() {
        let Ok(object) = get_dynamic_object::<BaseObject>(i.get().unit) else {
            console!("Player \"{}\" - Dead", display_utf16(&i.get().name));
            continue;
        };

        console!("Player \"{}\" - OID = 0x{:08X}, OAddress = 0x{:08X}", display_utf16(&i.get().name), i.get().unit.full_id(), (object as *mut _) as usize);
    }

    HS_RETURN.get()(b, 0);
}
