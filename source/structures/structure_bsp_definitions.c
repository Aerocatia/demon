#include <stdint.h>

#include "../tag_files/tag_groups.h"

#include "structure_bsp_definitions.h"

uint32_t *structure_bsp_get_cluster_pvs(struct structure_bsp *structure_bsp, int16_t cluster_index) {
    assert(cluster_index >= 0 && cluster_index < structure_bsp->clusters.count);
    assert((cluster_index + 1) * BIT_VECTOR_SIZE_IN_LONGS(structure_bsp->clusters.count) <= structure_bsp->cluster_data.size);

    return (uint32_t *)structure_bsp->cluster_data.address + cluster_index * BIT_VECTOR_SIZE_IN_LONGS(structure_bsp->clusters.count);
}

void structure_bsp_find_material_for_surface(struct structure_bsp *structure, int32_t surface_index, int16_t *lightmap_index, int16_t *material_index) {
    int16_t first_lightmap_index = *lightmap_index = 0;
    int16_t last_lightmap_index = structure->lightmaps.count - 1;
    while(last_lightmap_index > first_lightmap_index) {
        *lightmap_index = first_lightmap_index + (last_lightmap_index - first_lightmap_index) / 2;
        auto lightmap = structure_bsp_get_lightmap(structure, *lightmap_index);
        if(surface_index < structure_bsp_lightmap_get_material(lightmap, 0)->first_surface_index) {
            last_lightmap_index = --*lightmap_index;
        }
        else {
            auto material = structure_bsp_lightmap_get_material(lightmap, lightmap->materials.count - 1);
            if(surface_index >= material->first_surface_index + material->surface_count) {
                first_lightmap_index = ++*lightmap_index;
            }
            else {
                break;
            }
        }
    }

    auto lightmap = structure_bsp_get_lightmap(structure, *lightmap_index);
    int16_t first_material_index = *material_index = 0;
    int16_t last_material_index = lightmap->materials.count;
    while(first_material_index < last_material_index) {
        *material_index = first_material_index + (last_material_index - first_material_index) / 2;
        auto material = structure_bsp_lightmap_get_material(lightmap, *material_index);
        if(surface_index < material->first_surface_index) {
            last_material_index = --*material_index;
        }
        else if(surface_index >= material->first_surface_index + material->surface_count) {
            first_material_index = ++*material_index;
        }
        else {
            break;
        }
    }

#ifdef DEBUG_BUILD
    auto material = structure_bsp_lightmap_get_material(lightmap, *material_index);
    assert(surface_index >= material->first_surface_index);
    assert(surface_index < material->first_surface_index + material->surface_count);
#endif
}

void vertex_type_from_shader_tag(tag, int16_t *vertex_type, int16_t *lightmap_vertex_type, bool compressed) {
    if(compressed) {
        *vertex_type = _rasterizer_vertex_type_environment_compressed;
        *lightmap_vertex_type = _rasterizer_vertex_type_environment_lightmap_compressed;
    }
    else {
        *vertex_type = _rasterizer_vertex_type_environment_uncompressed;
        *lightmap_vertex_type = _rasterizer_vertex_type_environment_lightmap_uncompressed;
    }
}

uint8_t *structure_bsp_get_cluster_encoded_sound_data(struct structure_bsp *structure_bsp, int16_t row_index, int16_t column_index) {
	assert(row_index < column_index);
	int16_t offset = structure_bsp_encoded_sound_data_row_offset(structure_bsp, row_index) + column_index - row_index - 1;
	assert(offset >= 0 && offset < structure_bsp->sound_cluster_data.size);

    uint8_t *address = tag_data_get_address(&structure_bsp->sound_cluster_data);
	return address + offset;
}

uint8_t structure_bsp_get_cluster_encoded_sound_distance(struct structure_bsp *structure_bsp, int16_t from_cluster_index, int16_t to_cluster_index) {
    assert(from_cluster_index >= 0 && from_cluster_index < structure_bsp->clusters.count);
    assert(to_cluster_index >= 0 && to_cluster_index < structure_bsp->clusters.count);

    uint8_t result = 0;
    if(from_cluster_index != to_cluster_index) {
        if(from_cluster_index > to_cluster_index) {
            int16_t swap = from_cluster_index;
            from_cluster_index = to_cluster_index;
            to_cluster_index = swap;
        }

        result = *structure_bsp_get_cluster_encoded_sound_data(structure_bsp, from_cluster_index, to_cluster_index);
    }

    return result;
}
