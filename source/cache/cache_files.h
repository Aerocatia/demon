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

#endif
