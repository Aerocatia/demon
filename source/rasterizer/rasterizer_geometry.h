#ifndef __RASTERIZER_GEOMETRY_H__
#define __RASTERIZER_GEOMETRY_H__

#include <stdint.h>

enum {
    _rasterizer_vertex_type_environment_uncompressed,
    _rasterizer_vertex_type_environment_compressed,
    _rasterizer_vertex_type_environment_lightmap_uncompressed,
    _rasterizer_vertex_type_environment_lightmap_compressed,
    _rasterizer_vertex_type_model_uncompressed,
    _rasterizer_vertex_type_model_compressed,
    _rasterizer_vertex_type_dynamic_unlit,
    _rasterizer_vertex_type_dynamic_lit,
    _rasterizer_vertex_type_dynamic_screen,
    _rasterizer_vertex_type_debug,
    _rasterizer_vertex_type_decal,
    _rasterizer_vertex_type_detail_object,
    _rasterizer_vertex_type_environment_uncompressed_ff,
    _rasterizer_vertex_type_environment_lightmap_uncompressed_ff,
    _rasterizer_vertex_type_model_uncompressed_ff,
    _rasterizer_vertex_type_model_processed,
    _rasterizer_vertex_type_unlit_zsprite,
    _rasterizer_vertex_type_widget,
    NUMBER_OF_RASTERIZER_VERTEX_TYPES
};

struct vertex_buffer {
    int16_t type;
    uint16_t pad;
    int32_t count;
    int32_t offset;
    void *base_address;
    void *hardware_format;
};
static_assert(sizeof(struct vertex_buffer) == 20);

enum {
    _triangle_buffer_type_triangles,
    _triangle_buffer_type_precompiled_strip,
    NUMBER_OF_TRIANGLE_BUFFER_TYPES
};

struct triangle_buffer {
    int16_t type;
    uint16_t pad;
    int32_t count;
    void *base_address;
    void *hardware_format;
};
static_assert(sizeof(struct triangle_buffer) == 16);

#endif
