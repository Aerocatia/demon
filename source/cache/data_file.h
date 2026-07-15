#ifndef DEMON_DATA_FILE_H
#define DEMON_DATA_FILE_H

#include <stdint.h>

enum {
    _data_file_type_none,
    _data_file_type_bitmap,
    _data_file_type_sound,
    _data_file_type_loc,
    NUMBER_OF_DATA_FILE_TYPES //was data_file_type_max
};

struct data_file_header_s {
    uint32_t data_file_id;
    uint32_t names_offset;
    uint32_t items_offset;
    uint32_t item_count;
};
static_assert(sizeof(struct data_file_header_s) == 16);

struct data_file_item_s {
    uint32_t name_offset;
    uint32_t data_size;
    uint32_t data_offset;
};
static_assert(sizeof(struct data_file_item_s) == 12);

struct data_file_build_stats_s {
    uint32_t items;
    uint32_t items_size;
};
static_assert(sizeof(struct data_file_build_stats_s) == 8);

struct data_file_s {
    struct data_file_header_s header;
    struct data_file_item_s *items;
    uint32_t max_item_count;
    uint32_t max_names_len;
    uint32_t cur_names_len;
    char *names;
    bool writable;
    struct data_file_build_stats_s hit_stats;
    struct data_file_build_stats_s miss_stats;
    const char *file_name;
    void *hFile;
};
static_assert(sizeof(struct data_file_s) == 64);

uint32_t data_file_find_item(int32_t data_file_id, const char *item_name);
uint32_t data_file_load_tag(int32_t data_file_id, uint32_t item_index, void *tag_load_address);

#endif
