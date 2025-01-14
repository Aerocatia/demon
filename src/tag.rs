use core::ffi::{c_char, CStr};
use core::fmt::{Debug, Formatter};
use c_mine::c_mine;
use crate::id::ID;
use crate::init::{get_exe_type, ExeType};
use crate::table::DataTable;
use crate::util::VariableProvider;

pub const TAG_ID_SALT: u16 = 0x6174;
pub type TagID = ID<TAG_ID_SALT>;

pub const TAGS_TAG_INSTANCES: VariableProvider<Option<&mut DataTable<TagTagInstance, TAG_ID_SALT>>> = VariableProvider {
    name: "TAGS_TAG_INSTANCES",
    cache_address: 0x00000000 as *mut _,
    tags_address: 0x00FFDAF8 as *mut _
};

pub const CACHE_TAG_INSTANCES: VariableProvider<*mut CacheTagInstance> = VariableProvider {
    name: "CACHE_TAG_INSTANCES",
    cache_address: 0x00AF8364 as *mut _,
    tags_address: 0x00000000 as *mut _
};

pub const CACHE_TAGS_ARE_LOADED: VariableProvider<u8> = VariableProvider {
    name: "CACHE_TAGS_ARE_LOADED",
    cache_address: 0x00AF8368 as *mut _,
    tags_address: 0x00000000 as *mut _
};

pub const CACHE_FILE_TAG_HEADER: VariableProvider<Option<&mut CacheFileTagHeader>> = VariableProvider {
    name: "CACHE_FILE_TAG_HEADER",
    cache_address: 0x00AF8B70 as *mut _,
    tags_address: 0x00000000 as *mut _
};

/// These methods are unsafe as we cannot guarantee yet that the tag data is not being accessed
/// concurrently.
#[repr(C)]
pub struct Reflexive<T: Sized + 'static> {
    count: usize,
    objects: *mut T,
    unknown: u32
}
impl<T: Sized + 'static> Reflexive<T> {
    pub const fn len(&self) -> usize {
        self.count
    }
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub unsafe fn as_slice(&self) -> &[T] {
        if self.is_empty() {
            return &[]
        }
        if self.objects.is_null() {
            panic!("as_slice() -> Bad reflexive: {self:?} @ 0x{:08X}", (self as *const _) as usize);
        }
        core::slice::from_raw_parts(self.objects, self.count)
    }
    pub unsafe fn as_mut_slice(&self) -> &mut [T] {
        if self.is_empty() {
            return &mut []
        }

        if self.objects.is_null() {
            panic!("as_mut_slice() -> Bad reflexive: {self:?} @ 0x{:08X}", (self as *const _) as usize);
        }
        core::slice::from_raw_parts_mut(self.objects, self.count)
    }
    pub unsafe fn get(&self, index: usize) -> Option<&T> {
        self.as_slice().get(index)
    }
    pub unsafe fn get_mut(&self, index: usize) -> Option<&mut T> {
        self.as_mut_slice().get_mut(index)
    }
}
impl<T: Sized> Debug for Reflexive<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(
            format_args!(
                "Reflexive<{type_name}> {{ count={count}, objects=0x{objects:08X} }}",
                type_name=core::any::type_name::<T>(),
                count=self.count,
                objects=self.objects as usize
            ))
    }
}

/// These methods are unsafe as we cannot guarantee yet that the tag data is not being accessed
/// concurrently.
#[derive(Debug)]
#[repr(C)]
pub struct TagData {
    size: usize,
    flags: u32,
    file_offset: u32,
    data: *mut u8,
    unknown: u32,
}
impl TagData {
    pub const fn len(&self) -> usize {
        self.size
    }
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub unsafe fn as_slice(&self) -> &[u8] {
        if self.is_empty() {
            return &[]
        }
        if self.data.is_null() {
            panic!("as_slice() -> Bad data: {self:?} @ 0x{:08X}", (self as *const _) as usize);
        }
        core::slice::from_raw_parts(self.data, self.size)
    }
    pub unsafe fn as_mut_slice(&self) -> &mut [u8] {
        if self.is_empty() {
            return &mut []
        }
        if self.data.is_null() {
            panic!("as_mut_slice() -> Bad data: {self:?} @ 0x{:08X}", (self as *const _) as usize);
        }
        core::slice::from_raw_parts_mut(self.data, self.size)
    }
}

