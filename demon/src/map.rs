pub mod hsc;

use crate::util::VariableProvider;

const SHOULD_LOAD_MAIN_MENU: VariableProvider<u8> = variable! {
    name: "should_load_main_menu",
    cache_address: 0x00C99727,
    tag_address: 0x00D50CFF
};

/// Load the main menu, instantly ending the current game on the next main_loop iteration.
pub unsafe fn load_main_menu() {
    *SHOULD_LOAD_MAIN_MENU.get_mut() = 1;
}
