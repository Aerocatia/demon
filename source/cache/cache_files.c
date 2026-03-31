#include <stdint.h>
#include <string.h>

#include "../cseries/build_number.h"
#include "../cseries/cseries.h"
#include "../tag_files/tag_groups.h"

#include "cache_files.h"

asm(".set _cache_file_globals, 0x00AF8368"); // static
extern struct {
    bool tags_loaded;
    bool unknown; // set this to false and you get EXCEPTION halt in memory\data.c,#532: pc texture index #0 (0xe3700000) is unused or changed
    struct cache_file_header header;
    struct cache_file_tags_header *tags_header;
    struct cache_file_structure_bsp_header *structure_bsp_header;
}cache_file_globals;

asm(".set _cache_file_tag_instance, 0x00AF8364"); // global
extern struct cache_file_tag_instance *global_tag_instances;

uint32_t tag_groups_checksum() {
    assert(cache_file_globals.tags_loaded);
    return cache_file_globals.tags_header->tags_checksum;
}

uint32_t cache_files_get_checksum() {
    return cache_file_globals.header.checksum;
}

// TODO: This hack will allow both retail and custom edition map types to run, but the game will create a zero byte loc.map if one does not exist
static bool cache_file_valid_version(int32_t version) {
    if(version == CACHE_FILE_VERSION_RETAIL || version == CACHE_FILE_VERSION_CUSTOM_EDITION) {
        return true;
    }

    return false;
}

bool cache_file_header_verify(struct cache_file_header *header, [[maybe_unused]] const char *name, bool fatal) {
    if(header->header_signature != CACHE_FILE_HEADER_SIGNATURE ||
        header->footer_signature != CACHE_FILE_FOOTER_SIGNATURE ||
        header->size < 0 ||
        header->size > CACHE_FILE_MAXIMUM_SIZE ||
        strlen(header->name) > TAG_STRING_LENGTH
    ) {
        if(fatal) {
            vhalt(csprintf(temporary, "'%s' does not appear to be a cache file", name));
        };
    }
    else if(!cache_file_valid_version(header->version)) {
        if(fatal) {
            vhalt(csprintf(temporary, "the cache file '%s' is an unsupported version (%d)", name, header->version));
        };
    }
    else {
        return true;
    }

    return false;
}
