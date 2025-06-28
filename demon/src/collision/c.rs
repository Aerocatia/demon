use crate::collision::bsp3d::{get_global_collision_bsp, ModelCollisionGeometryBSPImpl};
use c_mine::c_mine;
use tag_structs::primitives::collision_bsp::CollisionBSPFunctions;
use tag_structs::primitives::vector::Vector3D;
use tag_structs::ModelCollisionGeometryBSP;

#[c_mine]
pub unsafe extern "C" fn bsp3d_test_point(bsp: &ModelCollisionGeometryBSP, starting_bsp3d_node: usize, point: &Vector3D) -> usize {
    assert_eq!(starting_bsp3d_node, 0, "bsp3d_test_point with nonzero starting bsp3d node");
    ModelCollisionGeometryBSPImpl::wrap(bsp)
        .leaf_index_for_point(point)
        .expect("bsp3d_test_point with invalid bsp")
        .unwrap_or(usize::MAX)
}

#[c_mine]
pub unsafe extern "C" fn scenario_leaf_index_from_point(point: &Vector3D) -> usize {
    get_global_collision_bsp()
        .leaf_index_for_point(point)
        .expect("scenario_leaf_index_from_point with invalid bsp")
        .unwrap_or(usize::MAX)
}

#[c_mine]
pub unsafe extern "C" fn global_bsp3d() -> *const ModelCollisionGeometryBSP {
    *super::GLOBAL_BSP3D.get()
}
