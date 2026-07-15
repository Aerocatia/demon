#ifndef DEMON_TEXT_GROUP_H
#define DEMON_TEXT_GROUP_H

#include <uchar.h>

#include "../tag_files/tag_groups.h"
#include "unicode.h"

/* ascii */

enum {
    STRING_LISTS_GROUP_TAG = 0x73747223,
    UNICODE_STRING_LISTS_GROUP_TAG = 0x75737472
};

struct string_list_string_reference {
    struct tag_data string;
};
static_assert(sizeof(struct string_list_string_reference) == 20);

struct string_list_group_header {
    struct tag_block string_references;
};
static_assert(sizeof(struct string_list_group_header) == 12);

/* ascii functions */

static inline struct string_list_group_header *string_list_get_header(int32_t index) {
    return tag_get(STRING_LISTS_GROUP_TAG, index);
}

static inline struct string_list_string_reference *string_list_get_string_reference(struct string_list_group_header *header, int32_t index) {
    return tag_block_get_element_with_size(&header->string_references, index, sizeof(struct string_list_string_reference));
}

const char *string_list_get_string(int32_t tag_index, int16_t string_index);

/* unicode */

struct unicode_string_list_string_reference {
    struct tag_data string;
};
static_assert(sizeof(struct unicode_string_list_string_reference) == 20);

struct unicode_string_list_group_header {
    struct tag_block string_references;
};
static_assert(sizeof(struct unicode_string_list_group_header) == 12);

/* unicode functions */

static inline struct unicode_string_list_group_header *unicode_string_list_get_header(int32_t index) {
    return tag_get(UNICODE_STRING_LISTS_GROUP_TAG, index);
}

static inline struct unicode_string_list_string_reference *unicode_string_list_get_string_reference(struct unicode_string_list_group_header *header, int32_t index) {
    return tag_block_get_element_with_size(&header->string_references, index, sizeof(struct unicode_string_list_string_reference));
}

const char16_t *unicode_string_list_get_string(int32_t tag_index, int16_t string_index);

#endif
