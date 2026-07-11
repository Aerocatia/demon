#ifndef DEMON_STRUCTURE_BSP_H
#define DEMON_STRUCTURE_BSP_H

#include <stdint.h>

#include "../cseries/cseries.h"
#include "../tag_files/tag_groups.h"

#include "../rasterizer/rasterizer_geometry.h"

#include "leaf_map.h"

enum {
    STRUCTURE_BSP_TAG = 0x73627370,
    STRUCTURE_BSP_VERSION = 5,
};

enum {
    MAXIMUM_COLLISION_MATERIALS_PER_STRUCTURE = 512,
    MAXIMUM_SURFACE_REFERENCES_PER_STRUCTURE = 256 * KIB,
    MAXIMUM_LIGHTMAPS_PER_STRUCTURE = 128,
    MAXIMUM_MATERIALS_PER_STRUCTURE_LIGHTMAP = 2 * KIB,
    MAXIMUM_SURFACES_PER_STRUCTURE_MATERIAL = 20000,
    MAXIMUM_VERTICES_PER_STRUCTURE_MATERIAL = 64000,
    MAXIMUM_CLUSTER_PORTALS_PER_CLUSTER = 128,
    MAXIMUM_MIRRORS_PER_CLUSTER = 16,
    MAXIMUM_SUBCLUSTERS_PER_CLUSTER = 4096,
    MAXIMUM_SURFACES_PER_SUBCLUSTER = 128,
    MAXIMUM_SURFACES_PER_CLUSTER = 32 * KIB,
    MAXIMUM_VERTICES_PER_MIRROR = 512,
    MAXIMUM_TEMPORARY_CLUSTERS_PER_STRUCTURE = 8 * KIB,
    MAXIMUM_CLUSTERS_PER_STRUCTURE = 512,
    MAXIMUM_CLUSTER_DATA_SIZE = 65536,
    MAXIMUM_CLUSTER_PORTALS_PER_STRUCTURE = 512,
    MAXIMUM_VERTICES_PER_CLUSTER_PORTAL = 128,
    MAXIMUM_FOG_PLANES_PER_STRUCTURE = 32,
    MAXIMUM_VERTICES_PER_STRUCTURE_FOG_PLANE = 4 * KIB,
    MAXIMUM_FOG_REGIONS_PER_STRUCTURE = 32,
    MAXIMUM_FOG_PALETTE_ENTRIES_PER_STRUCTURE = 32,
    MAXIMUM_WEATHER_PALETTE_ENTRIES_PER_STRUCTURE = 32,
    MAXIMUM_WEATHER_POLYHEDRA_PER_STRUCTURE = 32,
    MAXIMUM_PLANES_PER_WEATHER_POLYHEDRON = 16,
    MAXIMUM_BACKGROUND_SOUND_PALETTE_ENTRIES_PER_STRUCTURE = 64,
    MAXIMUM_SOUND_ENVIRONMENT_PALETTE_ENTRIES_PER_STRUCTURE = 64,
    MAXIMUM_MARKERS_PER_STRUCTURE = 1 * KIB,
    MAXIMUM_LENS_FLARES_PER_STRUCTURE = 256,
    MAXIMUM_LENS_FLARE_MARKERS_PER_STRUCTURE = 64 * KIB,
    MAXIMUM_DECALS_PER_STRUCTURE = 6 * KIB,
};

#include "../render/render.h"

struct structure_collision_material {
    struct tag_reference shader;
    uint16_t pad;
    int16_t runtime_physics_material_type;
};
static_assert(sizeof(struct structure_collision_material) == 20);

struct structure_node {
    byte_rectangle3d bounds;
};
static_assert(sizeof(struct structure_node) == 6);

struct structure_leaf {
    byte_rectangle3d bounds;
    uint16_t pad;
    int16_t cluster_index;
    int16_t surface_reference_count;
    int32_t first_surface_reference_index;
};
static_assert(sizeof(struct structure_leaf) == 16);

struct structure_surface {
    uint16_t vertex_indices[NUMBER_OF_VERTICES_PER_TRIANGLE];
};
static_assert(sizeof(struct structure_surface) == 6);

