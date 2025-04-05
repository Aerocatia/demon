use alloc::string::String;
use core::mem::transmute;
use c_mine::{c_mine, pointer_from_hook};
use tag_structs::primitives::color::{ColorARGB, ColorRGB};
use tag_structs::ScenarioScriptValueType;
use crate::init::get_exe_type;
use crate::memory::table::{data_make_valid, game_state_data_new};
use crate::script::{get_external_globals, get_functions, get_global_by_index, get_scenario_globals, HSExternalGlobalDefinition, HSScriptFunctionDefinition, DATUM_NEW_AT_INDEX, HS_ENUMERATE_ADD_RESULT, HS_EXTERNAL_GLOBALS_COUNT, HS_GLOBALS_TABLE, HS_THREAD_TABLE};
use crate::util::{CStrPtr, PointerProvider, StaticStringBytes};

#[c_mine]
pub unsafe extern "C" fn hs_runtime_initialize() {
    let hs_thread_table = game_state_data_new.get()(
        CStrPtr::from_cstr(c"hs thread"),
        0x100,
        0x218
    );
    if hs_thread_table.is_null() {
        panic!("hs_runtime_initialize failed to allocate hs_thread_table")
    }
    let hs_globals_table = game_state_data_new.get()(
        CStrPtr::from_cstr(c"hs globals"),
        0x400,
        8
    );
    if hs_globals_table.is_null() {
        panic!("hs_runtime_initialize failed to allocate hs_globals_table")
    }

    // safe to transmute pointers to optional refs
    *HS_THREAD_TABLE.get_mut() = transmute(hs_thread_table);
    *HS_GLOBALS_TABLE.get_mut() = transmute(hs_globals_table);
    data_make_valid.get()(transmute(hs_globals_table));

    let globals = get_external_globals();
    for (index, _) in globals.iter().enumerate() {
        let result = DATUM_NEW_AT_INDEX.get()(hs_globals_table, (0xACED0000 | (index & 0xFFFF)) as u32);
        assert_ne!(result, 0xFFFFFFFF, "hs_runtime_initialize can't set up external globals")
    }

    crate::init::sudo_write(
        HS_EXTERNAL_GLOBALS_COUNT.get_mut() as *mut _,
        globals.len() as u16
    );
}

#[c_mine]
pub unsafe extern "C" fn hs_global_get_value(index: u16) -> u32 {
    let external_globals = get_external_globals();

    if (index & 0x8000) == 0 {
        let actual_index = (index as usize) + external_globals.len();

        let Ok(global) = HS_GLOBALS_TABLE
            .get_copied()
            .unwrap()
            .get_element(transmute(actual_index))
            .map(|g| g.get().value) else {
            panic!("Failed to get scenario global {index} ({actual_index} in table)")
        };

        global
    }
    else {
        let Some(global) = external_globals.get((index & 0x7FFF) as usize) else {
            panic!("Failed to get external global {index}")
        };

        let pointer = global.definition.ptr;
        if pointer.is_null() {
            0
        }
        else {
            *(pointer as *const u32)
        }
    }
}

#[c_mine]
pub extern "C" fn hs_global_external_get(index: u16) -> &'static HSExternalGlobalDefinition {
    let globals = get_external_globals();
    let Some(global) = globals.get(index as usize) else {
        let global_count = globals.len();
        let exe_type = get_exe_type();
        panic!("hs_global_external_get tried to get global {index}, but there are only {global_count} globals for {exe_type:?} builds")
    };
    &global.definition
}

#[c_mine]
pub unsafe extern "C" fn hs_find_global_by_name(name: CStrPtr) -> u32 {
    let name = name.expect_str();

    const MAX_GLOBALS: usize = 0x8000;

    let globals = get_external_globals();
    for (index, global) in globals.iter().take(MAX_GLOBALS).enumerate() {
        if global.name() == name {
            return ((index & 0x7FFF) | 0x8000) as u32
        }
    }

    for (index, global) in get_scenario_globals().iter().take(MAX_GLOBALS).enumerate() {
        if global.name.to_str() == name {
            return index as u32
        }
    }

    u32::MAX
}

