use c_mine::{c_mine, pointer_from_hook};
use crate::console::{fade_console_text, set_console_color_text, set_console_prompt_text, CONSOLE_ENABLED, CONSOLE_HISTORY_LENGTH, CONSOLE_HISTORY_SELECTED_INDEX, CONSOLE_IS_ACTIVE, CONSOLE_TEXT, DEFAULT_CONSOLE_COLOR, DEFAULT_CONSOLE_PROMPT_TEXT, TERMINAL_INITIALIZED, TERMINAL_OUTPUT_TABLE};
use crate::util::PointerProvider;

#[c_mine]
pub extern "C" fn console_is_active() -> bool {
    // SAFETY: This is known to be valid
    unsafe { *CONSOLE_IS_ACTIVE.get() != 0 }
}

#[c_mine]
pub unsafe extern "C" fn terminal_update() {
    if *TERMINAL_INITIALIZED.get() == 0 {
        return
    }

    const POLL_CONSOLE_INPUT: PointerProvider<extern "C" fn()> = pointer_from_hook!("poll_console_input");
    POLL_CONSOLE_INPUT.get()();

    let t = TERMINAL_OUTPUT_TABLE
        .get_copied()
        .expect("TERMINAL_OUTPUT_TABLE not initialized");

    if !console_is_active.get()() {
        fade_console_text(t);
    }
}

#[c_mine]
pub unsafe extern "C" fn console_initialize() {
    set_console_prompt_text(DEFAULT_CONSOLE_PROMPT_TEXT);
    set_console_color_text(DEFAULT_CONSOLE_COLOR);
    CONSOLE_TEXT.get_mut().fill(0);
    *CONSOLE_HISTORY_SELECTED_INDEX.get_mut() = 0xFFFF;
    *CONSOLE_HISTORY_LENGTH.get_mut() = 0;
    *CONSOLE_HISTORY_SELECTED_INDEX.get_mut() = 0xFFFF;
    *CONSOLE_ENABLED.get_mut() = true;
}
