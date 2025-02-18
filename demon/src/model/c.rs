use crate::model::get_model_tag_data;
use crate::tag::{ReflexiveImpl, TagID};
use crate::util::CStrPtr;
use c_mine::c_mine;
use tag_structs::primitives::data::Index;
use tag_structs::primitives::vector::Matrix4x3;
use crate::object::ObjectMarker;

#[c_mine]
pub unsafe extern "C" fn model_find_marker(model_tag: TagID, name: CStrPtr) -> usize {
    if name.is_null() || model_tag.is_null() {
        return usize::MAX;
    }

    get_model_tag_data(model_tag)
        .ok()
        .and_then(|m| m.get_marker_index(name.as_str()))
        .and_then(|m| Index::new(m).ok())
        .unwrap_or(Index::new_none())
        .get()
        .unwrap_or(0xFFFFFFFF)
}

/// # Safety
///
/// I... don't even know where to begin with this one.
#[c_mine]
pub unsafe extern "C" fn model_get_marker_by_name(
    model_tag: TagID,
    name: CStrPtr,
    regions: *const u8,
    node_remapping_table: *const u16,
    node_remapping_table_length: u16,
    object_nodes: *const Matrix4x3,
    flip: u8,
    object_markers: *mut ObjectMarker,
    object_markers_count: u16
) -> u16 {
    assert!(!object_nodes.is_null(), "model_get_marker_by_name with null object nodes");
    assert!(!object_markers.is_null(), "model_get_marker_by_name with null markers");

    let Ok(model) = get_model_tag_data(model_tag) else {
        return 0
    };

    let Some(model_marker) = model.get_marker(name.as_str()) else {
        return 0
    };

    let object_nodes = {
        let actual_node_count = if node_remapping_table.is_null() {
            model.get_nodes().len()
        }
        else {
            node_remapping_table_length as usize
        };
        core::slice::from_raw_parts(object_nodes, actual_node_count)
    };

    let a = |n: u8| n as u16;
    let b = |n: u8| *node_remapping_table.add(n as usize);

    let get_node_index: &dyn Fn(u8) -> u16 = if node_remapping_table.is_null() {
        &a
    } else {
        &b
    };

    let object_markers = core::slice::from_raw_parts_mut(object_markers, object_markers_count as usize);

    let marker_count = model_marker
        .instances
        .as_slice()
        .iter()
        .filter(|marker_instance| {
            regions.is_null() || (*regions.wrapping_add(marker_instance.region_index as usize) == marker_instance.permutation_index)
        })
        .zip(object_markers.iter_mut())
        .map(|(marker_instance, object_marker)| {
            let node_index = get_node_index(marker_instance.node_index);
            let node = object_nodes[node_index as usize];
            *object_marker = ObjectMarker::new(
                node_index,
                &node,
                marker_instance,
                flip != 0
            );
        })
        .count();

    marker_count as u16
}