struct structure_surface_reference {
    int32_t surface_index;
    int32_t bsp3d_node_index;
};
static_assert(sizeof(struct structure_surface_reference) == 8);

struct structure_lightmap {
    int16_t bitmap_index;
    uint16_t pad;
    uint32_t unused[4];
    struct tag_block materials;
};
static_assert(sizeof(struct structure_lightmap) == 32);

enum {
    _structure_material_coplanar_bit,
    _structure_material_fog_plane_bit,
    NUMBER_OF_STRUCTURE_MATERIAL_FLAGS
};

struct structure_material {
    struct tag_reference shader;
    int16_t permutation_index;
    uint16_t flags;
    int32_t first_surface_index;
    int32_t surface_count;
    real_point3d centroid;
    struct render_lighting lighting;
    real_plane3d plane;
    int16_t breakable_surface_index;
    uint16_t pad;
    struct vertex_buffer vertices;
    struct vertex_buffer lightmap_vertices;
    struct tag_data uncompressed_vertex_data;
    struct tag_data compressed_vertex_data;
};
static_assert(sizeof(struct structure_material) == 256);

struct structure_lens_flare {
    struct tag_reference lens_flare;
};
static_assert(sizeof(struct structure_lens_flare) == 16);

struct structure_lens_flare_marker {
    real_point3d position;
    int8_t i_direction;
    int8_t j_direction;
    int8_t k_direction;
    uint8_t lens_flare_index;
};
static_assert(sizeof(struct structure_lens_flare_marker) == 16);

struct structure_subcluster {
    real_rectangle3d world_bounds;
    struct tag_block surface_indices;
};
static_assert(sizeof(struct structure_subcluster) == 36);

struct structure_cluster {
    int16_t sky_index;
    int16_t fog_designator;
    int16_t background_sound_palette_index;
    int16_t sound_environment_palette_index;
    int16_t weather_palette_index;
    int16_t transitions_to_structure_bsp_index;
    int16_t first_runtime_decal_index;
    uint16_t runtime_decal_count;
    uint32_t unused1[6];
    struct tag_block predicted_resources;
    struct tag_block subclusters;
    uint16_t first_lens_flare_marker_index;
    uint16_t lens_flare_marker_count;
    struct tag_block surface_indices;
    struct tag_block mirrors;
    struct tag_block portal_indices;
};
static_assert(sizeof(struct structure_cluster) == 104);

struct structure_mirror {
    real_plane3d plane;
    uint32_t unused[5];
    struct tag_reference shader;
    struct tag_block points;
};
static_assert(sizeof(struct structure_mirror) == 64);

enum {
    _cluster_portal_ai_deafening_bit,
    NUMBER_OF_CLUSTER_PORTAL_FLAGS
};

struct cluster_portal {
    int16_t cluster_indices[2];
    int32_t plane_index;
    real_point3d centroid;
    real bounding_radius;
    uint32_t flags;
    uint32_t unused[6];
    struct tag_block vertices;
};
static_assert(sizeof(struct cluster_portal) == 64);

struct structure_fog_plane {
    int16_t region_index;
    int16_t runtime_material_type;
    real_plane3d plane;
    struct tag_block vertices;
};
static_assert(sizeof(struct structure_fog_plane) == 32);

struct structure_fog_region {
    uint32_t unused[9];
    int16_t fog_palette_index;
    int16_t weather_palette_index;
};
static_assert(sizeof(struct structure_fog_region) == 40);

struct structure_fog_palette_entry {
    char name[TAG_STRING_BUFFER_LENGTH];
    struct tag_reference fog;
    uint16_t pad;
    int16_t runtime_global_function_index;
    char global_function_name[TAG_STRING_BUFFER_LENGTH];
    uint32_t unused[13];
};
static_assert(sizeof(struct structure_fog_palette_entry) == 136);

