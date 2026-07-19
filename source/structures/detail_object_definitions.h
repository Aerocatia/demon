#ifndef DEMON_DETAIL_OBJECT_DEFINITIONS_H
#define DEMON_DETAIL_OBJECT_DEFINITIONS_H

enum {
    DETAIL_OBJECT_COLLECTION_GROUP_TAG = 0x646F6263, // 'dobc'
    DETAIL_OBJECT_COLLECTION_VERSION = 1
};

enum {
    MAXIMUM_DETAIL_OBJECT_LAYERS_PER_STRUCTURE = 32,
    MAXIMUM_DETAIL_OBJECT_CELLS_PER_STRUCTURE = 256 * KIB,
    MAXIMUM_DETAIL_OBJECTS_PER_STRUCTURE = 2 * MIB,
    MAXIMUM_DETAIL_OBJECTS_PER_CELL = 512,
    MAXIMUM_DETAIL_OBJECT_TYPES_PER_COLLECTION = 16,
    MAXIMUM_DETAIL_OBJECT_SPRITES_PER_TYPE = 16,
    MAXIMUM_DETAIL_OBJECT_SPRITES_PER_COLLECTION = 128,
    MAXIMUM_DETAIL_OBJECT_CELLS_PER_AXIS = 3,
    MAXIMUM_DETAIL_OBJECT_CELLS_PER_FRAME = SQR(MAXIMUM_DETAIL_OBJECT_CELLS_PER_AXIS) * MAXIMUM_DETAIL_OBJECT_CELLS_PER_AXIS,
    DETAIL_OBJECT_CELL_SIZE = 8,
    DETAIL_OBJECT_CELL_GRANULARITY = 255
};

enum {
    _detail_object_collection_type_screen_facing,
    _detail_object_collection_type_viewer_facing,
    NUMBER_OF_DETAIL_OBJECT_COLLECTION_TYPES
};

struct detail_object_collection_definition {
    int16_t collection_type;
    uint16_t pad;
    real global_z_offset;
    int32_t unused1[11];
    struct tag_reference map;
    struct tag_block type_definitions;
    int32_t unused2[12];
};
static_assert(sizeof(struct detail_object_collection_definition) == 128);

enum {
    _detail_object_type_allow_horizontal_mirroring_bit,
    _detail_object_type_allow_vertical_mirroring_bit,
    _detail_object_type_color_interpolate_in_hsv_bit,
    _detail_object_type_color_interpolate_along_farthest_hue_path_bit,
    NUMBER_OF_DETAIL_OBJECT_TYPE_FLAGS
};

struct detail_object_type_definition {
    char name[TAG_STRING_LENGTH + 1];
    uint8_t sequence_index;
    uint8_t flags;
    uint8_t first_frame_index;
    uint8_t frame_count;
    real color_override_factor;
    int32_t unused1[2];
    real near_fade_distance;
    real far_fade_distance;
    real size_min;
    real size_max;
    real_rgb_color color_min;
    real_rgb_color color_max;
    pixel32 color_ambient;
    int32_t unused2[1];
};
static_assert(sizeof(struct detail_object_type_definition) == 96);

struct detail_object_count {
    uint16_t count;
};
static_assert(sizeof(struct detail_object_count) == 2);

struct detail_object_cell_definition {
    int16_t cell_x;
    int16_t cell_y;
    int16_t cell_z;
    int16_t offset_z;
    uint32_t valid_layers;
    int32_t start_index;
    int32_t count_index;
    int32_t unused[3];
};
static_assert(sizeof(struct detail_object_cell_definition) == 32);

struct detail_object_view_data {
    struct detail_object_layer_data *layers;
    int16_t layer_count;
    int16_t pad;
};
static_assert(sizeof(struct detail_object_view_data) == 8);

struct detail_object_layer_data {
    struct detail_object_cell_data *cells;
    int16_t cell_count;
    int16_t collection_definition_index;
};
static_assert(sizeof(struct detail_object_layer_data) == 8);

struct detail_object_cell_data {
    int32_t first_detail_object_index;
    int32_t detail_object_count;
    int16_t cell_x;
    int16_t cell_y;
    real cell_z;
    int32_t internal__first_vertex_index;
    const real_vector4d *z_reference_vector;
};
static_assert(sizeof(struct detail_object_cell_data) == 24);

struct detail_object {
    uint8_t position[3];
    uint8_t data;
    pixel16 color;
};
static_assert(sizeof(struct detail_object) == 6);

/* detail object definition functions */

static inline struct detail_object_collection_definition *detail_object_collection_get(int32_t tag_index) {
    return tag_get(DETAIL_OBJECT_COLLECTION_GROUP_TAG, tag_index);
}

static inline struct detail_object_type_definition *detail_object_collection_get_type(struct detail_object_collection_definition *collection_definition, int32_t type_definition_index) {
    return tag_block_get_element_with_size(&collection_definition->type_definitions, type_definition_index, sizeof(struct detail_object_type_definition));
}

#endif
