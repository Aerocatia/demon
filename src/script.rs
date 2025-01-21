use core::ffi::{c_char, CStr};
use core::ptr::null_mut;
use c_mine::c_mine;
use crate::init::{get_exe_type, ExeType};
use crate::tag::{global_scenario_get, Reflexive, String32, GLOBAL_SCENARIO_INDEX};
use crate::util::PointerProvider;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HSExternalGlobalDefinition {
    name: *const u8,
    global_type: u32,
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
                global_type: global_type as u32,
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
    pub global_type: u16,
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

const HS_ENUMERATE_ADD_RESULT: PointerProvider<unsafe extern "C" fn(what: *const u8)> = pointer! {
    name: "hs_enumerate_add_result",
    cache_address: 0x005F6A10,
    tags_address: 0x005FD350
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
