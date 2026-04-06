#include <stdint.h>

#include "../tag_files/tag_groups.h"

#include "structure_bsp_definitions.h"

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
