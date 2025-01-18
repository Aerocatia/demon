use crate::id::ID;
use crate::tag::TagID;
use crate::util::PointerProvider;

pub unsafe fn play_looping_sound(tag_id: TagID, object_id: ID<0x626F>, gain: f32) {
    const SCRIPT_LOOPING_SOUND_START: PointerProvider<unsafe extern "C" fn(TagID, ID<0x626F>, f32)> = pointer! {
        name: "SCRIPT_LOOPING_SOUND_START",
        cache_address: 0x0040912E,
        tags_address: 0x0040844F
    };
    SCRIPT_LOOPING_SOUND_START.get()(tag_id, object_id, gain);
}

pub unsafe fn stop_looping_sound(tag_id: TagID) {
    const SCRIPT_LOOPING_SOUND_STOP: PointerProvider<unsafe extern "C" fn(TagID)> = pointer! {
        name: "SCRIPT_LOOPING_SOUND_STOP",
        cache_address: 0x004025DB,
        tags_address: 0x0040AD30
    };
    SCRIPT_LOOPING_SOUND_STOP.get()(tag_id);
}