/// Get all cache file tags.
///
/// # Panics
///
/// Panics if not on a cache EXE.
pub fn get_cache_file_tags() -> &'static [CacheTagInstance] {
    // SAFETY: Should be set already.
    unsafe {
        if *CACHE_TAGS_ARE_LOADED.get() == 0 {
            return &[]
        }
        let Some(cache_header) = CACHE_FILE_TAG_HEADER.get() else {
            panic!("CACHE_FILE_TAG_HEADER is null!")
        };
        let tags = *CACHE_TAG_INSTANCES.get();
        assert!(!tags.is_null(), "CACHE_TAGS_ADDRESS is null!");
        core::slice::from_raw_parts(tags, (&*cache_header).tag_count as usize)
    }
}

pub struct CacheFileTagHeader {
    pub tags: *const CacheTagInstance,
    pub scenario_tag: TagID,
    pub checksum: u32,
    pub tag_count: u32
}

pub trait TagIndex {
    fn get_primary_tag_group(&self) -> TagGroupUnsafe;
    fn get_secondary_tag_group(&self) -> TagGroupUnsafe;
    fn get_tertiary_tag_group(&self) -> TagGroupUnsafe;

    /// Attempt to get the tag path.
    ///
    /// # Panics
    ///
    /// Panics if tag_path is null or is not valid UTF-8.
    ///
    /// # Safety
    ///
    /// This is unsafe because tag_path is not verified to be accurate or even pointing to anything.
    unsafe fn get_tag_path(&self) -> &str;
    fn get_tag_data(&self) -> *mut [u8; 0];
}

/// Used only in cache builds.
#[repr(C)]
pub struct CacheTagInstance {
    pub primary_tag_group: TagGroupUnsafe,
    pub secondary_tag_group: TagGroupUnsafe,
    pub tertiary_tag_group: TagGroupUnsafe,
    pub tag_id: TagID,
    pub tag_path: *const c_char,
    pub tag_data: *mut [u8; 0],
    pub external: u32,
    pub padding: u32
}
impl TagIndex for CacheTagInstance {
    fn get_primary_tag_group(&self) -> TagGroupUnsafe {
        self.primary_tag_group
    }

    fn get_secondary_tag_group(&self) -> TagGroupUnsafe {
        self.secondary_tag_group
    }

    fn get_tertiary_tag_group(&self) -> TagGroupUnsafe {
        self.tertiary_tag_group
    }

    unsafe fn get_tag_path(&self) -> &str {
        assert!(!self.tag_path.is_null(), "Tag path is null!");
        CStr::from_ptr(self.tag_path).to_str().expect("Tag path is not UTF-8!")
    }

    fn get_tag_data(&self) -> *mut [u8; 0] {
        self.tag_data
    }
}

/// Used only in tag builds.
#[repr(C)]
pub struct TagTagInstance {
    pub tag_path: [u8; 256],
    pub primary_tag_group: TagGroupUnsafe,
    pub secondary_tag_group: TagGroupUnsafe,
    pub tertiary_tag_group: TagGroupUnsafe,
    /// 0x00000000?
    pub idk1: u32,
    /// 0xFFFFFFFF?
    pub idk2: u32,
    pub crc: u32,
    pub valid: u32,
    pub tag_data: *mut [u8; 0],
    pub tag_definitions: *const u8,
}
impl TagIndex for TagTagInstance {
    fn get_primary_tag_group(&self) -> TagGroupUnsafe {
        self.primary_tag_group
    }

    fn get_secondary_tag_group(&self) -> TagGroupUnsafe {
        self.secondary_tag_group
    }

    fn get_tertiary_tag_group(&self) -> TagGroupUnsafe {
        self.tertiary_tag_group
    }

    unsafe fn get_tag_path(&self) -> &str {
        CStr::from_bytes_until_nul(self.tag_path.as_ref())
            .expect("Tag path is not a null-terminated C string!")
            .to_str()
            .expect("Tag path is not UTF-8!")
    }

    fn get_tag_data(&self) -> *mut [u8; 0] {
        self.tag_data
    }
}

