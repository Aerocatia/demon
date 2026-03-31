#ifndef __CACHE_FILES_H__
#define __CACHE_FILES_H__

#include <stdint.h>

#define CACHE_FILE_HEADER_SIGNATURE 0x68656164 // head
#define CACHE_FILE_FOOTER_SIGNATURE 0x666F6F74 // foot
#define CACHE_FILE_VERSION_RETAIL 7
#define CACHE_FILE_VERSION_CUSTOM_EDITION 609
#define CACHE_FILE_VERSION CACHE_FILE_VERSION_CUSTOM_EDITION

#define CACHE_FILE_MAXIMUM_SIZE 1 * GIG // was (384 * MEG)
#define CACHE_FILE_MAXIMUM_SOLO_SIZE CACHE_FILE_MAXIMUM_SIZE
#define CACHE_FILE_MAXIMUM_UI_SIZE CACHE_FILE_MAXIMUM_SIZE // was (35 * MEG)
#define CACHE_FILE_MAXIMUM_MULTIPLAYER_SIZE CACHE_FILE_MAXIMUM_SIZE // was (128 * MEG)

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

struct cache_file_tag_instance {
    int32_t group_tag;
    int32_t parent_group_tags[MAXIMUM_PARENT_GROUP_TYPES_PER_TAG];

    int32_t tag_index;

    char *name;
    void *base_address;
    uint32_t flags;
    uint32_t unused;
};
static_assert(sizeof(struct cache_file_tag_instance) == 32);

#define CACHE_FILE_TAGS_HEADER_SIGNATURE 0x74616773 // tags

struct cache_file_tags_header {
    struct cache_file_tag_instance *tag_instances; // should immediately follow header

    int32_t scenario_tag_index;

    uint32_t tags_checksum;
    int32_t tag_count;

    int32_t vertex_buffer_count;
    int32_t vertex_buffers_offset; // file offset
    int32_t index_buffer_count;
    int32_t index_buffers_offset; // offset from vertex buffers
    int32_t model_data_size;

    tag signature;
};
static_assert(sizeof(struct cache_file_tags_header) == 40);

#define CACHE_FILE_STRUCTURE_BSP_HEADER_SIGNATURE 0x73627370 // sbsp

struct cache_file_structure_bsp_header {
    struct structure_bsp *structure_bsp; // should immediately follow header

    int32_t vertex_buffer_count;
    void *vertex_buffers; // D3DVertexBuffer

    int32_t lightmap_vertex_buffer_count;
    void *lightmap_vertex_buffers; // D3DVertexBuffer

    tag signature;
};
static_assert(sizeof(struct cache_file_structure_bsp_header) == 24);

uint32_t cache_files_get_checksum();

#endif
