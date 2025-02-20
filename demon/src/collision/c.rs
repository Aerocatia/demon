use c_mine::c_mine;
use tag_structs::ModelCollisionGeometryBSP;
use tag_structs::primitives::vector::Vector3D;
use crate::collision::bsp3d::BSP3DFunctions;

#[c_mine]
pub unsafe extern "C" fn bsp3d_test_point(bsp: &ModelCollisionGeometryBSP, starting_bsp3d_node: usize, point: &Vector3D) -> usize {
    assert_eq!(starting_bsp3d_node, 0, "bsp3d_test_point with nonzero starting bsp3d node");
    bsp.test_point(*point).unwrap_or(0xFFFFFFFF)
}
