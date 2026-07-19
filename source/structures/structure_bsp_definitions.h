#ifndef DEMON_STRUCTURE_BSP_H
#define DEMON_STRUCTURE_BSP_H

#include <stdint.h>

#include "../cseries/cseries.h"
#include "../tag_files/tag_groups.h"
#include "../rasterizer/rasterizer_geometry.h"
#include "../physics/collision_bsp_definitions.h"

#include "leaf_map.h"
#include "detail_object_definitions.h"

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
    char name[TAG_STRING_LENGTH + 1];
    struct tag_reference fog;
    uint16_t pad;
    int16_t runtime_global_function_index;
    char global_function_name[TAG_STRING_LENGTH + 1];
    uint32_t unused[13];
};
static_assert(sizeof(struct structure_fog_palette_entry) == 136);

struct structure_weather_palette_entry {
    char name[TAG_STRING_LENGTH + 1];
    struct tag_reference particle_system;
    uint16_t pad1;
    int16_t runtime_particle_system_global_function_index;
    char particle_system_global_function_name[TAG_STRING_LENGTH + 1];
    uint32_t unused0[11];
    struct tag_reference wind;
    real_vector3d wind_direction;
    real wind_magnitude;
    uint16_t pad2;
    int16_t wind_global_function_index;
    char wind_global_function_name[TAG_STRING_LENGTH + 1];
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
    char name[TAG_STRING_LENGTH + 1];
    struct tag_reference sound_environment;
    uint32_t unused[8];
};

