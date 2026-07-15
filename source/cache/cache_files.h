#ifndef DEMON_CACHE_FILES_H
#define DEMON_CACHE_FILES_H

#include <stdint.h>
#include "../cseries/cseries.h"
#include "../tag_files/tag_groups.h"

#define CACHE_FILE_HEADER_SIGNATURE 0x68656164 // head
#define CACHE_FILE_FOOTER_SIGNATURE 0x666F6F74 // foot
#define CACHE_FILE_VERSION_RETAIL 7
#define CACHE_FILE_VERSION_CUSTOM_EDITION 609
#define CACHE_FILE_VERSION CACHE_FILE_VERSION_CUSTOM_EDITION

#define CACHE_FILE_MAXIMUM_SIZE 1 * GIB // was (384 * MIB)
#define CACHE_FILE_MAXIMUM_SOLO_SIZE CACHE_FILE_MAXIMUM_SIZE
#define CACHE_FILE_MAXIMUM_UI_SIZE CACHE_FILE_MAXIMUM_SIZE // was (35 * MIB)
#define CACHE_FILE_MAXIMUM_MULTIPLAYER_SIZE CACHE_FILE_MAXIMUM_SIZE // was (128 * MIB)

struct cache_file_header {
    int32_t header_signature;
    int32_t version;
    int32_t size;
    int32_t compressed_file_padding;
    int32_t tags_offset, tags_size;
    int32_t index_buffer_count;
    int32_t index_buffers_offset;
    char name[TAG_STRING_LENGTH + 1];
    char build_number[TAG_STRING_LENGTH + 1];
    int16_t scenario_type;
    uint16_t pad;
    uint32_t checksum;
    uint32_t unused2[485];
    int32_t footer_signature;
};
static_assert(sizeof(struct cache_file_header) == 2048);

enum {
    _cache_file_tag_instance_flags_tag_in_data_file_bit,
    NUMBER_OF_CACHE_FILE_TAG_INSTANCE_FLAGS
};

struct cache_file_tag_instance {
    tag group_tag;
    tag parent_group_tags[MAXIMUM_PARENT_GROUP_TYPES_PER_TAG];
    int32_t tag_index;
    char *name;
    void *base_address;
    uint32_t flags;
    uint32_t unused;
};
static_assert(sizeof(struct cache_file_tag_instance) == 32);

#define CACHE_FILE_TAGS_HEADER_SIGNATURE 0x74616773 // tags

struct cache_file_tags_header {
    struct cache_file_tag_instance *tag_instances;
    int32_t scenario_tag_index;
    uint32_t tags_checksum;
    int32_t tag_count;
    int32_t vertex_buffer_count;
    int32_t vertex_buffers_offset; // file offset (was void *vertex_buffers)
    int32_t index_buffer_count;
    int32_t index_buffers_offset; // offset from vertex buffers (was void *index_buffers)
    int32_t vertex_index_buffer_size;
    tag signature;
};
static_assert(sizeof(struct cache_file_tags_header) == 40);

#define CACHE_FILE_STRUCTURE_BSP_HEADER_SIGNATURE 0x73627370 // sbsp

struct cache_file_structure_bsp_header_xbox {
    struct structure_bsp *structure_bsp;
    int32_t vertex_buffer_count;
    void *vertex_buffers;
    int32_t lightmap_vertex_buffer_count;
    void *lightmap_vertex_buffers;
    tag signature;
};
static_assert(sizeof(struct cache_file_structure_bsp_header_xbox) == 24);

struct cache_file_structure_bsp_header {
    struct structure_bsp *structure_bsp;
    int32_t vertex_buffer_size;
    int32_t vertex_buffer_file_ofs;
    uint32_t dummy[2];
    tag signature;
};
static_assert(sizeof(struct cache_file_structure_bsp_header) == 24);

struct cache_file_read_request_params {
    volatile bool *finished_flag;
    void (*finished_func)(struct cache_file_read_request_params *);
    void *userdata;
};
static_assert(sizeof(struct cache_file_read_request_params) == 12);

bool cache_file_open(const char *scenario_name, struct cache_file_header *header);
int16_t cache_file_read(int32_t tag_index, int32_t offset, int32_t size, void *buffer, struct cache_file_read_request_params *params, bool blocking, bool data_file);

void cache_files_set_root_directory(const char *root_directory);
const char *cache_files_root_directory();
char *cache_files_map_directory();

uint32_t cache_files_get_checksum();
bool cache_file_header_verify(struct cache_file_header *header, [[maybe_unused]] const char *name, bool fatal);

void tags_header_register_vertex_and_index_buffers(struct cache_file_tags_header *tags_header);

#endif
