use crate::map::load_main_menu;
use crate::script::HS_RETURN;

pub unsafe extern "C" fn exit_to_menu_eval(a: u16, b: u32, c: u8) {
    load_main_menu();
    HS_RETURN.get()(b, 0);
}
