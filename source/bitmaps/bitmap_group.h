#ifndef DEMON_BITMAP_GROUP_H
#define DEMON_BITMAP_GROUP_H

#include "../cseries/cseries.h"
#include "../tag_files/tag_groups.h"

#include "bitmaps.h"

enum {
    BITMAP_GROUP_TAG = 0x6269746D, // 'bitm'
    BITMAP_GROUP_VERSION = 7,
    BITMAP_GROUP_SHOW_BITMAP_CUSTOM_ID = 0x62736877, // 'bshw'
    MAXIMUM_BITMAP_PIXELS_SIZE = 16 * MIB
};

enum {
    _bitmap_group_diffusion_dither_bit,
    _bitmap_group_disable_vector_compression_bit,
    _bitmap_group_uniform_sprite_sequences_bit,
    _bitmap_group_extract_sprites_filthy_bug_fix_bit,
    _bitmap_group_half_hud_scale_bit,
    _bitmap_group_invert_detail_fade_bit,
    _bitmap_group_use_average_color_for_detail_fade_bit,
    _bitmap_group_force_hud_use_highres_scale_bit,
    NUMBER_OF_BITMAP_GROUP_FLAGS
};

enum {
    _bitmap_group_type_2d_textures,
    _bitmap_group_type_3d_textures,
    _bitmap_group_type_cube_maps,
    _bitmap_group_type_sprites,
    _bitmap_group_type_interface_bitmaps,
    NUMBER_OF_BITMAP_GROUP_TYPES
};

enum {
    _bitmap_group_format_compressed_color_key_transparency,
    _bitmap_group_format_compressed_explicit_alpha,
    _bitmap_group_format_compressed_interpolated_alpha,
    _bitmap_group_format_16bit_color,
    _bitmap_group_format_32bit_color,
    _bitmap_group_format_monochrome,
    _bitmap_group_format_high_quality_compression,
    NUMBER_OF_BITMAP_GROUP_FORMATS
};

enum {
    _bitmap_group_usage_alpha_blend,
    _bitmap_group_usage_default,
    _bitmap_group_usage_height_map,
    _bitmap_group_usage_detail_map,
    _bitmap_group_usage_light_map,
    _bitmap_group_usage_vector_map,
    NUMBER_OF_BITMAP_GROUP_USAGES
};

enum {
    _bitmap_group_sprite_budget_32,
    _bitmap_group_sprite_budget_64,
    _bitmap_group_sprite_budget_128,
    _bitmap_group_sprite_budget_256,
    _bitmap_group_sprite_budget_512,
    _bitmap_group_sprite_budget_1024,
    NUMBER_OF_BITMAP_GROUP_SPRITE_BUDGETS
};

enum {
    _bitmap_group_sprite_no_alpha_bleed_bit,
    NUMBER_OF_BITMAP_GROUP_SPRITE_FLAGS
};

enum {
    _bitmap_group_sprite_usage_blend_add_sub_max,
    _bitmap_group_sprite_usage_mul_min,
    _bitmap_group_sprite_usage_double_multiply,
    NUMBER_OF_BITMAP_GROUP_SPRITE_USAGES
};

struct bitmap_group_sprite {
    int16_t bitmap_index;
    int16_t bitmap_pad;
    int32_t unused;
    real_rectangle2d bounds;
    real_point2d registration_point;
};
static_assert(sizeof(struct bitmap_group_sprite) == 32);

struct bitmap_group_sequence {
    char name[TAG_STRING_LENGTH + 1];
    int16_t first_bitmap_index;
    int16_t bitmap_count;
    int32_t unused[4];
    struct tag_block sprites;
};
static_assert(sizeof(struct bitmap_group_sequence) == 64);

struct bitmap_group {
    int16_t type;
    int16_t format;
    int16_t usage;
    uint16_t flags;
    real detail_fade;
    real sharpen_amount;
    real bump_height;
    int16_t sprite_budget_size;
    int16_t sprite_budget_count;
    int16_t import_width;
    int16_t import_height;
    struct tag_data import_bitmap;
    struct tag_data pixel_data;
    real smoothing_filter_size;
    real alpha_bias;
    int16_t mipmap_count;
    int16_t sprite_usage;
    int16_t sprite_spacing;
    uint16_t pad;
    struct tag_block sequences;
    struct tag_block bitmaps;
};
static_assert(sizeof(struct bitmap_group) == 108);

/* bitmap group functions */

static inline struct bitmap_group *bitmap_group_get(int32_t index) {
    return tag_get(BITMAP_GROUP_TAG, index);
}

static inline struct bitmap_group_sequence *bitmap_group_get_sequence(struct bitmap_group *bitmap_group, int32_t index) {
    return tag_block_get_element_with_size(&bitmap_group->sequences, index, sizeof(struct bitmap_group_sequence));
}

static inline struct bitmap_data *bitmap_group_get_bitmap(struct bitmap_group *bitmap_group, int32_t index) {
    return tag_block_get_element_with_size(&bitmap_group->bitmaps, index, sizeof(struct bitmap_data));
}

static inline struct bitmap_group_sprite *bitmap_group_sequence_get_sprite(struct bitmap_group_sequence *sequence, int32_t index) {
    return tag_block_get_element_with_size(&sequence->sprites, index, sizeof(struct bitmap_group_sprite));
}

#endif
