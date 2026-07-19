#ifndef DEMON_RASTERIZER_GEOMETRY_H
#define DEMON_RASTERIZER_GEOMETRY_H

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

struct environment_vertex_uncompressed {
    real_point3d position;
    real_vector3d normal;
    real_vector3d binormal;
    real_vector3d tangent;
    real_point2d texcoord;
};
static_assert(sizeof(struct environment_vertex_uncompressed) == 56);

struct environment_vertex_compressed {
    real_point3d position;
    uint32_t normal;
    uint32_t binormal;
    uint32_t tangent;
    real_point2d texcoord;
};
static_assert(sizeof(struct environment_vertex_compressed) == 32);

struct environment_lightmap_vertex_uncompressed {
    real_vector3d incident_radiosity;
    real_point2d texcoord;
};
static_assert(sizeof(struct environment_lightmap_vertex_uncompressed) == 20);

struct environment_lightmap_vertex_compressed {
    uint32_t incident_radiosity;
    uint16_t lightmap_u;
    uint16_t lightmap_v;
};
static_assert(sizeof(struct environment_lightmap_vertex_compressed) == 8);

struct triangle_buffer {
    int16_t type;
    uint16_t pad;
    int32_t count;
    void *base_address;
    void *hardware_format;
};
static_assert(sizeof(struct triangle_buffer) == 16);

#endif
