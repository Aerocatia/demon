use crate::object::{get_dynamic_object, BaseObject};
use crate::player::PLAYERS_TABLE;
use crate::script::HS_RETURN;

pub unsafe extern "C" fn demon_cheat_healthpack_eval(a: u16, b: u32, c: u8) {
    let table = PLAYERS_TABLE.get_copied().unwrap();
    for i in table.iter() {
        let Ok(object) = get_dynamic_object::<BaseObject>(i.get().unit) else { continue };
        object.health = object.health.max(1.0);
    }

    HS_RETURN.get()(b, 0);
}
