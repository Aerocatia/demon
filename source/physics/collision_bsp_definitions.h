#ifndef DEMON_COLLISION_BSP_DEFINITIONS_H
#define DEMON_COLLISION_BSP_DEFINITIONS_H

#include "../cseries/cseries.h"
#include "../tag_files/tag_groups.h"

#include "bsp3d.h"
#include "bsp2d.h"

enum {
    MAXIMUM_BSP2D_REFERENCES_PER_COLLISION_BSP = 131072,
    MAXIMUM_SURFACES_PER_COLLISION_BSP = 131072,
    MAXIMUM_EDGES_PER_COLLISION_BSP = 262144,
    MAXIMUM_VERTICES_PER_COLLISION_BSP = 131072,
    MAXIMUM_VERTICES_PER_COLLISION_SURFACE = 8,
    MAXIMUM_EDGES_PER_COLLISION_SURFACE = MAXIMUM_VERTICES_PER_COLLISION_SURFACE
};

enum {
    _collision_leaf_contains_two_sided_bit,
    NUMBER_OF_COLLISION_LEAF_FLAGS
};

enum {
    _collision_surface_two_sided_bit,
    _collision_surface_invisible_bit,
    _collision_surface_climbable_bit,
    _collision_surface_breakable_bit,
    NUMBER_OF_COLLISION_SURFACE_FLAGS
};

struct collision_bsp {
    struct bsp3d bsp3d;
    struct tag_block leaves;
    struct tag_block bsp2d_references;
    struct bsp2d bsp2d;
    struct tag_block surfaces;
    struct tag_block edges;
    struct tag_block vertices;
};
static_assert(sizeof(struct collision_bsp) == 96);

struct collision_leaf {
    uint16_t flags;
    int16_t bsp2d_reference_count;
    int32_t first_bsp2d_reference_index;
};
static_assert(sizeof(struct collision_leaf) == 8);

struct bsp2d_reference {
    int32_t plane_designator;
    int32_t root_index;
};
static_assert(sizeof(struct bsp2d_reference) == 8);

struct collision_surface {
    int32_t plane_designator;
    int32_t first_edge_index;
    uint8_t flags;
    uint8_t breakable_surface_index;
    int16_t material_index;
};
static_assert(sizeof(struct collision_surface) == 12);

struct collision_edge {
    int32_t vertex_indices[2];
    int32_t edge_indices[2];
    int32_t surface_indices[2];
};
static_assert(sizeof(struct collision_edge) == 24);

struct collision_vertex {
    real_point3d point;
    int32_t first_edge_index;
};
static_assert(sizeof(struct collision_vertex) == 16);

/* collision bsp functions */

static inline struct collision_leaf *collision_bsp_get_leaf(struct collision_bsp *collision_bsp, int32_t leaf_index) {
    return tag_block_get_element_with_size(&collision_bsp->leaves, leaf_index, sizeof(struct collision_leaf));
}

static inline struct bsp2d_reference *collision_bsp_get_bsp2d_reference(struct collision_bsp *collision_bsp, int32_t bsp2d_reference_index) {
    return tag_block_get_element_with_size(&collision_bsp->bsp2d_references, bsp2d_reference_index, sizeof(struct bsp2d_reference));
}

static inline struct collision_surface *collision_bsp_get_surface(struct collision_bsp *collision_bsp, int32_t surface_index) {
    return tag_block_get_element_with_size(&collision_bsp->surfaces, surface_index, sizeof(struct collision_surface));
}

static inline struct collision_edge *collision_bsp_get_edge(struct collision_bsp *collision_bsp, int32_t edge_index) {
    return tag_block_get_element_with_size(&collision_bsp->edges, edge_index, sizeof(struct collision_edge));
}

static inline struct collision_vertex *collision_bsp_get_vertex(struct collision_bsp *collision_bsp, int32_t vertex_index) {
    return tag_block_get_element_with_size(&collision_bsp->vertices, vertex_index, sizeof(struct collision_vertex));
}

#endif
