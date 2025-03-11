use c_mine::c_mine;
use tag_structs::{ObjectType, Weapon};
use tag_structs::primitives::float::FloatFunctions;
use crate::object::c::object_get_and_verify_type;
use crate::object::ObjectID;
use crate::tag::{get_tag_info_typed, TagID};

#[c_mine]
pub unsafe extern "C" fn weapon_get_zoom_magnification(object: ObjectID, zoom_level: u16) -> f32 {
    if zoom_level == u16::MAX {
        return 1.0
    }

    let object = object_get_and_verify_type.get()(object, ObjectType::Weapon.into()) as *mut TagID;
    let (tag_index, weapon) = get_tag_info_typed::<Weapon>(*object).expect("weapon_get_zoom_magnification on a non-weapon tag");

    let max_zoom_levels = weapon.zoom_levels;
    if !(0..max_zoom_levels).contains(&zoom_level) {
        panic!("weapon_get_zoom_magnification: {zoom_level} is out-of-bounds for the weapon {tag_index}");
    }

    let ratio = if max_zoom_levels == 1 || zoom_level >= max_zoom_levels {
        0.0
    }
    else {
        (zoom_level as f32) / ((max_zoom_levels - 1) as f32)
    };

    let max_zoom_magnification = weapon.zoom_magnification_range;
    let low = max_zoom_magnification.lower_bound.max(1.0);
    let high = max_zoom_magnification.upper_bound.max(1.0);

    (high / low).powf(ratio) * low
}
