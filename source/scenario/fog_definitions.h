#ifndef DEMON_FOG_DEFINITIONS_H
#define DEMON_FOG_DEFINITIONS_H

#include "../cseries/cseries.h"
#include "../tag_files/tag_groups.h"

enum {
    FOG_TAG = 0x666F6720, // 'fog '
    FOG_VERSION = 1
};

enum {
    _fog_screen_no_environment_multipass_bit,
    _fog_screen_no_model_multipass_bit,
    _fog_screen_no_texture_falloff_bit,
    NUMBER_OF_FOG_SCREEN_FLAGS
};

struct fog_screen {
    uint16_t flags;
    int16_t layer_count;
    real near_distance;
    real far_distance;
    real near_density;
    real far_density;
    real start_distance_from_fog_plane;
    uint32_t unused1[1];
    pixel32 color;
    real rotation_multiplier;
    real strafing_multiplier;
    real zoom_multiplier;
    uint32_t unused2[2];
    real map_scale;
    struct tag_reference map;
    real animation_period;
    real animation_unused[1];
    real wind_velocity_lower_bound;
    real wind_velocity_upper_bound;
    real wind_period_lower_bound;
    real wind_period_upper_bound;
    real wind_acceleration_weight;
    real wind_perpendicular_weight;
    uint32_t wind_unused[2];
};
static_assert(sizeof(struct fog_screen) == 112);

enum {
    _fog_definition_is_water_bit,
    _fog_definition_atmosphere_dominant_bit,
    _fog_definition_screen_effect_only_bit,
    NUMBER_OF_FOG_DEFINITION_FLAGS,
};

struct fog_definition {
    uint32_t flags;
    real animation_distance;
    int32_t animation_unused[19];
    int32_t unused1[1];
    real maximum_density;
    int32_t unused2[1];
    real maximum_distance;
    int32_t unused3[1];
    real maximum_depth;
    int32_t unused4[2];
    real distance_to_water_plane;
    real_rgb_color color;
    struct fog_screen screen;
    struct tag_reference background_sound;
    struct tag_reference sound_environment;
    int32_t sound_unused[30];
};
static_assert(sizeof(struct fog_definition) == 396);

static inline struct fog_definition *fog_definition_get(int32_t tag_index) {
    return tag_get(FOG_TAG, tag_index);
}

#endif
