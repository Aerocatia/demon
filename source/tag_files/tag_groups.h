#ifndef __TAG_GROUPS_H__
#define __TAG_GROUPS_H__

#include <stdint.h>

#include "../cseries/cseries.h"

#define MAXIMUM_PARENT_GROUP_TYPES_PER_TAG 2
#define MAXIMUM_SIMULTANEOUS_TAG_INSTANCES 4000

enum {
    TAG_STRING_LENGTH = 31,
    TAG_STRING_BUFFER_LENGTH
};

enum {
    _tag_data_flags_external_bit, // if the data is in a resource map (only used by sounds)
    NUMBER_OF_TAG_DATA_FLAGS
};

struct tag_block {
    int32_t count;
    void *address;
    void *definition; // should be struct tag_block_definition
};
static_assert(sizeof(struct tag_block) == 12);

struct tag_data {
    int32_t size;
    uint32_t flags;
    int32_t file_offset;
    void *address;
    void *definition; // should be struct tag_data_definition
};
static_assert(sizeof(struct tag_data) == 20);

struct tag_reference {
    tag group_tag;
    char *name;
    int32_t name_length;
    int32_t index;
};
static_assert(sizeof(struct tag_reference) == 16);

#define TAG_GET(group_tag, index, type) ((type *)tag_get((group_tag), (index)))
#define TAG_BLOCK_GET_ELEMENT(block, index, type) ((type *)tag_block_get_element_with_size(block, index, sizeof(type)))
#define TAG_DATA_GET_POINTER(data, offset, size) tag_data_get_pointer(data, offset, size)

void *tag_get(tag group_tag, int32_t tag_index);
void *tag_data_get_address(const struct tag_data *data);
void *tag_data_get_pointer(const struct tag_data *data, int32_t offset, int32_t size);
void *tag_block_get_element_with_size(const struct tag_block *block, int32_t index, int32_t element_size);

#endif
