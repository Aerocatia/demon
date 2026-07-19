#ifndef DEMON_BSP2D_H
#define DEMON_BSP2D_H

#include "../cseries/cseries.h"
#include "../tag_files/tag_groups.h"

enum {
    MAXIMUM_NODES_PER_BSP2D = 65535,
    MAXIMUM_BSP2D_DEPTH = 64
};

struct bsp2d {
    struct tag_block nodes;
};
static_assert(sizeof(struct bsp2d) == 12);

struct bsp2d_node {
    real_plane2d plane;
    int32_t child_indices[2];
};
static_assert(sizeof(struct bsp2d_node) == 20);

#endif
