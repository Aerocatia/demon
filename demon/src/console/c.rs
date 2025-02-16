use c_mine::{c_mine, pointer_from_hook};
use crate::console::{CONSOLE_IS_ACTIVE, CONSOLE_BUFFER, CONSOLE_INPUT_TEXT, CONSOLE_CURSOR_POSITION};
use crate::util::{PointerProvider, VariableProvider};

const CONSOLE_PROMPT_TEXT: VariableProvider<[u8; 32]> = variable! {
    name: "CONSOLE_PROMPT_TEXT",
    cache_address: 0x00C98B78,
    tag_address: 0x00D50138
};

const CONSOLE_HISTORY_LENGTH: VariableProvider<u16> = variable! {
    name: "CONSOLE_HISTORY_LENGTH",
    cache_address: 0x00C9949C,
    tag_address: 0x00D5015C
};

const CONSOLE_HISTORY_NEXT_INDEX: VariableProvider<u16> = variable! {
    name: "CONSOLE_HISTORY_NEXT_INDEX",
    cache_address: 0x00C9949E,
    tag_address: 0x00D5015E
};

const CONSOLE_HISTORY_SELECTED_INDEX: VariableProvider<u16> = variable! {
    name: "CONSOLE_HISTORY_SELECTED_INDEX",
    cache_address: 0x00C994A0,
    tag_address: 0x00D50160
};

const CONSOLE_ENABLED: VariableProvider<bool> = variable! {
    name: "CONSOLE_ENABLED",
    cache_address: 0x00C98AE1,
    tag_address: 0x00D500A1
};

#[c_mine]
pub extern "C" fn console_is_active() -> bool {
    // SAFETY: This is known to be valid
    unsafe { *CONSOLE_IS_ACTIVE.get() != 0 }
}

#[c_mine]
pub unsafe extern "C" fn terminal_draw() {
    super::render_console();
}

#[c_mine]
pub unsafe extern "C" fn terminal_update() {
    let old_position = CONSOLE_CURSOR_POSITION.get_copied();

    const POLL_CONSOLE_INPUT: PointerProvider<extern "C" fn()> = pointer_from_hook!("poll_console_input");
    POLL_CONSOLE_INPUT.get()();

    let new_position = CONSOLE_CURSOR_POSITION.get_copied();

    if old_position != new_position {
        CONSOLE_BUFFER.read().cursor_timer.start();
    }
}

#[c_mine]
pub unsafe extern "C" fn console_initialize() {
    CONSOLE_INPUT_TEXT.get_mut().fill(0);
    *CONSOLE_HISTORY_SELECTED_INDEX.get_mut() = 0xFFFF;
    *CONSOLE_HISTORY_LENGTH.get_mut() = 0;
    *CONSOLE_HISTORY_SELECTED_INDEX.get_mut() = 0xFFFF;
    *CONSOLE_ENABLED.get_mut() = true;
}
