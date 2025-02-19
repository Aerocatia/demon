use core::ffi::c_char;
use c_mine::c_mine;
use tag_structs::primitives::data::{Data, Index, Reflexive};
use tag_structs::{Biped, BipedFlagsFields, ModelAnimations, ModelAnimationsAnimationGraphNodeFlagsFields, Scenario};
use tag_structs::primitives::float::FloatFunctions;
use crate::model::get_model_tag_data;
use crate::tag::{get_tag_data, get_tag_data_checking_tag_group, get_tag_info, lookup_tag, ReflexiveImpl, TagData, TagGroupUnsafe, TagID, UnknownType, GLOBAL_SCENARIO};
use crate::timing::TICK_RATE;
use crate::util::CStrPtr;

#[c_mine]
pub unsafe extern "C" fn tag_get(group: TagGroupUnsafe, id: TagID) -> *mut [u8; 0] {
    get_tag_data_checking_tag_group(id, group).expect("tag_get failed!")
}

#[c_mine]
pub unsafe extern "C" fn tag_block_get_address(reflexive: Option<&Reflexive<UnknownType>>) -> *mut [u8; 0] {
    reflexive.expect("tag_block_get_address with null reflexive").address.0 as *mut _
}

#[c_mine]
pub unsafe extern "C" fn tag_block_get_element_with_size(
    reflexive: Option<&Reflexive<UnknownType>>,
    index: usize,
    element_size: usize
) -> *mut UnknownType {
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

    let objects = reflexive.address.0 as *mut UnknownType;
    objects.wrapping_byte_offset(offset)
}

#[c_mine]
pub unsafe extern "C" fn global_scenario_get() -> &'static mut Scenario {
    GLOBAL_SCENARIO
        .get_copied()
        .expect("global_scenario_get(): global_scenario is null!")
}

#[c_mine]
pub unsafe extern "C" fn tag_loaded(group: TagGroupUnsafe, path: CStrPtr) -> TagID {
    lookup_tag(path.as_str(), group).map(|t| t.1).unwrap_or(TagID::NULL)
}

#[c_mine]
pub unsafe extern "C" fn tag_name_strip_path(path: CStrPtr) -> CStrPtr {
    path.as_str()
        .as_bytes()
        .iter()
        .rev()
        .find(|b| **b == b'\\')
        .map(|b| (b as *const u8).wrapping_byte_add(1) as *const c_char)
        .map(CStrPtr)
        .unwrap_or(path)
}

#[c_mine]
pub unsafe extern "C" fn tag_get_name(tag_id: TagID) -> CStrPtr {
    let path = get_tag_info(tag_id)
        .expect("failed to get the tag name")
        .get_tag_path()
        .as_ptr() as *const c_char;
    CStrPtr(path)
}

#[c_mine]
pub unsafe extern "C" fn get_data_address(data: &mut Data) -> *mut u8 {
    data.as_mut_slice().as_mut_ptr()
}

#[c_mine]
pub unsafe extern "C" fn preprocess_biped(tag_id: TagID, unknown: u8) -> bool {
    let unknown = unknown != 0;
    let mut success = false;

    let biped = get_tag_data::<Biped>(tag_id).expect("can't find the biped we just passed in?");

    let crouch_camera_ticks = biped.crouch_transition_time * TICK_RATE;
    biped.crouch_camera_velocity = if crouch_camera_ticks > 0.0 { 1.0 / crouch_camera_ticks } else { 1.0 };
    biped.cosine_stationary_turning_threshold = biped.stationary_turning_threshold.0.cos();
    biped.cosine_maximum_slope_angle = biped.maximum_slope_angle.0.cos();
    biped.negative_sine_downhill_falloff_angle = -biped.downhill_falloff_angle.0.sin();
    biped.negative_sine_downhill_cutoff_angle = -biped.downhill_cutoff_angle.0.sin();
    biped.sine_uphill_falloff_angle = biped.uphill_falloff_angle.0.sin();
    biped.sine_uphill_cutoff_angle = biped.uphill_cutoff_angle.0.sin();

    let Ok(model) = get_model_tag_data(biped.unit.object.model.tag_id.into()) else {
        panic!("Biped {} does not have a model reference.", get_tag_info(tag_id).unwrap().get_tag_path());
    };

    let Ok(animation) = get_tag_data::<ModelAnimations>(biped.unit.object.animation_graph.tag_id.into()) else {
        panic!("Biped {} does not have an animation tag reference.", get_tag_info(tag_id).unwrap().get_tag_path());
    };

    let find_node = |node: &str| -> Index {
        model.get_node_index(node)
            .and_then(|i| Index::new(i).ok())
            .unwrap_or(Index::new_none())
    };

    biped.pelvis_model_node_index = find_node("bip01 pelvis");
    biped.head_model_node_index = find_node("bip01 head");

    if model.get_marker("body").is_none() {
        panic!("Biped {} does not have a \"body\" marker.", get_tag_info(tag_id).unwrap().get_tag_path());
    }

    if model.get_marker("head").is_none() {
        panic!("Biped {} does not have a \"head\" marker.", get_tag_info(tag_id).unwrap().get_tag_path());
    }

    let mut flags = biped.flags;
    if !unknown && flags.is_set(BipedFlagsFields::UsesLimpBodyPhysics) {
        for node in model.get_nodes().iter().skip(1) {
            let magnitude = node.default_translation.magnitude();
            let node_distance_from_parent = node.node_distance_from_parent;
            let difference = (node.node_distance_from_parent - magnitude).fabs();
            if difference >= 0.0001 {
                flags.unset(BipedFlagsFields::UsesLimpBodyPhysics);
                error!("Biped {}'s model nodes cannot use limp body physics. Limp body physics have been disabled.", get_tag_info(tag_id).unwrap().get_tag_path());
                break;
            }
        }
        for node in animation.nodes.as_slice().iter().skip(1) {
            if node.node_joint_flags.is_set(ModelAnimationsAnimationGraphNodeFlagsFields::NoMovement) {
                continue
            }
            let magnitude = node.base_vector.magnitude();
            if magnitude.fabs() < 0.0001 {
                flags.unset(BipedFlagsFields::UsesLimpBodyPhysics);
                error!("Biped {}'s animation nodes cannot use limp body physics. Limp body physics have been disabled.", get_tag_info(tag_id).unwrap().get_tag_path());
                break;
            }
        }
        success = success && flags.is_set(BipedFlagsFields::UsesLimpBodyPhysics);
        biped.flags = flags;
    }

    success
}
