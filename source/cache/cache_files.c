#include <stdint.h>
#include <string.h>
#include <stdio.h>

#include "../cseries/build_number.h"
#include "../cseries/cseries.h"
#include "../tag_files/files.h"
#include "../tag_files/tag_files.h"
#include "../tag_files/tag_groups.h"
#include "../memory/byte_swapping.h"
#include "../memory/data.h"
#include "../scenario/scenario_definitions.h"

#include "cache_files.h"
#include "sound_cache.h"
#include "texture_cache.h"
#include "physical_memory_map.h"

static char cache_root_directory[MAXIMUM_FILENAME_LENGTH + 1] = {};

void cache_files_set_root_directory(const char *root_directory) {
    assert(root_directory);
    strncpy(cache_root_directory, root_directory, sizeof(cache_root_directory));
    cache_root_directory[sizeof(cache_root_directory) - 1] = '\0';
}

const char *cache_files_root_directory() {
    return cache_root_directory;
}

// TODO: This is not used to get the path for resource maps, they will always be loaded from `maps`
char *cache_files_map_directory() {
    return "maps\\";
}

#ifdef CACHE_FILE_BUILD

asm(".set _cache_file_globals, 0x00AF8368"); // TODO: make static
extern struct {
    bool tags_loaded;
    bool unknown_bool; // set this to false and you get EXCEPTION halt in memory\data.c,#532: pc texture index #0 (0xe3700000) is unused or changed
    struct cache_file_header header;
    uint32_t unknown; // set to 0 and never read
    struct cache_file_tags_header *tags_header;
    struct cache_file_structure_bsp_header *structure_bsp_header;
} cache_file_globals;

asm(".set _global_tag_instances, 0x00AF8364"); // TODO: remove extern
extern struct cache_file_tag_instance *global_tag_instances;

static struct cache_file_tag_instance *cache_file_tag_instance_get(int32_t tag_index);

int32_t scenario_tags_load(const char *name) {
    assert(name);
    texture_cache_open();
    sound_cache_open();
    cache_file_globals.unknown_bool = true;
    const char *name_without_path = tag_name_strip_path(name);
    if(!cache_file_open(name_without_path, &cache_file_globals.header)) {
        return NONE;
    }

    if(!cache_file_header_verify(&cache_file_globals.header, name, true)) {
        return NONE;
    }

    void *tag_cache_base_address = physical_memory_get_tag_cache_base_address();
#ifdef DEBUG_BUILD
    memset(tag_cache_base_address, 0xCD, physical_memory_get_tag_cache_size());
#endif
    struct cache_file_read_request_params params = {};
    volatile bool finished_flag;
    params.finished_flag = &finished_flag;
    cache_file_read(NONE,
        cache_file_globals.header.tags_offset,
        cache_file_globals.header.tags_size,
        tag_cache_base_address,
        &params,
        true,
        false);

    while(!finished_flag) {
        system_sleep(0);
    }

    cache_file_globals.tags_header = tag_cache_base_address;
#ifdef DEBUG_BUILD
    vassert(cache_file_globals.tags_header->signature == CACHE_FILE_TAGS_HEADER_SIGNATURE,
        csprintf(temporary, "signature is '%c%c%c%c', should be '%c%c%c%c'",
            ((char*)&cache_file_globals.tags_header->signature)[3],
            ((char*)&cache_file_globals.tags_header->signature)[2],
            ((char*)&cache_file_globals.tags_header->signature)[1],
            ((char*)&cache_file_globals.tags_header->signature)[0],
            (CACHE_FILE_TAGS_HEADER_SIGNATURE >> 24) & 0xFF,
            (CACHE_FILE_TAGS_HEADER_SIGNATURE >> 16) & 0xFF,
            (CACHE_FILE_TAGS_HEADER_SIGNATURE >> 8)  & 0xFF,
            (CACHE_FILE_TAGS_HEADER_SIGNATURE)       & 0xFF));
#endif
    global_tag_instances = cache_file_globals.tags_header->tag_instances;
    cache_file_globals.tags_loaded = true;

    tags_header_register_vertex_and_index_buffers(cache_file_globals.tags_header);

    struct tag_iterator iterator;
    tag_iterator_new(&iterator, TAG_NONE);
    int32_t tag_index;
    while((tag_index = tag_iterator_next(&iterator)) != NONE) {
        struct cache_file_tag_instance *tag_instance = cache_file_tag_instance_get(tag_index);
        if(tag_instance->group_tag == SCENARIO_GROUP_TAG) {
            struct scenario *scenario = scenario_get(tag_index);
            struct data_array *scenario_hs_syntax_data = scenario->hs_syntax_data.address;
            scenario_hs_syntax_data->data = scenario_hs_syntax_data + 1;
        }

         // TODO: handle indexed tags here;
    }

    return cache_file_globals.tags_header->scenario_tag_index;
}

