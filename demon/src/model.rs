use tag_structs::{GBXModel, Model, ModelMarker};
use tag_structs::primitives::data::Index;
use tag_structs::primitives::string::String32;
use tag_structs::primitives::tag_group::TagGroup;
use crate::tag::{get_tag_info, GetTagDataError, ReflexiveImpl, TagID};

pub mod c;

unsafe impl ModelFunctions for Model {
    unsafe fn markers(&self) -> &[ModelMarker] {
        self.runtime_markers.as_slice()
    }
    unsafe fn get_marker(&self, name: &str) -> Index {
        binary_search_model_marker(self.markers(), name)
    }
}

unsafe impl ModelFunctions for GBXModel {
    unsafe fn markers(&self) -> &[ModelMarker] {
        self.runtime_markers.as_slice()
    }
    unsafe fn get_marker(&self, name: &str) -> Index {
        binary_search_model_marker(self.markers(), name)
    }
}

pub unsafe trait ModelFunctions {
    unsafe fn markers(&self) -> &[ModelMarker];
    unsafe fn get_marker(&self, name: &str) -> Index;
}

pub unsafe fn get_model_tag_data(model_tag: TagID) -> Result<&'static dyn ModelFunctions, GetTagDataError> {
    let tag = get_tag_info(model_tag).ok_or(GetTagDataError::NoMatch { id: model_tag })?;

    if tag.verify_tag_group(TagGroup::Model.into()).is_ok() {
        let data = &*(tag.get_tag_data() as *mut Model);
        return Ok(data);
    }

    if tag.verify_tag_group(TagGroup::GBXModel.into()).is_ok() {
        let data = &*(tag.get_tag_data() as *mut GBXModel);
        return Ok(data);
    }

    Err(GetTagDataError::BadTagGroup {
        id: model_tag,
        tag_group: tag.get_primary_tag_group(),
        expected: [ TagGroup::Model.into(), TagGroup::GBXModel.into(), TagGroup::Null.into(), TagGroup::Null.into() ] }
    )
}

fn binary_search_model_marker(markers: &[ModelMarker], name: &str) -> Index {
    let Some(name) = String32::from_str(name) else {
        return Index::new_none();
    };

    markers
        .binary_search_by(|marker| marker.name.cmp(&name))
        .map(|i| Index::new(i).expect("can't make a model marker index when binary searching"))
        .unwrap_or(Index::new_none())
}
