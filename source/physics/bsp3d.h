#ifndef DEMON_BSP3D_H
#define DEMON_BSP3D_H

#include "../cseries/cseries.h"
#include "../tag_files/tag_groups.h"

/* constants */

constexpr int32_t MAXIMUM_NODES_PER_BSP3D = 131072;
constexpr int32_t MAXIMUM_LEAVES_PER_BSP3D = 65536;
constexpr int32_t MAXIMUM_PLANES_PER_BSP3D = 65536;
constexpr int32_t MAXIMUM_BSP3D_DEPTH = 128;
constexpr int32_t BSP3D_ROOT_NODE_INDEX = 0;

/* definitions */

struct bsp3d {
    struct tag_block nodes;
    struct tag_block planes;
};
static_assert(sizeof(struct bsp3d) == 24);

struct bsp3d_node {
    int32_t plane_index;
    int32_t child_indices[2];
};
static_assert(sizeof(struct bsp3d_node) == 12);

#endif
