use c_mine::c_mine;
use tag_structs::{ObjectType, Weapon};
use tag_structs::primitives::float::FloatFunctions;
use tag_structs::primitives::vector::Vector3D;
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

#[c_mine]
pub unsafe extern "C" fn projectile_aim_linear(
    param_1: f32,
    param_2: &Vector3D,
    param_3: &Vector3D,
    result_aim_vector: &mut Vector3D,
    param_5: Option<&mut f32>,
    param_6: Option<&mut f32>,
    param_7: Option<&mut f32>
) {
    let vector = *param_3 - *param_2;
    let magnitude = vector.magnitude();
    *result_aim_vector = vector.normalized().unwrap_or(vector);

    // ???
    if let Some(param_5) = param_5 {
        *param_5 = param_1;
    }

    if let Some(param_6) = param_6 {
        *param_6 = if param_1 <= 0.0 { 0.0 } else { magnitude / param_1 };
    }

    if let Some(param_7) = param_7 {
        *param_7 = magnitude;
    }
}
