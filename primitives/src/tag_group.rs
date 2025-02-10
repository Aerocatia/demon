use core::fmt::{Debug, Display, Formatter};

pub trait TagGroupStruct: Sized {
    fn get_tag_group() -> TagGroup;
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

impl Display for TagGroup {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.name())
    }
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
