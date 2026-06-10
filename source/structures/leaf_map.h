#ifndef DEMON_LEAF_MAP_H
#define DEMON_LEAF_MAP_H

#include <stdint.h>

#include "../tag_files/tag_groups.h"
#include "../physics/bsp3d.h"

enum {
    MAXIMUM_FACES_PER_MAP_LEAF = 256,
    MAXIMUM_PORTALS_PER_MAP_LEAF = 256,
    MAXIMUM_PORTALS_PER_LEAF_MAP = 524288,
    MAXIMUM_VERTICES_PER_LEAF_PORTAL = 64,
};

struct leaf_map {
    const struct bsp3d *bsp;
    struct tag_block leaves;
    struct tag_block portals;
};
static_assert(sizeof(struct leaf_map) == 28);

struct map_leaf {
    struct tag_block faces;
    struct tag_block portal_designators;
};
static_assert(sizeof(struct map_leaf) == 24);

struct map_leaf_face {
    int32_t node_index;
    struct tag_block vertices;
};
static_assert(sizeof(struct map_leaf_face) == 16);

struct leaf_portal {
    int32_t plane_index;
    int32_t leaf_indices[2];
    struct tag_block vertices;
};
static_assert(sizeof(struct leaf_portal) == 24);

#endif
