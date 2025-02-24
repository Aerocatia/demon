use tag_structs::{GBXModel, Model, ModelMarker, ModelNode};
use tag_structs::primitives::tag_group::TagGroup;
use crate::tag::{get_tag_info, GetTagDataError, ReflexiveImpl, TagID, TagIndex};
use crate::util::compare_ascii_case_insensitive;

pub mod c;

unsafe impl ModelFunctions for Model {
    unsafe fn get_markers(&self) -> &[ModelMarker] {
        self.runtime_markers.as_slice()
    }
    unsafe fn get_nodes(&self) -> &[ModelNode] {
        self.nodes.as_slice()
    }
}

unsafe impl ModelFunctions for GBXModel {
    unsafe fn get_markers(&self) -> &[ModelMarker] {
        self.runtime_markers.as_slice()
    }
    unsafe fn get_nodes(&self) -> &[ModelNode] {
        self.nodes.as_slice()
    }
}

pub unsafe trait ModelFunctions {
    unsafe fn get_markers(&self) -> &[ModelMarker];
    unsafe fn get_nodes(&self) -> &[ModelNode];

    unsafe fn get_marker(&self, name: &str) -> Option<&ModelMarker> {
        Some(&self.get_markers()[self.get_marker_index(name)?])
    }
    unsafe fn get_marker_index(&self, name: &str) -> Option<usize> {
        binary_search_model_marker(self.get_markers(), name)
    }
    unsafe fn get_node(&self, name: &str) -> Option<&ModelNode> {
        Some(&self.get_nodes()[self.get_node_index(name)?])
    }
    unsafe fn get_node_index(&self, name: &str) -> Option<usize> {
        self.get_nodes().iter().position(|n| n.name.string_bytes() == name.as_bytes())
    }
}

pub unsafe fn get_model_tag_data(model_tag: TagID) -> Result<(&'static dyn TagIndex, &'static dyn ModelFunctions), GetTagDataError> {
    let tag = get_tag_info(model_tag).ok_or(GetTagDataError::NoMatch { id: model_tag })?;

    if tag.verify_tag_group(TagGroup::Model.into()).is_ok() {
        let data = &*(tag.get_tag_data_address() as *mut Model);
        return Ok((tag, data));
    }

    if tag.verify_tag_group(TagGroup::GBXModel.into()).is_ok() {
        let data = &*(tag.get_tag_data_address() as *mut GBXModel);
        return Ok((tag, data));
    }

    Err(GetTagDataError::BadTagGroup {
        id: model_tag,
        tag_group: tag.get_primary_tag_group(),
        expected: [ TagGroup::Model.into(), TagGroup::GBXModel.into(), TagGroup::Null.into(), TagGroup::Null.into() ] }
    )
}

fn binary_search_model_marker(markers: &[ModelMarker], name: &str) -> Option<usize> {
    let name_bytes = name.as_bytes();

    // TODO: we are doing tons of case insensitive string comparisons every tick/frame; we should
    //       cache these in a table on scenario load (e.g. a table that maps unit seat camera marker
    //       to marker indices) for better performance
    markers
        .binary_search_by(|marker| compare_ascii_case_insensitive(marker.name.string_bytes(), name_bytes))
        .ok()
}
