use tag_structs::ModelCollisionGeometryBSP;
use tag_structs::primitives::vector::Vector3D;
use crate::tag::ReflexiveImpl;

pub unsafe trait BSP3DFunctions {
    /// Get the leaf the point resides in.
    ///
    /// Returns `None` if the point is outside of the BSP.
    unsafe fn test_point(&self, point: Vector3D) -> Option<usize>;
}

unsafe impl BSP3DFunctions for ModelCollisionGeometryBSP {
    unsafe fn test_point(&self, point: Vector3D) -> Option<usize> {
        let nodes = self.bsp3d_nodes.as_slice();
        let node_count = nodes.len();
        let mut attempts = 0;
        let mut current_node = 0usize;
        while current_node < 0x80000000 {
            // These should ideally be caught on map load.
            let Some(node) = nodes.get(current_node) else {
                panic!("Tried to access node index {current_node} / {node_count}");
            };
            let Some(plane) = self.planes.get(node.plane as usize) else {
                panic!("Node {current_node} points to an invalid plane");
            };
            attempts += 1;
            if attempts > node_count {
                panic!("Infinite BSP3D loop detected! (current_node={current_node})");
            }

            current_node = if plane.plane.distance_to_point(point) >= 0.0 { node.front_child } else { node.back_child } as usize;
        }

        if current_node != u32::MAX as usize {
            Some(current_node & 0x7FFFFFFF)
        }
        else {
            None
        }
    }
}
