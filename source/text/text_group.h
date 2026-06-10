#ifndef DEMON_TEXT_GROUP_H
#define DEMON_TEXT_GROUP_H

#include <uchar.h>

#include "../tag_files/tag_groups.h"
#include "unicode.h"

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

#define string_list_get_header(index) TAG_GET(STRING_LISTS_GROUP_TAG, (index), struct string_list_group_header)
#define string_list_get_string_reference(header, index) (TAG_BLOCK_GET_ELEMENT(&(header)->string_references, (index), struct string_list_string_reference))

const char *string_list_get_string(int32_t tag_index, int16_t string_index);

struct unicode_string_list_string_reference {
    struct tag_data string;
};
static_assert(sizeof(struct unicode_string_list_string_reference) == 20);

struct unicode_string_list_group_header {
    struct tag_block string_references;
};
static_assert(sizeof(struct unicode_string_list_group_header) == 12);

#define unicode_string_list_get_header(index) TAG_GET(UNICODE_STRING_LISTS_GROUP_TAG, (index), struct unicode_string_list_group_header)
#define unicode_string_list_get_string_reference(header, index) (TAG_BLOCK_GET_ELEMENT(&(header)->string_references, (index), struct unicode_string_list_string_reference))

const char16_t *unicode_string_list_get_string(int32_t tag_index, int16_t string_index);

#endif