struct structure_background_sound_palette_entry {
    char name[TAG_STRING_LENGTH + 1];
    struct tag_reference background_sound;
    uint16_t pad;
    int16_t runtime_global_function_index;
    char global_function_name[TAG_STRING_LENGTH + 1];
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
    char name[TAG_STRING_LENGTH + 1];
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

/* structure bsp functions */

static inline struct structure_bsp *structure_bsp_get(int32_t tag_index) {
    return tag_get(STRUCTURE_BSP_TAG, tag_index);
}

static inline struct structure_collision_material *structure_bsp_get_collision_material(struct structure_bsp *structure_bsp, int32_t collision_material_index) {
    return tag_block_get_element_with_size(&structure_bsp->collision_materials, collision_material_index, sizeof(struct structure_collision_material));
}

static inline struct collision_bsp *structure_bsp_get_collision_bsp(struct structure_bsp *structure_bsp) {
    return tag_block_get_element_with_size(&structure_bsp->collision_bsp, 0, sizeof(struct collision_bsp));
}

static inline struct structure_node *structure_bsp_get_node(struct structure_bsp *structure_bsp, int32_t node_index) {
    return tag_block_get_element_with_size(&structure_bsp->nodes, node_index, sizeof(struct structure_node));
}

static inline struct structure_leaf *structure_bsp_get_leaf(struct structure_bsp *structure_bsp, int32_t leaf_index) {
    return tag_block_get_element_with_size(&structure_bsp->leaves, leaf_index, sizeof(struct structure_leaf));
}

static inline struct structure_surface *structure_bsp_get_surface(struct structure_bsp *structure_bsp, int32_t surface_index) {
    return tag_block_get_element_with_size(&structure_bsp->surfaces, surface_index, sizeof(struct structure_surface));
}

static inline struct structure_surface_reference *structure_bsp_get_surface_reference(struct structure_bsp *structure_bsp, int32_t surface_reference_index) {
    return tag_block_get_element_with_size(&structure_bsp->surface_references, surface_reference_index, sizeof(struct structure_surface_reference));
}

static inline struct structure_lightmap *structure_bsp_get_lightmap(struct structure_bsp *structure_bsp, int32_t lightmap_index) {
    return tag_block_get_element_with_size(&structure_bsp->lightmaps, lightmap_index, sizeof(struct structure_lightmap));
}

static inline struct structure_material *structure_bsp_lightmap_get_material(struct structure_lightmap *lightmap, int32_t material_index) {
    return tag_block_get_element_with_size(&lightmap->materials, material_index, sizeof(struct structure_material));
}

static inline int32_t structure_bsp_get_unlit_lightmap_index(struct structure_bsp *structure_bsp) {
    return structure_bsp->lightmaps.count -1;
}

static inline struct environment_vertex_compressed *structure_bsp_material_get_compressed_vertex(struct structure_material *material, int32_t index) {
    struct environment_vertex_compressed *compressed_environment_vertex_data = tag_data_get_address(&material->compressed_vertex_data);
    return &compressed_environment_vertex_data[index];
}

static inline struct environment_lightmap_vertex_compressed *structure_bsp_material_get_compressed_lightmap_vertex(struct structure_material *material, int32_t index) {
    uint8_t *data = tag_data_get_address(&material->compressed_vertex_data);
    size_t compressed_environment_vertex_data_size = material->vertices.count * sizeof(struct environment_vertex_compressed);
    auto compressed_lightmap_vertex_data = (struct environment_lightmap_vertex_compressed *)(data + compressed_environment_vertex_data_size);
    return &compressed_lightmap_vertex_data[index];
}

static inline struct structure_lens_flare *structure_bsp_get_lens_flare(struct structure_bsp *structure_bsp, int32_t lens_flare_index) {
    return tag_block_get_element_with_size(&structure_bsp->lens_flares, lens_flare_index, sizeof(struct structure_lens_flare));
}

static inline struct structure_lens_flare_marker *structure_bsp_get_lens_flare_marker(struct structure_bsp *structure_bsp, int32_t lens_flare_marker_index) {
    return tag_block_get_element_with_size(&structure_bsp->lens_flare_markers, lens_flare_marker_index, sizeof(struct structure_lens_flare_marker));
}

static inline struct structure_cluster *structure_bsp_get_cluster(struct structure_bsp *structure_bsp, int32_t cluster_index) {
    return tag_block_get_element_with_size(&structure_bsp->clusters, cluster_index, sizeof(struct structure_cluster));
}

static inline struct cluster_portal *structure_bsp_get_cluster_portal(struct structure_bsp *structure_bsp, int32_t cluster_portal_index) {
    return tag_block_get_element_with_size(&structure_bsp->cluster_portals, cluster_portal_index, sizeof(struct cluster_portal));
}

static inline int16_t *structure_bsp_cluster_get_portal_index(struct structure_cluster *cluster, int32_t index) {
    return tag_block_get_element_with_size(&cluster->portal_indices, index, sizeof(int16_t));
}

static inline real_point3d *structure_bsp_cluster_portal_get_vertex(struct cluster_portal *portal, int32_t index) {
    return tag_block_get_element_with_size(&portal->vertices, index, sizeof(real_point3d));
}

uint32_t *structure_bsp_get_cluster_pvs(struct structure_bsp *structure_bsp, int16_t cluster_index);

static inline struct structure_mirror *structure_bsp_cluster_get_mirror(struct structure_cluster *cluster, int32_t mirror_index) {
    return tag_block_get_element_with_size(&cluster->mirrors, mirror_index, sizeof(struct structure_mirror));
}

static inline real_point3d *structure_bsp_mirror_get_point(struct structure_mirror *mirror, int32_t point_index) {
    return tag_block_get_element_with_size(&mirror->points, point_index, sizeof(real_point3d));
}

static inline int32_t *structure_bsp_cluster_get_surface_index(struct structure_cluster *cluster, int32_t surface_index) {
    return tag_block_get_element_with_size(&cluster->surface_indices, surface_index, sizeof(int32_t));
}

static inline struct structure_subcluster *structure_bsp_cluster_get_subcluster(struct structure_cluster *cluster, int32_t subcluster_index) {
    return tag_block_get_element_with_size(&cluster->subclusters, subcluster_index, sizeof(struct structure_subcluster));
}

static inline int32_t *structure_bsp_subcluster_get_surface_index(struct structure_subcluster *subcluster, int32_t surface_index) {
    return tag_block_get_element_with_size(&subcluster->surface_indices, surface_index, sizeof(int32_t));
}

static inline struct structure_breakable_surface *structure_bsp_get_breakable_surface(struct structure_bsp *structure_bsp, int32_t breakable_surface_index) {
    return tag_block_get_element_with_size(&structure_bsp->breakable_surfaces, breakable_surface_index, sizeof(struct structure_breakable_surface));
}

static inline struct structure_weather_polyhedron *structure_bsp_get_weather_polyhedron(struct structure_bsp *structure_bsp, int32_t weather_polyhedron_index) {
    return tag_block_get_element_with_size(&structure_bsp->weather_polyhedra, weather_polyhedron_index, sizeof(struct structure_weather_polyhedron));
}

static inline real_plane3d *structure_bsp_weather_polyhedron_get_plane(struct structure_weather_polyhedron *polyhedron, int32_t plane_index) {
    return tag_block_get_element_with_size(&polyhedron->planes, plane_index, sizeof(real_plane3d));
}

static inline bool structure_bsp_fog_index_is_inside(int16_t index) {
    return TEST_FLAG(index, INT16_BITS - 1);
}

static inline struct structure_fog_plane *structure_bsp_get_fog_plane(struct structure_bsp *structure_bsp, int32_t fog_plane_index) {
    return tag_block_get_element_with_size(&structure_bsp->fog_planes, fog_plane_index, sizeof(struct structure_fog_plane));
}

static inline real_point3d *structure_fog_plane_get_vertex(struct structure_fog_plane *fog_plane, int32_t vertex_index) {
    return tag_block_get_element_with_size(&fog_plane->vertices, vertex_index, sizeof(real_point3d));
}

static inline struct structure_fog_region *structure_bsp_get_fog_region(struct structure_bsp *structure_bsp, int32_t fog_region_index) {
    return tag_block_get_element_with_size(&structure_bsp->fog_regions, fog_region_index, sizeof(struct structure_fog_region));
}

static inline struct structure_sound_environment_palette_entry *structure_bsp_get_sound_environment_palette_entry(struct structure_bsp *structure_bsp, int32_t sound_environment_palette_index) {
    return tag_block_get_element_with_size(&structure_bsp->sound_environment_palette, sound_environment_palette_index, sizeof(struct structure_sound_environment_palette_entry));
}

static inline struct structure_background_sound_palette_entry *structure_bsp_get_background_sound_palette_entry(struct structure_bsp *structure_bsp, int32_t background_sound_palette_index) {
    return tag_block_get_element_with_size(&structure_bsp->background_sound_palette, background_sound_palette_index, sizeof(struct structure_background_sound_palette_entry));
}

static inline struct structure_fog_palette_entry *structure_bsp_get_fog_palette_entry(struct structure_bsp *structure_bsp, int32_t fog_palette_index) {
    return tag_block_get_element_with_size(&structure_bsp->fog_palette, fog_palette_index, sizeof(struct structure_fog_palette_entry));
}

static inline struct structure_weather_palette_entry *structure_bsp_get_weather_palette_entry(struct structure_bsp *structure_bsp, int32_t weather_palette_index) {
    return tag_block_get_element_with_size(&structure_bsp->weather_palette, weather_palette_index, sizeof(struct structure_weather_palette_entry));
}

uint8_t *structure_bsp_get_cluster_encoded_sound_data(struct structure_bsp *structure_bsp, int16_t row_index, int16_t column_index);

static inline int16_t structure_bsp_encoded_sound_data_row_offset(struct structure_bsp *structure_bsp, int16_t row_index) {
    return (structure_bsp->clusters.count * row_index) - (row_index * (row_index + 1) / 2);
}

#define MAXIMUM_ENCODED_SOUND_DISTANCE 256.0f

static inline uint8_t sound_distance_encode(real distance, bool ai_deafening) {
    if(distance > MAXIMUM_ENCODED_SOUND_DISTANCE) {
        return UINT8_MAX;
    }

    uint8_t encoded_distance = PIN(distance * INT8_MAX / MAXIMUM_ENCODED_SOUND_DISTANCE, 0, INT8_MAX);
    if(ai_deafening) {
        encoded_distance |= FLAG(INT8_BITS - 1);
    }

    return encoded_distance;
}

static inline real sound_distance_decode(uint8_t encoded_distance) {
    return (encoded_distance & ~FLAG(INT8_BITS - 1)) * MAXIMUM_ENCODED_SOUND_DISTANCE / INT8_MAX;
}

static inline bool sound_distance_decode_ai_deafening(uint8_t encoded_distance) {
    return TEST_FLAG(encoded_distance, INT8_BITS - 1);
}

static inline struct structure_marker *structure_bsp_get_marker(struct structure_bsp *structure_bsp, int32_t marker_index) {
    return tag_block_get_element_with_size(&structure_bsp->markers, marker_index, sizeof(struct structure_marker));
}

void structure_bsp_find_material_for_surface(struct structure_bsp *structure, int32_t surface_index, int16_t *lightmap_index, int16_t *material_index);
void vertex_type_from_shader_tag(tag group_tag, int16_t *vertex_type, int16_t *lightmap_vertex_type, bool compressed);

uint8_t structure_bsp_get_cluster_encoded_sound_distance(struct structure_bsp *structure_bsp, int16_t from_cluster_index, int16_t to_cluster_index);

static inline struct structure_detail_object_data *structure_bsp_get_detail_object_data(struct structure_bsp *structure_bsp) {
    if(!structure_bsp->detail_object_data.count) {
        return nullptr;
    }

    return tag_block_get_element_with_size(&structure_bsp->detail_object_data, 0, sizeof(struct structure_detail_object_data));
}

static inline struct detail_object_cell_definition *structure_bsp_detail_object_data_get_cell(struct structure_detail_object_data *data, int32_t cell_index) {
    return tag_block_get_element_with_size(&data->cells, cell_index, sizeof(struct detail_object_cell_definition));
}

static inline struct detail_object *structure_bsp_detail_object_data_get_detail_object(struct structure_detail_object_data *data, int32_t detail_object_index) {
    return tag_block_get_element_with_size(&data->detail_objects, detail_object_index, sizeof(struct detail_object));
}

static inline uint16_t *structure_bsp_detail_object_data_get_detail_object_count(struct structure_detail_object_data *data, int32_t detail_object_count_index) {
    return tag_block_get_element_with_size(&data->detail_objects_counts, detail_object_count_index, sizeof(uint16_t));
}

static inline real_vector4d *structure_bsp_detail_object_data_get_z_reference_vector(struct structure_detail_object_data *data, int32_t z_reference_vector_index) {
    return tag_block_get_element_with_size(&data->detail_object_z_reference_vectors, z_reference_vector_index, sizeof(real_vector4d));
}

static inline struct structure_runtime_decal *structure_bsp_runtime_decal_get(struct structure_bsp *structure_bsp, int32_t runtime_decal_index) {
    return tag_block_get_element_with_size(&structure_bsp->runtime_decals, runtime_decal_index, sizeof(struct structure_runtime_decal));
}

#endif
