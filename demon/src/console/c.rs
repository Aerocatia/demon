use c_mine::{c_mine, pointer_from_hook};
use crate::console::{fade_console_text, CONSOLE_ENABLED, CONSOLE_HISTORY_LENGTH, CONSOLE_HISTORY_SELECTED_INDEX, CONSOLE_IS_ACTIVE, CONSOLE_INPUT_TEXT, TERMINAL_INITIALIZED, TERMINAL_OUTPUT_TABLE, CONSOLE_CURSOR_POSITION, CONSOLE_BUFFER};
use crate::util::PointerProvider;

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
    if *TERMINAL_INITIALIZED.get() == 0 {
        return
    }

    let old_position = CONSOLE_CURSOR_POSITION.get_copied();

    const POLL_CONSOLE_INPUT: PointerProvider<extern "C" fn()> = pointer_from_hook!("poll_console_input");
    POLL_CONSOLE_INPUT.get()();

    let new_position = CONSOLE_CURSOR_POSITION.get_copied();

    if old_position != new_position {
        CONSOLE_BUFFER.read().cursor_timer.start();
    }

    let t = TERMINAL_OUTPUT_TABLE
        .get_copied()
        .expect("TERMINAL_OUTPUT_TABLE not initialized");

    if !console_is_active.get()() {
        fade_console_text(t);
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
