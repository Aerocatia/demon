#include <stdint.h>
#include <string.h>

#include "../cseries/build_number.h"
#include "../cseries/cseries.h"
#include "../tag_files/tag_groups.h"
#include "../memory/data.h"

#include "cache_files.h"

// TODO: This is not used to get the path for resource maps, they will always be loaded from `maps`
char *cache_files_map_directory() {
    return "maps\\";
}

#ifdef REQUIRE_CACHE_FILE

asm(".set _cache_file_globals, 0x00AF8368"); // TODO: make static
extern struct {
    bool tags_loaded;
    bool unknown_bool; // set this to false and you get EXCEPTION halt in memory\data.c,#532: pc texture index #0 (0xe3700000) is unused or changed
    struct cache_file_header header;
    uint32_t unknown; // set to 0 and never read
    struct cache_file_tags_header *tags_header;
    struct cache_file_structure_bsp_header *structure_bsp_header;
}cache_file_globals;

asm(".set _global_tag_instances, 0x00AF8364"); // TODO: remove extern
extern struct cache_file_tag_instance *global_tag_instances;

uint32_t tag_groups_checksum() {
    assert(cache_file_globals.tags_loaded);
    return cache_file_globals.tags_header->tags_checksum;
}

uint32_t cache_files_get_checksum() {
    return cache_file_globals.header.checksum;
}

int32_t tag_loaded(tag group_tag, const char *name) {
    if(!cache_file_globals.tags_loaded) {
        return NONE;
    }

    assert(global_tag_instances);

    // find this tag in the index
    int32_t tag_index = NONE;
    for(int index = 0; index < cache_file_globals.tags_header->tag_count; ++index) {
        if(group_tag == global_tag_instances[index].group_tag && !stricmp(name, global_tag_instances[index].name)) {
            tag_index = global_tag_instances[index].tag_index;
            break;
        }
    }

    return tag_index;
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

static struct cache_file_tag_instance *cache_file_tag_instance_get(int32_t tag_index) {
    int16_t absolute_index = DATUM_INDEX_TO_ABSOLUTE_INDEX(tag_index);
    struct cache_file_tag_instance *tag_instance;

    assert(cache_file_globals.tags_loaded);
    assert(global_tag_instances);

    vassert(absolute_index >= 0 && absolute_index < cache_file_globals.tags_header->tag_count,
        csprintf(temporary, "i don't think %08x is a tag index", tag_index));
    tag_instance = &global_tag_instances[absolute_index];

    vassert(!DATUM_INDEX_TO_IDENTIFIER(tag_index) || tag_instance->tag_index == tag_index,
        csprintf(temporary, "i don't think %08x is a tag index", tag_index));

    return tag_instance;
}

#endif
