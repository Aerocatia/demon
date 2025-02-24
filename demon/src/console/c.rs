use c_mine::{c_mine, pointer_from_hook};
use tag_structs::primitives::color::{ColorARGB, ColorRGB};
use crate::console::{CONSOLE_IS_ACTIVE_HALO, CONSOLE_BUFFER, CONSOLE_INPUT_TEXT, CONSOLE_CURSOR_POSITION, console_put_args};
use crate::util::{CStrPtr, PointerProvider, VariableProvider};


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
    unsafe { *CONSOLE_IS_ACTIVE_HALO.get() != 0 }
}

#[c_mine]
pub unsafe extern "C" fn terminal_draw() {
    super::render_console();
}

const KEYBOARD_CONTROL_BUTTONS: VariableProvider<u32> = variable! {
    name: "KEYBOARD_CONTROL_BUTTONS",
    cache_address: 0x00C80358
};
const MOUSE_CONTROL_BUTTONS: VariableProvider<u32> = variable! {
    name: "MOUSE_CONTROL_BUTTONS",
    cache_address: 0x00C81748
};
const UNK: VariableProvider<u16> = variable! {
    name: "UNK",
    cache_address: 0x00C98AE8
};

#[c_mine]
pub unsafe extern "C" fn terminal_update() {
    let old_position = CONSOLE_CURSOR_POSITION.get_copied();

    const POLL_CONSOLE_INPUT: PointerProvider<extern "C" fn()> = pointer_from_hook!("poll_console_input");
    POLL_CONSOLE_INPUT.get()();

    let new_position = CONSOLE_CURSOR_POSITION.get_copied();

    if old_position != new_position {
        CONSOLE_BUFFER.read().input_cursor_timer.start();
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

const HS_DISABLE_FORCE_LOWERCASE: VariableProvider<u8> = variable! {
    name: "HS_DISABLE_FORCE_LOWERCASE",
    cache_address: 0x00C7DCB0,
    tag_address: 0x00D35268
};

const HS_COMPILE_AND_EVALUATE: PointerProvider<unsafe extern "C" fn(CStrPtr)> = pointer_from_hook!("hs_compile_and_evaluate");

const CONSOLE_HISTORY_MAX_ENTRIES: usize = 8;
const CONSOLE_HISTORY_MAX_LENGTH_PER_ENTRY: usize = 255;

const CONSOLE_HISTORY_TEXT: VariableProvider<[[u8; CONSOLE_HISTORY_MAX_LENGTH_PER_ENTRY]; CONSOLE_HISTORY_MAX_ENTRIES]> = variable! {
    name: "console_history_text",
    cache_address: 0x00C98CA4,
    tag_address: 0x00D50264
};

#[c_mine]
pub unsafe extern "C" fn run_console_command(command: CStrPtr) {
    const ERROR_COLOR: ColorARGB = ColorRGB { r: 1.0, g: 0.0, b: 0.0 }.as_colorargb();

    let Some(command_str) = command.to_str_lossless() else {
        console_put_args(Some(&ERROR_COLOR), format_args!("Failed to execute console command: input is not UTF-8."));
        return;
    };

    let Some(first_character) = command_str.chars().filter(|c| !c.is_whitespace()).next() else {
        return;
    };

    let command_bytes = command.as_cstr().to_bytes_with_nul();
    if command_bytes.len() > CONSOLE_HISTORY_MAX_LENGTH_PER_ENTRY {
        console_put_args(Some(&ERROR_COLOR), format_args!("Failed to execute console command: command is too long."));
        return;
    }

    let console_history_next_index = CONSOLE_HISTORY_NEXT_INDEX.get_mut();
    let full_history = CONSOLE_HISTORY_TEXT.get_mut();
    let console_history_length = CONSOLE_HISTORY_LENGTH.get_mut();

    if *console_history_length == CONSOLE_HISTORY_MAX_ENTRIES as u16 {
        for i in 1..CONSOLE_HISTORY_MAX_ENTRIES {
            full_history[i - 1] = full_history[i];
        }
    }
    else {
        *console_history_length = (*console_history_length + 1).min(CONSOLE_HISTORY_MAX_ENTRIES as u16);
    }
    full_history[*console_history_length as usize - 1][..command_bytes.len()].copy_from_slice(command_bytes);
    *console_history_next_index = *console_history_length - 1;
    *CONSOLE_HISTORY_SELECTED_INDEX.get_mut() = 0xFFFF;

    *HS_DISABLE_FORCE_LOWERCASE.get_mut() = 1;
    HS_COMPILE_AND_EVALUATE.get()(command);
    *HS_DISABLE_FORCE_LOWERCASE.get_mut() = 0;
}
