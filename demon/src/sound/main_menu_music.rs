use spin::Mutex;
use c_mine::c_mine;
use tag_structs::primitives::tag_group::TagGroup;
use crate::id::ID;
use crate::sound::sound_looping::{play_looping_sound, stop_looping_sound};
use crate::tag::lookup_tag;
use crate::util::VariableProvider;

static MAIN_MENU_MUSIC_ACTIVE: Mutex<bool> = Mutex::new(false);

#[c_mine]
pub unsafe extern "C" fn ui_start_main_menu_music() {
    let mut active = MAIN_MENU_MUSIC_ACTIVE.lock();
    if *active || main_menu_fade_active() {
        return
    }

    let Some((_, id)) = lookup_tag(MAIN_MENU_MUSIC_PATH, MAIN_MENU_TAG_GROUP.into()) else {
        error!("No main menu music found (failed to find {MAIN_MENU_MUSIC_PATH}.{MAIN_MENU_TAG_GROUP})");
        return
    };

    play_looping_sound(id, ID::NULL, 1.0);
    *active = true;
}

#[c_mine]
pub unsafe extern "C" fn ui_stop_main_menu_music() {
    let mut active = MAIN_MENU_MUSIC_ACTIVE.lock();
    if !*active {
        return;
    }

    if let Some((_, id)) = lookup_tag(MAIN_MENU_MUSIC_PATH, MAIN_MENU_TAG_GROUP.into()) {
        stop_looping_sound(id);
    };

    *active = false;
}

#[c_mine]
pub extern "C" fn ui_main_menu_music_active() -> bool {
    *MAIN_MENU_MUSIC_ACTIVE.lock()
}

const MAIN_MENU_MUSIC_PATH: &str = "sound\\music\\title1\\title1";
const MAIN_MENU_TAG_GROUP: TagGroup = TagGroup::SoundLooping;

// ???
pub const MAIN_MENU_FADE_ACTIVE: VariableProvider<u32> = variable! {
    name: "MAIN_MENU_FADE_ACTIVE",
    cache_address: 0x00C99718,
    tag_address: 0x00D50CF0
};

fn main_menu_fade_active() -> bool {
    *unsafe { MAIN_MENU_FADE_ACTIVE.get() } != 0u32
}