#[c_mine]
pub unsafe extern "C" fn hs_enumerate_globals() {
    for i in get_external_globals() {
        if i.definition.ptr.is_null() {
            continue
        }
        HS_ENUMERATE_ADD_RESULT.get()(i.name_buffer.as_ptr() as *const u8)
    }
    for i in get_scenario_globals() {
        HS_ENUMERATE_ADD_RESULT.get()(i.name.to_str().as_ptr())
    }
}

#[c_mine]
pub unsafe extern "C" fn hs_enumerate_functions() {
    for i in get_functions() {
        HS_ENUMERATE_ADD_RESULT.get()(i.name.0 as *const _);
    }
}

#[c_mine]
pub unsafe extern "C" fn hs_global_get_name(index: u16) -> *const u8 {
    // SAFETY: External globals are safe; scenario globals are only safe if they are null terminated.
    get_global_by_index(index).name().as_ptr()
}

#[c_mine]
pub unsafe extern "C" fn hs_global_get_type(index: u16) -> ScenarioScriptValueType {
    get_global_by_index(index).global_type()
}

#[c_mine]
pub unsafe extern "C" fn display_scripting_error(file: CStrPtr, reason: CStrPtr, code: CStrPtr, source: CStrPtr) {
    let offending_line = code.as_cstr().to_bytes();
    let eol = offending_line
        .iter()
        .enumerate()
        .filter(|(index, b)| **b == b'\n' || **b == b'\r')
        .map(|(index, _)| index)
        .next()
        .unwrap_or(offending_line.len());
    let offending_line = String::from_utf8_lossy(&offending_line[..eol]);

    let error_message = StaticStringBytes::<1024>::from_fmt(
        format_args!("{}: {offending_line}", reason.display_lossy())
    ).unwrap();

    let location: StaticStringBytes<256> = if file.is_null() {
        StaticStringBytes::from_fmt(
            format_args!("console")
        )
    }
    else {
        let source_bytes = source.as_cstr().to_bytes();
        let code_at = code.as_cstr().to_bytes().as_ptr();
        let source_start = source_bytes.as_ptr();
        let difference = (code_at as usize).checked_sub(source_start as usize).expect("code/source passed into display_scripting_error were bad");
        assert!(difference <= source_bytes.len(), "<code> not in <source> in display_scripting_error");

        let range = core::slice::from_raw_parts(source_start, difference);
        let line = range.iter().filter(|b| **b == b'\n').count();

        StaticStringBytes::from_fmt(
            format_args!("{}:{line}", file.display_lossy())
        )
    }.unwrap();

    debug_log!("HSC syntax error @ {location} - {error_message}");

    console_color!(ColorARGB { a: 1.0, color: ColorRGB { r: 1.0, g: 0.0, b: 0.0 } }, "HSC syntax error @ {location}:");
    console_color!(ColorARGB { a: 1.0, color: ColorRGB { r: 1.0, g: 0.0, b: 0.0 } }, "- {error_message}");
}

#[c_mine]
pub unsafe extern "C" fn hs_find_function_by_name(name: CStrPtr) -> usize {
    let Some(name) = name.to_str_lossless() else { return usize::MAX };

    let Some(f) = get_functions()
        .iter()
        .position(|p| p.name.expect_str().eq_ignore_ascii_case(name)) else {

        return usize::MAX;
    };

    f
}

#[c_mine]
pub unsafe extern "C" fn hs_get_function(index: u16) -> &'static HSScriptFunctionDefinition<0> {
    match get_functions().get(index as usize) {
        Some(n) => n,
        None => panic!("hs_get_function with index {index}")
    }
}

const HS_MACRO_FUNCTION_PARSE: PointerProvider<unsafe extern "C" fn(u32, u32) -> u32> = pointer_from_hook!("hs_macro_function_parse");

pub unsafe extern "C" fn hs_macro_function_parse_c(a: u32, b: u32) -> u32 {
    HS_MACRO_FUNCTION_PARSE.get()(a,b)
}
