pub mod print;

use core::ffi::CStr;
use core::mem::transmute;
use c_mine::{c_mine, pointer_from_hook};
use tag_structs::{ScenarioGlobal, ScenarioScriptValueType};
use crate::init::{get_exe_type, ExeType};
use crate::memory::table::{data_make_valid, game_state_data_new, DataTable};
use crate::tag::{ReflexiveImpl, GLOBAL_SCENARIO_INDEX};
use crate::tag::c::global_scenario_get;
use crate::util::{CStrPtr, PointerProvider, VariableProvider};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HSExternalGlobalDefinition {
    name: *const u8,
    global_type: ScenarioScriptValueType,
    padding: u16,
    ptr: *mut [u8; 0],
    unknown: u32
}

pub struct ExternalGlobal {
    name_buffer: &'static [u8],
    definition: HSExternalGlobalDefinition
}
impl ExternalGlobal {
    pub const fn new(name: &'static [u8], global_type: ScenarioScriptValueType, address: *mut [u8; 0]) -> Self {
        Self {
            name_buffer: name,
            definition: HSExternalGlobalDefinition {
                name: name.as_ptr(),
                global_type: global_type,
                padding: 0,
                ptr: address,
                unknown: 0
            }
        }
    }
    pub const fn name(&self) -> &str {
        let Ok(n) = CStr::from_bytes_until_nul(self.name_buffer) else {
            panic!("not null terminated")
        };
        let Ok(n) = n.to_str() else {
            panic!("not utf-8")
        };
        n
    }
}

const EXTERNAL_GLOBALS: (&[ExternalGlobal], &[ExternalGlobal]) = c_mine::generate_hs_external_globals_array!();

pub fn get_external_globals() -> &'static [ExternalGlobal] {
    let exe_type = get_exe_type();
    match exe_type {
        ExeType::Cache => EXTERNAL_GLOBALS.0,
        ExeType::Tag => EXTERNAL_GLOBALS.1
    }
}

#[c_mine]
pub unsafe extern "C" fn main_crash(param: CStrPtr) {
    for i in get_external_globals().iter().take(1) {
        console!("{}", i.name());
    }
    let message = param.as_str();
    panic!("crash command called with message:\n\n{message}");
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

pub unsafe fn get_scenario_globals() -> &'static [ScenarioGlobal] {
    if GLOBAL_SCENARIO_INDEX.get().is_null() {
        return &[];
    }
    global_scenario_get.get()().globals.as_slice()
}

trait HSGlobal {
    /// Name (not null terminated)
    fn name(&'static self) -> &'static str;

    /// Null terminated
    fn name_bytes(&'static self) -> &'static [u8];

    /// Type of global
    fn global_type(&self) -> ScenarioScriptValueType;
}

#[c_mine]
pub unsafe extern "C" fn hs_find_global_by_name(name: CStrPtr) -> u32 {
    let name = name.as_str();

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

fn get_global_by_index(index: u16) -> &'static dyn HSGlobal {
    let global = if (index & 0x8000) == 0 {
        match unsafe { get_scenario_globals().get(index as usize) } {
            Some(n) => n as &dyn HSGlobal,
            None => panic!("No scenario global of index 0x{index:04X}")
        }
    }
    else {
        match get_external_globals().get((index as usize) & 0x7FFF) {
            Some(n) => n as &dyn HSGlobal,
            None => panic!("No external global of index 0x{index:04X}")
        }
    };
    global
}

impl HSGlobal for ExternalGlobal {
    fn name(&'static self) -> &'static str {
        self.name()
    }
    fn name_bytes(&'static self) -> &'static [u8] {
        self.name_buffer
    }
    fn global_type(&self) -> ScenarioScriptValueType {
        self.definition.global_type
    }
}

impl HSGlobal for ScenarioGlobal {
    fn name(&'static self) -> &'static str {
        self.name.to_str()
    }
    fn name_bytes(&'static self) -> &'static [u8] {
        self.name.to_str().as_bytes()
    }
    fn global_type(&self) -> ScenarioScriptValueType {
        self.r#type.get()
    }
}

const HS_ENUMERATE_ADD_RESULT: PointerProvider<unsafe extern "C" fn(what: *const u8)> = pointer_from_hook!("hs_enumerate_add_result");

#[c_mine]
pub unsafe extern "C" fn hs_enumerate_globals() {
    for i in get_external_globals() {
        if i.definition.ptr.is_null() {
            continue
        }
        HS_ENUMERATE_ADD_RESULT.get()(i.name_buffer.as_ptr())
    }
    for i in get_scenario_globals() {
        HS_ENUMERATE_ADD_RESULT.get()(i.name.to_str().as_ptr())
    }
}

#[c_mine]
pub unsafe extern "C" fn hs_global_get_name(index: u16) -> *const u8 {
    get_global_by_index(index).name_bytes().as_ptr()
}

#[c_mine]
pub unsafe extern "C" fn hs_global_get_type(index: u16) -> ScenarioScriptValueType {
    get_global_by_index(index).global_type()
}

const HS_THREAD_TABLE: VariableProvider<Option<&mut DataTable<[u8; 0x100], 0x7368>>> = variable! {
    name: "hs_thread_table",
    cache_address: 0x00C7DE48,
    tag_address: 0x00D35400
};

#[repr(C)]
struct HSGlobalTableEntry {
    pub identifier: u16,
    pub unknown: u16,
    pub value: u32
}

const HS_GLOBALS_TABLE: VariableProvider<Option<&mut DataTable<HSGlobalTableEntry, 0x7368>>> = variable! {
    name: "hs_globals_table",
    cache_address: 0x00C7DE4C,
    tag_address: 0x00D35404
};

const DATUM_NEW_AT_INDEX: PointerProvider<unsafe extern "C" fn(table: *mut DataTable<[u8; 0], 0>, index: u32) -> u32> = pointer_from_hook!("datum_new_at_index");

const HS_EXTERNAL_GLOBALS_COUNT: VariableProvider<u16> = variable! {
    name: "HS_EXTERNAL_GLOBALS_COUNT",
    cache_address: 0x0097E8CC,
    tag_address: 0x009E2D94
};

#[c_mine]
pub unsafe extern "C" fn hs_runtime_initialize() {
    let hs_thread_table = game_state_data_new.get()(
        CStrPtr::from_bytes(b"hs thread\x00"),
        0x100,
        0x218
    );
    if hs_thread_table.is_null() {
        panic!("hs_runtime_initialize failed to allocate hs_thread_table")
    }
    let hs_globals_table = game_state_data_new.get()(
        CStrPtr::from_bytes(b"hs globals\x00"),
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
