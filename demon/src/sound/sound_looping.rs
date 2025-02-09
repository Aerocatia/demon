use c_mine::pointer_from_hook;
use crate::object::ObjectID;
use crate::tag::TagID;
use crate::util::PointerProvider;

pub unsafe fn play_looping_sound(tag_id: TagID, object_id: ObjectID, gain: f32) {
    const SCRIPT_LOOPING_SOUND_START: PointerProvider<unsafe extern "C" fn(TagID, ObjectID, f32)> = pointer_from_hook!("script_looping_sound_start");
    SCRIPT_LOOPING_SOUND_START.get()(tag_id, object_id, gain);
}

pub unsafe fn stop_looping_sound(tag_id: TagID) {
    const SCRIPT_LOOPING_SOUND_STOP: PointerProvider<unsafe extern "C" fn(TagID)> = pointer_from_hook!("script_looping_sound_stop");
    SCRIPT_LOOPING_SOUND_STOP.get()(tag_id);
}
