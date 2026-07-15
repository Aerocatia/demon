#ifndef DEMON_BITMAPS_H
#define DEMON_BITMAPS_H

#include <stdint.h>

#include "../cseries/cseries.h"

enum {
    BITMAP_SIGNATURE = 0x6269746D, // 'bitm'
    BITMAP_MAXIMUM_SPRITE_PAGE_MIPMAP_COUNT = 2,
    NUMBER_OF_ENTRIES_IN_PALETTE = 256,
    BITMAP_BLOCK_SIZE = 4,
    BITMAP_PIXELS_PER_BLOCK = 16
};

extern pixel32 global_vector_palette[NUMBER_OF_ENTRIES_IN_PALETTE];

enum {
    _bitmap_type_2d,
    _bitmap_type_3d,
    _bitmap_type_cube_map,
    NUMBER_OF_BITMAP_TYPES
};

enum {
    _bitmap_usage_additive,
    _bitmap_usage_multiplicative,
    _bitmap_usage_detail,
    _bitmap_usage_vector,
    NUMBER_OF_BITMAP_USAGES
};

enum {
    _bitmap_format_a8,
    _bitmap_format_y8,
    _bitmap_format_ay8,
    _bitmap_format_a8y8,
    _bitmap_format_unused1,
    _bitmap_format_unused2,
    _bitmap_format_r5g6b5,
    _bitmap_format_unused3,
    _bitmap_format_a1r5g5b5,
    _bitmap_format_a4r4g4b4,
    _bitmap_format_x8r8g8b8,
    _bitmap_format_a8r8g8b8,
    _bitmap_format_unused4,
    _bitmap_format_unused5,
    _bitmap_format_dxt1,
    _bitmap_format_dxt3,
    _bitmap_format_dxt5,
    _bitmap_format_p8_bump,
    _bitmap_format_bc7,
    NUMBER_OF_BITMAP_FORMATS
};

enum {
    _cube_map_positive_x,
    _cube_map_negative_x,
    _cube_map_positive_y,
    _cube_map_negative_y,
    _cube_map_positive_z,
    _cube_map_negative_z,
    NUMBER_OF_CUBE_MAP_FACES
};

enum {
    _bitmap_has_power_of_two_dimensions_bit,
    _bitmap_compressed_bit,
    _bitmap_palettized_bit,
    _bitmap_swizzled_bit,
    _bitmap_linear_bit,
    _bitmap_format_v16u16_bit,
    NUMBER_OF_ON_DISK_BITMAP_FLAGS,
    _bitmap_free_on_delete_bit = NUMBER_OF_ON_DISK_BITMAP_FLAGS,
    _bitmap_cached_bit,
    _bitmap_data_file_cache_bit,
    NUMBER_OF_BITMAP_FLAGS
};

enum {
    MAXIMUM_BITMAP_WIDTH = 30000,
    MAXIMUM_BITMAP_HEIGHT = 30000,
    MAXIMUM_BITMAP_DEPTH = 256,
};

struct bitmap_data {
    tag signature;
    int16_t width;
    int16_t height;
    int16_t depth;
    int16_t type;
    int16_t format;
    uint16_t flags;
    point2d registration_point;
    int16_t mipmap_count;
    int16_t mipmap_pad;
    int32_t pixels_offset;
    int32_t pixels_size;
    int32_t tag_index;
    int32_t cache_block_index;
    void *hardware_format;
    void *base_address;
};
static_assert(sizeof(struct bitmap_data) == 48);

#endif