#[c_mine]
pub unsafe extern "C" fn resolve_tag_loaded(group: TagGroupUnsafe, path: *const c_char) -> TagID {
    let path = CStr::from_ptr(path).to_str().expect("input tag is not UTF-8");
    match get_exe_type() {
        ExeType::Tag => {
            let Some(table) = TAGS_TAG_INSTANCES.get_mut() else {
                panic!("TAGS_TAG_INSTANCES is null!");
            };

            let mut iterator = table.iter();
            let Some(_) = (&mut iterator)
                .filter(|tag| tag.item.get_primary_tag_group() == group && tag.item.get_tag_path() == path)
                .next() else {
                return TagID::NULL
            };

            iterator.id()
        },
        ExeType::Cache => get_cache_file_tags()
            .iter()
            .find(|f| f.get_primary_tag_group() == group && f.get_tag_path() == path)
            .map(|t| t.tag_id)
            .unwrap_or(TagID::NULL)
    }
}

#[derive(Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum TagGroup {
    Actor = 0x61637472,
    ActorVariant = 0x61637476,
    Antenna = 0x616E7421,
    Biped = 0x62697064,
    Bitmap = 0x6269746D,
    CameraTrack = 0x7472616B,
    ColorTable = 0x636F6C6F,
    ContinuousDamageEffect = 0x63646D67,
    Contrail = 0x636F6E74,
    DamageEffect = 0x6A707421,
    Decal = 0x64656361,
    DetailObjectCollection = 0x646F6263,
    Device = 0x64657669,
    DeviceControl = 0x6374726C,
    DeviceLightFixture = 0x6C696669,
    DeviceMachine = 0x6D616368,
    Dialogue = 0x75646C67,
    Effect = 0x65666665,
    Equipment = 0x65716970,
    Flag = 0x666C6167,
    Fog = 0x666F6720,
    Font = 0x666F6E74,
    Garbage = 0x67617262,
    GBXModel = 0x6D6F6432,
    Globals = 0x6D617467,
    Glow = 0x676C7721,
    GrenadeHUDInterface = 0x67726869,
    HUDGlobals = 0x68756467,
    HUDMessageText = 0x686D7420,
    HUDNumber = 0x68756423,
    InputDeviceDefaults = 0x64657663,
    Item = 0x6974656D,
    ItemCollection = 0x69746D63,
    LensFlare = 0x6C656E73,
    Light = 0x6C696768,
    LightVolume = 0x6D677332,
    Lightning = 0x656C6563,
    MaterialEffects = 0x666F6F74,
    Meter = 0x6D657472,
    Model = 0x6D6F6465,
    ModelAnimations = 0x616E7472,
    ModelCollisionGeometry = 0x636F6C6C,
    MultiplayerScenarioDescription = 0x6D706C79,
    Object = 0x6F626A65,
    Particle = 0x70617274,
    ParticleSystem = 0x7063746C,
    Physics = 0x70687973,
    Placeholder = 0x706C6163,
    PointPhysics = 0x70706879,
    PreferencesNetworkGame = 0x6E677072,
    Projectile = 0x70726F6A,
    Scenario = 0x73636E72,
    ScenarioStructureBSP = 0x73627370,
    Scenery = 0x7363656E,
    Shader = 0x73686472,
    ShaderEnvironment = 0x73656E76,
    ShaderModel = 0x736F736F,
    ShaderTransparentChicago = 0x73636869,
    ShaderTransparentChicagoExtended = 0x73636578,
    ShaderTransparentGeneric = 0x736F7472,
    ShaderTransparentGlass = 0x73676C61,
    ShaderTransparentMeter = 0x736D6574,
    ShaderTransparentPlasma = 0x73706C61,
    ShaderTransparentWater = 0x73776174,
    Sky = 0x736B7920,
    Sound = 0x736E6421,
    SoundEnvironment = 0x736E6465,
    SoundLooping = 0x6C736E64,
    SoundScenery = 0x73736365,
    Spheroid = 0x626F6F6D,
    StringList = 0x73747223,
    TagCollection = 0x74616763,
    UIWidgetCollection = 0x536F756C,
    UIWidgetDefinition = 0x44654C61,
    UnicodeStringList = 0x75737472,
    Unit = 0x756E6974,
    UnitHUDInterface = 0x756E6869,
    Vehicle = 0x76656869,
    VirtualKeyboard = 0x76636B79,
    Weapon = 0x77656170,
    WeaponHUDInterface = 0x77706869,
    WeatherParticleSystem = 0x7261696E,
    Wind = 0x77696E64,
    Null = 0xFFFFFFFF
}

