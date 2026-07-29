#ifndef DEMON_GEOMETRY_H
#define DEMON_GEOMETRY_H

#include "../cseries/cseries.h"

struct geosphere {
    int16_t segment_count;
    real_point3d *vertices;
    int16_t *triangle_strip_vertex_indices;
    int16_t vertex_count;
    int16_t triangle_count;
    int16_t triangle_strip_count;
};

struct geosphere *geosphere_new(int16_t segment_count);
void geosphere_dispose(struct geosphere *sphere);

#endif
