#![no_std]
pub extern crate primitives;

use num_enum::TryFromPrimitive;
use tag_structs_gen::tag_definitions;
use primitives::NamedTagStruct;
use primitives::tag_group::TagGroupStruct;

tag_definitions!();
