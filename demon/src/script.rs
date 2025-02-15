pub mod print;
pub mod c;
pub mod crash;
pub mod cls;

use core::ffi::CStr;
use c_mine::pointer_from_hook;
use tag_structs::{ScenarioGlobal, ScenarioScriptValueType};
use crate::init::{get_exe_type, ExeType};
use crate::memory::table::DataTable;
use crate::tag::{ReflexiveImpl, GLOBAL_SCENARIO_INDEX};
use crate::tag::c::global_scenario_get;
use crate::util::{PointerProvider, VariableProvider};

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