impl TagGroup {
    pub fn from_unsafe(group: TagGroupUnsafe) -> Option<TagGroup> {
        Self::get_data_from_fourcc(group.0).map(|i| i.0)
    }
    pub fn name(self) -> &'static str {
        Self::get_data_from_fourcc(self as u32)
            .expect("this fourcc should be valid")
            .1
    }
    fn get_data_from_fourcc(fourcc: u32) -> Option<(TagGroup, &'static str)> {
        let fourcc_map = [
            (Self::UIWidgetDefinition, "ui_widget_definition"),
            (Self::UIWidgetCollection, "ui_widget_collection"),
            (Self::Actor, "actor"),
            (Self::ActorVariant, "actor_variant"),
            (Self::Antenna, "antenna"),
            (Self::ModelAnimations, "model_animations"),
            (Self::Biped, "biped"),
            (Self::Bitmap, "bitmap"),
            (Self::Spheroid, "spheroid"),
            (Self::ContinuousDamageEffect, "continuous_damage_effect"),
            (Self::ModelCollisionGeometry, "model_collision_geometry"),
            (Self::ColorTable, "color_table"),
            (Self::Contrail, "contrail"),
            (Self::DeviceControl, "device_control"),
            (Self::Decal, "decal"),
            (Self::InputDeviceDefaults, "input_device_defaults"),
            (Self::Device, "device"),
            (Self::DetailObjectCollection, "detail_object_collection"),
            (Self::Effect, "effect"),
            (Self::Lightning, "lightning"),
            (Self::Equipment, "equipment"),
            (Self::Flag, "flag"),
            (Self::Fog, "fog"),
            (Self::Font, "font"),
            (Self::MaterialEffects, "material_effects"),
            (Self::Garbage, "garbage"),
            (Self::Glow, "glow"),
            (Self::GrenadeHUDInterface, "grenade_hud_interface"),
            (Self::HUDMessageText, "hud_message_text"),
            (Self::HUDNumber, "hud_number"),
            (Self::HUDGlobals, "hud_globals"),
            (Self::Item, "item"),
            (Self::ItemCollection, "item_collection"),
            (Self::DamageEffect, "damage_effect"),
            (Self::LensFlare, "lens_flare"),
            (Self::DeviceLightFixture, "device_light_fixture"),
            (Self::Light, "light"),
            (Self::SoundLooping, "sound_looping"),
            (Self::DeviceMachine, "device_machine"),
            (Self::Globals, "globals"),
            (Self::Meter, "meter"),
            (Self::LightVolume, "light_volume"),
            (Self::GBXModel, "gbxmodel"),
            (Self::Model, "model"),
            (Self::MultiplayerScenarioDescription, "multiplayer_scenario_description"),
            (Self::PreferencesNetworkGame, "preferences_network_game"),
            (Self::Object, "object"),
            (Self::Particle, "particle"),
            (Self::ParticleSystem, "particle_system"),
            (Self::Physics, "physics"),
            (Self::Placeholder, "placeholder"),
            (Self::PointPhysics, "point_physics"),
            (Self::Projectile, "projectile"),
            (Self::WeatherParticleSystem, "weather_particle_system"),
            (Self::ScenarioStructureBSP, "scenario_structure_bsp"),
            (Self::Scenery, "scenery"),
            (Self::ShaderTransparentChicagoExtended, "shader_transparent_chicago_extended"),
            (Self::ShaderTransparentChicago, "shader_transparent_chicago"),
            (Self::Scenario, "scenario"),
            (Self::ShaderEnvironment, "shader_environment"),
            (Self::ShaderTransparentGlass, "shader_transparent_glass"),
            (Self::Shader, "shader"),
            (Self::Sky, "sky"),
            (Self::ShaderTransparentMeter, "shader_transparent_meter"),
            (Self::Sound, "sound"),
            (Self::SoundEnvironment, "sound_environment"),
            (Self::ShaderModel, "shader_model"),
            (Self::ShaderTransparentGeneric, "shader_transparent_generic"),
            (Self::ShaderTransparentPlasma, "shader_transparent_plasma"),
            (Self::SoundScenery, "sound_scenery"),
            (Self::StringList, "string_list"),
            (Self::ShaderTransparentWater, "shader_transparent_water"),
            (Self::TagCollection, "tag_collection"),
            (Self::CameraTrack, "camera_track"),
            (Self::Dialogue, "dialogue"),
            (Self::UnitHUDInterface, "unit_hud_interface"),
            (Self::Unit, "unit"),
            (Self::UnicodeStringList, "unicode_string_list"),
            (Self::VirtualKeyboard, "virtual_keyboard"),
            (Self::Vehicle, "vehicle"),
            (Self::Weapon, "weapon"),
            (Self::Wind, "wind"),
            (Self::WeaponHUDInterface, "weapon_hud_interface"),
            (Self::Null, "null")
        ];
        fourcc_map
            .binary_search_by(|b| (b.0 as u32).cmp(&fourcc))
            .map(|i| fourcc_map[i])
            .ok()
    }
}

