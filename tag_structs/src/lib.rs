#![no_std]
pub extern crate primitives;

use num_enum::TryFromPrimitive;
use tag_structs_gen::tag_definitions;
use primitives::named_tag_struct::NamedTagStruct;
use primitives::tag_group::TagGroupStruct;

tag_definitions!();