bool cache_file_header_verify(struct cache_file_header *header, [[maybe_unused]] const char *name, bool fatal) {
    if(
        header->header_signature != CACHE_FILE_HEADER_SIGNATURE ||
        header->footer_signature != CACHE_FILE_FOOTER_SIGNATURE ||
        header->size < 0 ||
        header->size > CACHE_FILE_MAXIMUM_SIZE ||
        strlen(header->name) > TAG_STRING_LENGTH
    ) {
        if(fatal) {
            vhalt(csprintf(temporary, "'%s' does not appear to be a cache file", name));
        };
    }
    // TODO: This hack will allow both retail and custom edition map types to run, but the game will create a zero byte loc.map if one does not exist
    else if(!(header->version == CACHE_FILE_VERSION_RETAIL || header->version == CACHE_FILE_VERSION_CUSTOM_EDITION)) {
        if(fatal) {
            vhalt(csprintf(temporary, "the cache file '%s' is an unsupported version (%d)", name, header->version));
        };
    }
    else {
        return true;
    }

    return false;
}

//#ifdef DEBUG_BUILD // FIXME: the game uses the tag instances pointer directly in release builds
void *tag_get(tag expected_group_tag, int32_t tag_index) {
    struct cache_file_tag_instance *tag_instance = cache_file_tag_instance_get(tag_index);
#ifdef DEBUG_BUILD
    if(
        tag_instance->group_tag != expected_group_tag &&
        tag_instance->parent_group_tags[0] != expected_group_tag &&
        tag_instance->parent_group_tags[1] != expected_group_tag
    ) {
        char group1[16];
        char group2[16];
        vhalt(csprintf(temporary, "expected tag group '%s' but got '%s' for %08x",
            tag_to_string(expected_group_tag, group1), tag_to_string(tag_instance->group_tag, group2), tag_index));
    }
#endif
    vassert(tag_instance->base_address, csprintf(temporary, "can't get() a tag with a base address!"));

    return tag_instance->base_address;
}
//#endif

char *tag_get_name(int32_t tag_index) {
    struct cache_file_tag_instance *tag_instance = cache_file_tag_instance_get(tag_index);

    return tag_instance->name;
}

tag tag_get_group_tag(int32_t tag_index) {
    struct cache_file_tag_instance *tag_instance = cache_file_tag_instance_get(tag_index);

    return tag_instance->group_tag;
}

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

void tag_iterator_new(struct tag_iterator *iterator, tag key_group_tag) {
    iterator->iterator.absolute_index = 0;
    iterator->key_group_tag = key_group_tag;
}

int32_t tag_iterator_next(struct tag_iterator *iterator) {
    while(iterator->iterator.absolute_index < cache_file_globals.tags_header->tag_count) {
        struct cache_file_tag_instance *instance = &global_tag_instances[iterator->iterator.absolute_index++];
        if(instance) {
            if(
                iterator->key_group_tag == TAG_NONE ||
                iterator->key_group_tag == instance->group_tag ||
                iterator->key_group_tag == instance->parent_group_tags[0] ||
                iterator->key_group_tag == instance->parent_group_tags[1]
            ) {
                return instance->tag_index;
            }
        }
    }

    return NONE;
}

static struct cache_file_tag_instance *cache_file_tag_instance_get(int32_t tag_index) {
    assert(cache_file_globals.tags_loaded);
    assert(global_tag_instances);
    int16_t absolute_index = DATUM_INDEX_TO_ABSOLUTE_INDEX(tag_index);
    vassert(absolute_index >= 0 && absolute_index < cache_file_globals.tags_header->tag_count,
        csprintf(temporary, "i don't think %08x is a tag index", tag_index));

    struct cache_file_tag_instance * tag_instance = &global_tag_instances[absolute_index];
    vassert(!DATUM_INDEX_TO_IDENTIFIER(tag_index) || tag_instance->tag_index == tag_index,
        csprintf(temporary, "i don't think %08x is a tag index", tag_index));

    return tag_instance;
}

#endif
