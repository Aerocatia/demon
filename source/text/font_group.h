#ifndef DEMON_FONT_GROUP_H
#define DEMON_FONT_GROUP_H

#include <stdint.h>

#include "../tag_files/tag_groups.h"
#include "draw_string.h"

#define HIGH_BYTE(c) (((c) >> INT8_BITS) & 0xFF)
#define LOW_BYTE(c) ((c) & 0xFF)

enum {
    FONT_GROUP_TAG = 0x666F6E74 // 'font'
};

struct font_character {
    uint16_t character;
    int16_t character_width;
    int16_t bitmap_width;
    int16_t bitmap_height;
    int16_t bitmap_origin_x;
    int16_t bitmap_origin_y;
    int16_t hardware_character_index;
    uint16_t pad;
    int32_t pixels_offset;
};
static_assert(sizeof(struct font_character) == 20);

struct font_character_table_entry {
    int16_t character_index;
};
static_assert(sizeof(struct font_character_table_entry) == 2);

struct font_character_tables_entry {
    struct tag_block table;
};
static_assert(sizeof(struct font_character_tables_entry) == 12);

struct font_header {
    uint32_t flags;
    int16_t ascending_height;
    int16_t descending_height;
    int16_t leading_height;
    int16_t leading_width;
    int32_t pad[9];
    struct tag_block character_tables;
    struct tag_reference style_fonts[NUMBER_OF_TEXT_STYLES];
    struct tag_block characters;
    struct tag_data pixels;
};
static_assert(sizeof(struct font_header) == 156);

/* font group functions */

static inline struct font_header *font_get_header(int32_t tag_index) {
    return tag_get(FONT_GROUP_TAG, tag_index);
}

static inline struct font_character *font_get_character(struct font_header *header, int32_t character_index) {
    return tag_block_get_element_with_size(&header->characters, character_index, sizeof(struct font_character));
}

static inline struct font_character_tables_entry *font_get_character_tables_entry(struct font_header *header, int32_t high_byte) {
    return tag_block_get_element_with_size(&header->character_tables, high_byte, sizeof(struct font_character_tables_entry));
}

static inline struct font_character_table_entry *font_get_character_table_entry(struct font_character_tables_entry *table_entry, int32_t low_byte) {
    if(table_entry->table.count != UINT8_MAX + 1) {
        return nullptr;
    }

    return tag_block_get_element_with_size(&table_entry->table, low_byte, sizeof(struct font_character_table_entry));
}

struct font_character *font_get_character_by_ascii_code(struct font_header *header, uint16_t character);

#endif
