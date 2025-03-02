#![no_std]
pub extern crate primitives;

use num_enum::TryFromPrimitive;
use tag_structs_gen::tag_definitions;
use primitives::named_tag_struct::NamedTagStruct;
use primitives::rectangle::Rectangle;
use primitives::tag_group::TagGroupStruct;

tag_definitions!();

impl UICanvas {
    pub const fn get_bounds(self) -> Rectangle {
        match self {
            UICanvas::_640x480  => Rectangle::from_width_and_height(640, 480),
            UICanvas::_854x480  => Rectangle::from_width_and_height(854, 480),
            UICanvas::_1280x960 => Rectangle::from_width_and_height(1280, 960),
            UICanvas::_1708x960 => Rectangle::from_width_and_height(1708, 960),
        }
    }
}
