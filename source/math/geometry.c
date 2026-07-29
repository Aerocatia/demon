#include "../cseries/cseries.h"

#include "geometry.h"

#include "../demon/exe_functions.h"

struct geosphere *geosphere_new(int16_t segment_count) {
    return RUN_EXE_FUNCTION(geosphere_new, segment_count);
}

void geosphere_dispose(struct geosphere *sphere) {
    assert(sphere);
    assert(sphere->vertices);
    assert(sphere->triangle_strip_vertex_indices);

    free(sphere->vertices);
    free(sphere->triangle_strip_vertex_indices);
    free(sphere);
}
