use crate::tag::{get_tag_info_typed, ReflexiveImpl, TagID};
use crate::util::VariableProvider;
use tag_structs::primitives::color::ColorARGB;
use tag_structs::{Globals, GlobalsInterfaceBitmaps, HUDGlobals};

pub const GLOBAL_GAME_GLOBALS: VariableProvider<Option<&Globals>> = variable! {
    name: "global_game_globals",
    cache_address: 0x00F1A68C,
    tag_address: 0x00FD1C54
};

pub unsafe fn get_interface_bitmaps() -> &'static GlobalsInterfaceBitmaps {
    let globals = GLOBAL_GAME_GLOBALS.get().expect("get_interface_bitmaps with null globals");
    let interface_bitmaps = globals.interface_bitmaps;
    interface_bitmaps.get(0).expect("no interface bitmaps?!")
}

pub struct InterfaceFonts {
    pub terminal_font: TagID,
    pub full_screen_font: TagID,
    pub split_screen_font: TagID,
    pub hud_text_color: ColorARGB,
    pub hud_icon_color: ColorARGB
}

pub unsafe fn get_interface_fonts() -> InterfaceFonts {
    let interface = get_interface_bitmaps();
    let hud_globals = get_tag_info_typed::<HUDGlobals>(interface.hud_globals.tag_id.into())
        .expect("failed to get hud globals")
        .1;

    InterfaceFonts {
        terminal_font: interface.font_terminal.tag_id.into(),
        full_screen_font: hud_globals.messaging_parameters.fullscreen_font.tag_id.into(),
        split_screen_font: hud_globals.messaging_parameters.splitscreen_font.tag_id.into(),
        hud_text_color: hud_globals.messaging_parameters.text_color,
        hud_icon_color: hud_globals.messaging_parameters.icon_color
    }
}