impl From<TagGroup> for TagGroupUnsafe {
    fn from(value: TagGroup) -> Self {
        Self(value as u32)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq)]
pub struct TagGroupUnsafe(pub u32);

impl Debug for TagGroupUnsafe {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match TagGroup::from_unsafe(*self) {
            Some(n) => f.write_fmt(format_args!("{}", n.name())),
            None => f.write_fmt(format_args!("TagGroupUnsafe<0x{:08X} (INVALID)>", self.0)),
        }
    }
}

#[derive(Copy, Clone)]
pub enum GetTagDataError {
    NoMatch { id: TagID },
    BadTagGroup { id: TagID, fourcc: TagGroupUnsafe, expected: [TagGroupUnsafe; 4] }
}

impl Debug for GetTagDataError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            GetTagDataError::NoMatch { id } => f.write_fmt(format_args!("Cannot find tag with ID {id:?}")),
            GetTagDataError::BadTagGroup { id, fourcc: group, expected } => f.write_fmt(format_args!("Found a tag with {id:?}, but the tag group is incorrect: '{group:?}' not in {expected:?}")),
        }
    }
}

pub unsafe fn get_tag_info(id: TagID) -> Option<&'static dyn TagIndex> {
    match get_exe_type() {
        ExeType::Cache => {
            let index = id.index()?;
            let tags = get_cache_file_tags();
            let result = tags.get(index)?;
            Some(result)
        },
        ExeType::Tag => {
            let Some(table) = TAGS_TAG_INSTANCES.get_mut() else {
                panic!("TAGS_TAG_INSTANCES is null!");
            };
            let tag = table.get_element(id).ok()?;
            Some(&tag.item)
        }
    }

}

/// Gets the tag data.
pub unsafe fn get_tag_data_checking_tag_group(group: TagGroupUnsafe, id: TagID) -> Result<*mut [u8; 0], GetTagDataError> {
    let tag = get_tag_info(id).ok_or(GetTagDataError::NoMatch { id })?;

    let expected = [
        tag.get_primary_tag_group(),
        tag.get_secondary_tag_group(),
        tag.get_tertiary_tag_group(),
        TagGroupUnsafe(u32::MAX),
    ];

    if !expected.contains(&group) {
        return Err(GetTagDataError::BadTagGroup { id, fourcc: group, expected });
    }

    Ok(tag.get_tag_data())
}

#[c_mine]
pub unsafe extern "C" fn tag_get(group: TagGroupUnsafe, id: TagID) -> *mut [u8; 0] {
    get_tag_data_checking_tag_group(group, id).expect("tag_get failed!")
}

#[c_mine]
pub unsafe extern "C" fn tag_block_get_address(reflexive: Option<&Reflexive<[u8; 0]>>) -> *mut [u8; 0] {
    reflexive.expect("tag_block_get_address with null reflexive").objects
}

#[c_mine]
pub unsafe extern "C" fn tag_block_get_element_with_size(
    reflexive: Option<&Reflexive<[u8; 0]>>,
    index: usize,
    element_size: usize
) -> *mut [u8; 0] {
    let reflexive = reflexive.expect("tag_block_get_element_with_size with null reflexive");
    assert!(
        index < reflexive.len(),
        "tag_block_get_element_with_size with out-of-bounds index {index} < {} @ 0x{:08X}",
        reflexive.len(),
        (reflexive as *const _) as usize
    );

    let offset = index.checked_mul(element_size)
        .and_then(|v| isize::try_from(v).ok())
        .expect("tag_block_get_element_with_size with invalid offset/element size");

    reflexive
        .objects
        .wrapping_byte_offset(offset)
}
