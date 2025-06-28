use tag_structs::ModelCollisionGeometryBSP;
use crate::util::VariableProvider;

pub mod c;
pub mod bsp3d;

const GLOBAL_BSP3D: VariableProvider<*const ModelCollisionGeometryBSP> = variable! {
    name: "global_bsp3d",
    cache_address: 0x00F1A688,
    tag_address: 0x00FD1C50
};
