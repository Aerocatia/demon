#ifndef DEMON_TAG_GROUPS_H
#define DEMON_TAG_GROUPS_H

#include <stdint.h>

#include "../cseries/cseries.h"
#include "../memory/byte_swapping.h"
#include "../memory/data.h"

#define MAXIMUM_PARENT_GROUP_TYPES_PER_TAG 2
#define MAXIMUM_SIMULTANEOUS_TAG_INSTANCES 4000

enum {
    TAG_STRING_LENGTH = 31
};

enum {
    _tag_data_flags_external_bit, // if the data is in a resource map (only used by sounds)
    NUMBER_OF_TAG_DATA_FLAGS
};

struct tag_block;

typedef void (*byte_swap_block_proc)(void *header);
typedef bool (*postprocess_block_proc)(void *element, bool editing);
typedef char *(*format_block_proc)(int32_t tag_index, struct tag_block *block, int32_t element_index, char *buffer);
typedef void (*delete_block_proc)(struct tag_block *block, int32_t element_index);

struct tag_block_definition {
    char *name;
    uint32_t flags;
    int32_t maximum_element_count;
    int32_t element_size;
    void *default_element;
    struct tag_field *fields;
    byte_swap_block_proc byte_swap_block;
    postprocess_block_proc postprocess_block;
    format_block_proc format_block;
    delete_block_proc delete_block;
    byte_swap_code *byte_swap_codes;
};

struct tag_block {
    int32_t count;
    void *address;
    struct tag_block_definition *definition;
};
static_assert(sizeof(struct tag_block) == 12);

typedef void (*byte_swap_data_proc)(void *block_element, void *data, int32_t size);

struct tag_data_definition {
    char *name;
    uint32_t flags;
    int32_t maximum_size;
    byte_swap_data_proc byte_swap_data;
};
static_assert(sizeof(struct tag_data_definition) == 16);

struct tag_data {
    int32_t size;
    uint32_t flags;
    int32_t file_offset;
    void *address;
    struct tag_data_definition *definition;
};
static_assert(sizeof(struct tag_data) == 20);

struct tag_reference {
    tag group_tag;
    char *name;
    int32_t name_length;
    int32_t index;
};
static_assert(sizeof(struct tag_reference) == 16);

void *tag_block_get_element_with_size(const struct tag_block *block, int32_t index, int32_t element_size);
void *tag_data_get_address(const struct tag_data *data);
void *tag_data_get_pointer(const struct tag_data *data, int32_t offset, int32_t size);
void *tag_get(tag group_tag, int32_t tag_index);
char *tag_get_name(int32_t tag_index);

struct tag_iterator {
    struct data_iterator iterator;
    tag key_group_tag;
};

void tag_iterator_new(struct tag_iterator *iterator, tag key_group_tag);
int32_t tag_iterator_next(struct tag_iterator *iterator);

#endif
