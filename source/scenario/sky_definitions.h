#ifndef DEMON_SKY_DEFINITIONS_H
#define DEMON_SKY_DEFINITIONS_H

#include "../cseries/cseries.h"
#include "../tag_files/tag_groups.h"

enum {
    SKY_TAG = 0x736B7920, // 'sky '
    SKY_VERSION = 1,
    MAXIMUM_SHADER_FUNCTIONS_PER_SKY = 8,
    MAXIMUM_ANIMATIONS_PER_SKY = 8,
    MAXIMUM_LIGHTS_PER_SKY = 8
};

struct sky_atmospheric_fog {
    real_rgb_color color;
    real unused[2];
    real maximum_density;
    real z_near;
    real z_far;
};
static_assert(sizeof(struct sky_atmospheric_fog) == 32);

struct sky {
    struct tag_reference model;
    struct tag_reference animation_graph;
    int32_t unused[6];
    real_rgb_color radiosity_indoor_ambient_color;
    real radiosity_indoor_ambient_power;
    real_rgb_color radiosity_outdoor_ambient_color;
    real radiosity_outdoor_ambient_power;
    struct sky_atmospheric_fog outdoor_fog;
    struct sky_atmospheric_fog indoor_fog;
    struct tag_reference indoor_fog_plane;
    int32_t unused2[1];
    struct tag_block shader_functions;
    struct tag_block animations;
    struct tag_block lights;
};
static_assert(sizeof(struct sky) == 208);

struct sky_shader_function {
    uint16_t pad;
    int16_t runtime_global_function_index;
    char global_function_name[TAG_STRING_LENGTH + 1];
};
static_assert(sizeof(struct sky_shader_function) == 36);

struct sky_animation {
    int16_t animation_index;
    uint16_t pad;
    real period;
    int32_t unused[7];
};
static_assert(sizeof(struct sky_animation) == 36);

enum {
    _sky_radiosity_light_exterior_bit,
    _sky_radiosity_light_interior_bit,
    NUMBER_OF_SKY_RADIOSITY_LIGHT_FLAGS
};

struct sky_radiosity_light {
    uint32_t flags;
    real_rgb_color color;
    real power;
    real test_distance;
    int32_t unused;
    real_euler_angles2d direction;
    real diameter;
};
static_assert(sizeof(struct sky_radiosity_light) == 40);

struct sky_light {
    struct tag_reference lens_flare;
    char lens_flare_marker_name[TAG_STRING_LENGTH + 1];
    int32_t unused[7];
    struct sky_radiosity_light radiosity;
};
static_assert(sizeof(struct sky_light) == 116);

/* sky functions */

static inline struct sky *sky_get(int32_t tag_index) {
    return tag_get(SKY_TAG, tag_index);
}

static inline struct sky_shader_function *sky_get_shader_function(struct sky *sky, int32_t shader_function_index) {
    return tag_block_get_element_with_size(&sky->shader_functions, shader_function_index, sizeof(struct sky_shader_function));
}

static inline struct sky_animation *sky_get_animation(struct sky *sky, int32_t animation_index) {
    return tag_block_get_element_with_size(&sky->animations, animation_index, sizeof(struct sky_animation));
}

static inline struct sky_light *sky_get_light(struct sky *sky, int32_t light_index) {
    return tag_block_get_element_with_size(&sky->lights, light_index, sizeof(struct sky_light));
}

#endif
