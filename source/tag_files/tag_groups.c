#include <stdint.h>

#include "../cseries/cseries.h"
#include "tag_groups.h"

void *tag_data_get_address(const struct tag_data *data) {
    assert(data);
    return data->address;
}

void *tag_data_get_pointer(const struct tag_data *data, int32_t offset, [[maybe_unused]] int32_t size) {
    assert(data && size >= 0);
    assert(offset >= 0 && offset + size <= data->size);

    uint8_t *address = tag_data_get_address(data);
    // The exe does not check this and cheat_all_vehicles will cause it to be run on data with a null address.
    //assert(address);

    return address + offset;
}

void *tag_block_get_address(const struct tag_block *block) {
    assert(block);
    return block->address;
}

void *tag_block_get_element_with_size(const struct tag_block *block, int32_t index, int32_t element_size) {
    assert(block);
    assert(block->count >= 0);
    assert(!block->definition || block->definition->element_size == element_size);
    vassert(index >= 0 && index < block->count,
        csprintf(temporary, "#%d is not a valid %s index in [#0,#%d)", index,
            block->definition ? block->definition->name : "<unknown>", block->count));

    uint8_t *address = tag_block_get_address(block);
    assert(address);

    return address + index * element_size;
}
