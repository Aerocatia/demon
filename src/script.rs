use core::ffi::{c_char, CStr};
use core::mem::transmute;
use core::ptr::null_mut;
use c_mine::c_mine;
use crate::init::{get_exe_type, ExeType};
use crate::memory::table::{data_make_valid, game_state_data_new, DataTable};
use crate::tag::{global_scenario_get, Reflexive, String32, GLOBAL_SCENARIO_INDEX};
use crate::util::{PointerProvider, VariableProvider};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HSExternalGlobalDefinition {
    name: *const u8,
    global_type: ScriptValueType,
    padding: u16,
    ptr: *mut [u8; 0],
    unknown: u32
}

pub struct ExternalGlobal {
    name_buffer: &'static [u8],
    definition: HSExternalGlobalDefinition
}
impl ExternalGlobal {
    pub const fn new(name: &'static [u8], global_type: ScriptValueType, address: *mut [u8; 0]) -> Self {
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

#[derive(Copy, Clone, Debug)]
#[repr(u16)]
pub enum ScriptValueType {
    Unparsed,
    SpecialForm,
    FunctionName,
    Passthrough,
    Void,
    Boolean,
    Real,
    Short,
    Long,
    String,
    Script,
    TriggerVolume,
    CutsceneFlag,
    CutsceneCameraPoint,
    CutsceneTitle,
    CutsceneRecording,
    DeviceGroup,
    Ai,
    AiCommandList,
    StartingProfile,
    Conversation,
    Navpoint,
    HudMessage,
    ObjectList,
    Sound,
    Effect,
    Damage,
    LoopingSound,
    AnimationGraph,
    ActorVariant,
    DamageEffect,
    ObjectDefinition,
    GameDifficulty,
    Team,
    AiDefaultState,
    ActorType,
    HudCorner,
    Object,
    Unit,
    Vehicle,
    Weapon,
    Device,
    Scenery,
    ObjectName,
    UnitName,
    VehicleName,
    WeaponName,
    DeviceName,
    SceneryName,
}


#[c_mine]
pub unsafe extern "C" fn main_crash(param: *const c_char) {
    for i in get_external_globals().iter().take(1) {
        console!("{}", i.name());
    }
    let message = CStr::from_ptr(param).to_string_lossy();
    panic!("crash command called with message:\n\n{}", message.as_ref());
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

// TODO: use definitions
#[repr(C)]
pub struct ScenarioGlobal {
    pub name: String32,
    pub global_type: ScriptValueType,
    pub padding: u16,
    pub more_padding: u32,
    pub initialization_expression_index: u32,
    pub even_more_padding: [u8; 48]
}

pub unsafe fn get_scenario_globals() -> &'static Reflexive<ScenarioGlobal> {
    if GLOBAL_SCENARIO_INDEX.get().is_null() {
        return &Reflexive { count: 0, objects: null_mut(), unknown: 0 };
    }

    let scenario = global_scenario_get.get()().wrapping_byte_add(0x4A8) as *const Reflexive<ScenarioGlobal>;
    &*scenario
}

trait HSGlobal {
    /// Name (not null terminated)
    fn name(&'static self) -> &'static str;

    /// Null terminated
    fn name_bytes(&'static self) -> &'static [u8];

    /// Type of global
    fn global_type(&self) -> ScriptValueType;
}

#[c_mine]
pub unsafe extern "C" fn hs_find_global_by_name(name: *const c_char) -> u32 {
    let Ok(name) = CStr::from_ptr(name).to_str() else {
        return u32::MAX
    };

    const MAX_GLOBALS: usize = 0x8000;

    let globals = get_external_globals();
    for (index, global) in globals.iter().take(MAX_GLOBALS).enumerate() {
        if global.name() == name {
            return ((index & 0x7FFF) | 0x8000) as u32
        }
    }

    for (index, global) in get_scenario_globals().as_slice().iter().take(MAX_GLOBALS).enumerate() {
        if global.name.as_str() == name {
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
    fn global_type(&self) -> ScriptValueType {
        self.definition.global_type
    }
}

impl HSGlobal for ScenarioGlobal {
    fn name(&'static self) -> &'static str {
        self.name.as_str()
    }
    fn name_bytes(&'static self) -> &'static [u8] {
        self.name.data.as_slice()
    }
    fn global_type(&self) -> ScriptValueType {
        self.global_type
    }
}

const HS_ENUMERATE_ADD_RESULT: PointerProvider<unsafe extern "C" fn(what: *const u8)> = pointer! {
    name: "hs_enumerate_add_result",
    cache_address: 0x005F6A10,
    tag_address: 0x005FD350
};

#[c_mine]
pub unsafe extern "C" fn hs_enumerate_globals() {
    for i in get_external_globals() {
        if i.definition.ptr.is_null() {
            continue
        }
        HS_ENUMERATE_ADD_RESULT.get()(i.name_buffer.as_ptr())
    }
    for i in get_scenario_globals().as_slice() {
        HS_ENUMERATE_ADD_RESULT.get()(i.name.as_str().as_ptr())
    }
}

#[c_mine]
pub unsafe extern "C" fn hs_global_get_name(index: u16) -> *const u8 {
    get_global_by_index(index).name_bytes().as_ptr()
}

#[c_mine]
pub unsafe extern "C" fn hs_global_get_type(index: u16) -> ScriptValueType {
    get_global_by_index(index).global_type()
}

const HS_THREAD_TABLE: VariableProvider<Option<&mut DataTable<[u8; 0xFC], 0x7368>>> = variable! {
    name: "hs_thread_table",
    cache_address: 0x00C7DE48,
    tag_address: 0x00D35400
};

const HS_GLOBALS_TABLE: VariableProvider<Option<&mut DataTable<usize, 0x7368>>> = variable! {
    name: "hs_globals_table",
    cache_address: 0x00C7DE4C,
    tag_address: 0x00D35404
};

const DATUM_NEW_AT_INDEX: PointerProvider<unsafe extern "C" fn(table: *mut DataTable<[u8; 0], 0>, index: u32) -> u32> = pointer! {
    name: "datum_new_at_index",
    cache_address: 0x00406B86,
    tag_address: 0x00405bAA
};

const HS_EXTERNAL_GLOBALS_COUNT: VariableProvider<u16> = variable! {
    name: "HS_EXTERNAL_GLOBALS_COUNT",
    cache_address: 0x0097E8CC,
    tag_address: 0x009E2D94
};

#[c_mine]
pub unsafe extern "C" fn hs_runtime_initialize() {
    let hs_thread_table = game_state_data_new.get()(
        b"hs thread\x00".as_ptr() as *const c_char,
        0x100,
        0x218
    );
    if hs_thread_table.is_null() {
        panic!("hs_runtime_initialize failed to allocate hs_thread_table")
    }
    let hs_globals_table = game_state_data_new.get()(
        b"hs globals\x00".as_ptr() as *const c_char,
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
            .get_mut()
            .as_mut()
            .unwrap()
            .get_element(transmute(actual_index))
            .map(|g| g.item as u32) else {
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
