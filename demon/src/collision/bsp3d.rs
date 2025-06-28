use crate::tag::ReflexiveImpl;
use tag_structs::primitives::collision_bsp::{BSP2DNodeReference, CollisionBSP2DNode, CollisionBSP2DNodeIndex, CollisionBSP3DNode, CollisionBSP3DNodeIndex, CollisionBSPLeaf, CollisionBSPSurface};
use tag_structs::primitives::vector::Plane3D;
use tag_structs::{primitives, ModelCollisionGeometryBSP, ModelCollisionGeometryBSPLeafFlagsFields};
use crate::collision::GLOBAL_BSP3D;

pub unsafe fn get_global_collision_bsp() -> ModelCollisionGeometryBSPImpl<'static> {
    ModelCollisionGeometryBSPImpl::wrap(&**GLOBAL_BSP3D.get())
}

#[repr(transparent)]
pub struct ModelCollisionGeometryBSPImpl<'a> {
    bsp: &'a ModelCollisionGeometryBSP
}

impl<'a> ModelCollisionGeometryBSPImpl<'a> {
    #[inline(always)]
    pub unsafe fn wrap(bsp: &'a ModelCollisionGeometryBSP) -> Self {
        Self { bsp }
    }
}

impl<'a> primitives::collision_bsp::CollisionBSPFunctions for ModelCollisionGeometryBSPImpl<'a> {
    fn get_3d_node(&self, node: usize) -> Option<CollisionBSP3DNode> {
        unsafe { self.bsp.bsp3d_nodes.get(node) }.map(|n| CollisionBSP3DNode {
            front_child: CollisionBSP3DNodeIndex(n.front_child),
            back_child: CollisionBSP3DNodeIndex(n.back_child),
            plane_index: n.plane as usize
        })
    }

    fn get_3d_node_count(&self) -> usize {
        self.bsp.bsp3d_nodes.len()
    }

    fn get_plane(&self, plane: usize) -> Option<Plane3D> {
        unsafe { self.bsp.planes.get(plane) }.map(|n| n.plane)
    }

    fn get_plane_count(&self) -> usize {
        self.bsp.planes.len()
    }

    fn get_leaf(&self, leaf: usize) -> Option<CollisionBSPLeaf> {
        unsafe { self.bsp.leaves.get(leaf) }.map(|n| CollisionBSPLeaf {
            contains_double_sided_surfaces: n.flags.is_set(ModelCollisionGeometryBSPLeafFlagsFields::ContainsDoubleSidedSurfaces),
            bsp_2d_node_reference_start: n.first_bsp2d_reference as usize,
            bsp_2d_node_reference_count: n.bsp2d_reference_count as usize,
        })
    }

    fn get_leaf_count(&self) -> usize {
        self.bsp.leaves.len()
    }

    fn get_2d_node_reference(&self, node: usize) -> Option<BSP2DNodeReference> {
        unsafe { self.bsp.bsp2d_references.get(node) }.map(|n| BSP2DNodeReference {
            plane: n.plane as usize,
            node: CollisionBSP2DNodeIndex(n.bsp2d_node)
        })
    }

    fn get_2d_node_reference_count(&self) -> usize {
        self.bsp.bsp2d_references.len()
    }

    fn get_2d_node(&self, node: usize) -> Option<CollisionBSP2DNode> {
        unsafe { self.bsp.bsp2d_nodes.get(node) }.map(|n| CollisionBSP2DNode {
            plane: n.plane,
            left_child: CollisionBSP2DNodeIndex(n.left_child),
            right_child: CollisionBSP2DNodeIndex(n.right_child),
        })
    }

    fn get_2d_node_count(&self) -> usize {
        self.bsp.bsp2d_nodes.len()
    }

    fn get_surface(&self, surface: usize) -> Option<CollisionBSPSurface> {
        unsafe { self.bsp.surfaces.get(surface) }.map(|n| CollisionBSPSurface {
            plane: n.plane as usize,
            material: n.material.0
        })
    }

    fn get_surface_count(&self) -> usize {
        self.bsp.surfaces.len()
    }
}
