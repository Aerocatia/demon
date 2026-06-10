#ifndef DEMON_RENDER_H
#define DEMON_RENDER_H

#include "../cseries/cseries.h"

enum {
    MAXIMUM_RENDERED_DISTANT_LIGHTS = 2,
    MAXIMUM_RENDERED_POINT_LIGHTS = 2,
    MAXIMUM_RENDERED_ENVIRONMENT_SURFACES = 16384,
    MAXIMUM_RENDERED_CLUSTERS = 128,
    MAXIMUM_SURFACES_PER_STRUCTURE = 131072,
    MAXIMUM_RENDERED_LIGHTS = 128,
    MAXIMUM_LIGHTS_PER_MAP = 896,
    MAXIMUM_LENS_FLARES_PER_LIGHT = 8,
    MAXIMUM_QUEUED_LENS_FLARES = 8
};

struct render_distant_light {
    real_rgb_color color;
    real_vector3d direction;
};
static_assert(sizeof(struct render_distant_light) == 24);

struct render_lighting {
    real_rgb_color ambient_color;
    int16_t distant_light_count;
    uint16_t pad;
    struct render_distant_light distant_lights[MAXIMUM_RENDERED_DISTANT_LIGHTS];
    int16_t point_light_count;
    uint16_t pad1;
    int32_t point_light_indices[MAXIMUM_RENDERED_POINT_LIGHTS];
    real_argb_color reflection_tint_color;
    real_vector3d shadow_vector;
    real_rgb_color shadow_color;
};
static_assert(sizeof(struct render_lighting) == 116);

#endif
