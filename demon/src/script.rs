pub mod print;
pub mod c;
pub mod crash;
pub mod cls;
pub mod script_doc;

use core::ffi::CStr;
use c_mine::pointer_from_hook;
use tag_structs::{ScenarioGlobal, ScenarioScriptValueType};
use crate::init::{get_exe_type, ExeType};
use crate::memory::table::DataTable;
use crate::tag::{ReflexiveImpl, GLOBAL_SCENARIO_INDEX};
use crate::tag::c::global_scenario_get;
use crate::util::{CStrPtr, PointerProvider, VariableProvider};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HSExternalGlobalDefinition {
    name: CStrPtr,
    global_type: ScenarioScriptValueType,
    padding: u16,
    ptr: *mut [u8; 0],
    unknown: u32
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HSScriptFunctionDefinition {
    pub return_type: ScenarioScriptValueType,
    pub padding_0x2: u16,
    pub name: CStrPtr,
    pub compile: *const [u8; 0],
    pub evaluate: *const [u8; 0],
    pub description: CStrPtr,
    pub usage: CStrPtr,
    pub unknown1: u16,
    pub argument_count: u16,
    pub argument_types: [ScenarioScriptValueType; 6]
}

#[derive(Copy, Clone)]
pub struct ExternalGlobal {
    name_buffer: &'static CStr,
    definition: HSExternalGlobalDefinition
}
impl ExternalGlobal {
    pub const fn new(name: &'static CStr, global_type: ScenarioScriptValueType, address: *mut [u8; 0]) -> Self {
        Self {
            name_buffer: name,
            definition: HSExternalGlobalDefinition {
                name: CStrPtr::from_cstr(name),
                global_type: global_type,
                padding: 0,
                ptr: address,
                unknown: 0
            }
        }
    }
    pub const fn name(&self) -> &str {
        let Ok(n) = self.name_buffer.to_str() else {
            panic!("not utf-8")
        };
        n
    }
}

const EXTERNAL_GLOBALS: (&[ExternalGlobal], &[ExternalGlobal]) = c_mine::generate_hs_external_globals_array!();
const SCRIPT_FUNCTIONS: (&[HSScriptFunctionDefinition], &[HSScriptFunctionDefinition]) = c_mine::generate_hs_functions_array!();

pub fn get_external_globals() -> &'static [ExternalGlobal] {
    let exe_type = get_exe_type();
    match exe_type {
        ExeType::Cache => EXTERNAL_GLOBALS.0,
        ExeType::Tag => EXTERNAL_GLOBALS.1
    }
}

pub fn get_functions() -> &'static [HSScriptFunctionDefinition] {
    let exe_type = get_exe_type();
    match exe_type {
        ExeType::Cache => SCRIPT_FUNCTIONS.0,
        ExeType::Tag => SCRIPT_FUNCTIONS.1
    }
}

pub unsafe fn get_scenario_globals() -> &'static [ScenarioGlobal] {
    if GLOBAL_SCENARIO_INDEX.get().is_null() {
        return &[];
    }
    global_scenario_get.get()().globals.as_slice()
}

trait HSGlobal {
    /// Name
    fn name(&'static self) -> &'static str;

    /// Type of global
    fn global_type(&self) -> ScenarioScriptValueType;
}

unsafe fn get_global_by_index(index: u16) -> &'static dyn HSGlobal {
    let global = if (index & 0x8000) == 0 {
        match get_scenario_globals().get(index as usize) {
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
    fn global_type(&self) -> ScenarioScriptValueType {
        self.definition.global_type
    }
}

impl HSGlobal for ScenarioGlobal {
    fn name(&'static self) -> &'static str {
        self.name.to_str()
    }
    fn global_type(&self) -> ScenarioScriptValueType {
        self.r#type.get()
    }
}

const HS_ENUMERATE_ADD_RESULT: PointerProvider<unsafe extern "C" fn(what: *const u8)> = pointer_from_hook!("hs_enumerate_add_result");

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

pub const HS_MACRO_FUNCTION_EVALUATE: PointerProvider<unsafe extern "C" fn(u16, u32, u8) -> *const [u8; 0]> = pointer_from_hook!("hs_macro_function_evaluate");
pub const HS_RETURN: PointerProvider<unsafe extern "C" fn(u32, u32) -> *const [u8; 0]> = pointer_from_hook!("hs_return");

