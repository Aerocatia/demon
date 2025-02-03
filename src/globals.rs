use crate::tag::{get_tag_data_checking_tag_group, Reflexive, TagGroup, TagID, TagReference};
use crate::util::VariableProvider;

pub const GLOBAL_GAME_GLOBALS: VariableProvider<*const [u8; 0]> = variable! {
    name: "global_game_globals",
    cache_address: 0x00F1A68C,
    tag_address: 0x00FD1C54
};

pub struct InterfaceBitmaps {
    pub font_system: TagReference,
    pub font_terminal: TagReference,
    pub screen_color_table: TagReference,
    pub hud_color_table: TagReference,
    pub editor_color_table: TagReference,
    pub dialog_color_table: TagReference,
    pub hud_globals: TagReference,
    pub motion_sensor_sweep_bitmap: TagReference,
    pub motion_sensor_sweep_bitmap_mask: TagReference,
    pub multiplayer_hud_bitmap: TagReference,
    pub localization: TagReference,
    pub hud_digits_definition: TagReference,
    pub motion_sensor_blip_bitmap: TagReference,
    pub interface_goo_map1: TagReference,
    pub interface_goo_map2: TagReference,
    pub interface_goo_map3: TagReference,
    pub _padding: [u8; 48]
}

// TODO: de-hardcode this, use definitions
pub unsafe fn get_interface_bitmaps() -> &'static InterfaceBitmaps {
    let globals = GLOBAL_GAME_GLOBALS.get();
    assert!(!globals.is_null(), "get_interface_bitmaps with null globals");
    let interface_bitmaps_reflexive = globals
        .wrapping_byte_add(0x140) as *const Reflexive<InterfaceBitmaps>;
    let interface_bitmaps_reflexive = &*interface_bitmaps_reflexive;
    interface_bitmaps_reflexive.get(0).expect("no interface bitmaps?!")
}

pub struct InterfaceFonts {
    pub terminal_font: TagID,
    pub full_screen_font: TagID,
    pub split_screen_font: TagID
}

pub unsafe fn get_interface_fonts() -> InterfaceFonts {
    let interface = get_interface_bitmaps();
    let hud_globals = get_tag_data_checking_tag_group(interface.hud_globals.tag_id, TagGroup::HUDGlobals.into()).expect("failed to get hud globals");

    let full_screen = hud_globals.wrapping_byte_add(0x48) as *const TagReference;
    let split_screen = hud_globals.wrapping_byte_add(0x58) as *const TagReference;

    InterfaceFonts {
        terminal_font: interface.font_terminal.tag_id,
        full_screen_font: (*full_screen).tag_id,
        split_screen_font: (*split_screen).tag_id,
    }
}