struct structure_weather_palette_entry {
    char name[TAG_STRING_BUFFER_LENGTH];
    struct tag_reference particle_system;
    uint16_t pad1;
    int16_t runtime_particle_system_global_function_index;
    char particle_system_global_function_name[TAG_STRING_BUFFER_LENGTH];
    uint32_t unused0[11];
    struct tag_reference wind;
    real_vector3d wind_direction;
    real wind_magnitude;
    uint16_t pad2;
    int16_t wind_global_function_index;
    char wind_global_function_name[TAG_STRING_BUFFER_LENGTH];
    uint32_t unused1[11];
};
static_assert(sizeof(struct structure_weather_palette_entry) == 240);

struct structure_weather_polyhedron {
    real_point3d bounding_sphere_center;
    real bounding_sphere_radius;
    uint32_t unused;
    struct tag_block planes;
};
static_assert(sizeof(struct structure_weather_polyhedron) == 32);

struct structure_sound_environment_palette_entry {
    char name[TAG_STRING_BUFFER_LENGTH];
    struct tag_reference sound_environment;
    uint32_t unused[8];
};

struct structure_background_sound_palette_entry {
    char name[TAG_STRING_BUFFER_LENGTH];
    struct tag_reference background_sound;
    uint16_t pad;
    int16_t runtime_global_function_index;
    char global_function_name[TAG_STRING_BUFFER_LENGTH];
    uint32_t unused[8];
};
static_assert(sizeof(struct structure_background_sound_palette_entry) == 116);

struct structure_breakable_surface {
    real_point3d centroid;
    real bounding_radius;
    int32_t collision_surface_index;
    uint32_t unused[7];
};
static_assert(sizeof(struct structure_breakable_surface) == 48);

struct structure_marker {
    char name[TAG_STRING_BUFFER_LENGTH];
    real_quaternion rotation;
    real_point3d translation;
};
static_assert(sizeof(struct structure_marker) == 60);

struct structure_detail_object_data {
    struct tag_block cells;
    struct tag_block detail_objects;
    struct tag_block detail_objects_counts;
    struct tag_block detail_object_z_reference_vectors;
    bool valid;
    uint8_t pad[3];
    uint32_t unused[3];
};
static_assert(sizeof(struct structure_detail_object_data) == 64);

struct structure_runtime_decal {
    real_point3d position;
    uint8_t palette_index;
    uint8_t unused;
    int8_t yaw;
    int8_t pitch;
};
static_assert(sizeof(struct structure_runtime_decal) == 16);

struct structure_bsp {
    struct tag_reference lightmap_group;
    real vehicle_floor;
    real vehicle_ceiling;
    uint32_t sad_unused[5];
    struct render_lighting default_lighting;
    uint32_t lonely_unused;
    struct tag_block collision_materials;
    struct tag_block collision_bsp;
    struct tag_block nodes;
    real_rectangle3d world_bounds;
    struct tag_block leaves;
    struct tag_block surface_references;
    struct tag_block surfaces;
    struct tag_block lightmaps;
    uint32_t render_unused[3];
    struct tag_block lens_flares;
    struct tag_block lens_flare_markers;
    struct tag_block clusters;
    struct tag_data cluster_data;
    struct tag_block cluster_portals;
    uint32_t cluster_unused[3];
    struct tag_block breakable_surfaces;
    struct tag_block fog_planes;
    struct tag_block fog_regions;
    struct tag_block fog_palette;
    uint32_t fog_unused[6];
    struct tag_block weather_palette;
    struct tag_block weather_polyhedra;
    uint32_t weather_unused[6];
    struct tag_block pathfinding_surfaces;
    struct tag_block pathfinding_edges;
    struct tag_block background_sound_palette;
    struct tag_block sound_environment_palette;
    struct tag_data sound_cluster_data;
    uint32_t sound_unused[6];
    struct tag_block markers;
    struct tag_block detail_object_data;
    struct tag_block runtime_decals;
    uint32_t diminishing_misc_unused[2];
    struct leaf_map leaf_map;
};
static_assert(sizeof(struct structure_bsp) == 648);

#define structure_bsp_encoded_sound_data_row_offset(structure_bsp, row_index) (((structure_bsp)->clusters.count * (row_index)) - ((row_index) * ((row_index) + 1) / 2))

#endif
