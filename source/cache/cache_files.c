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

#include "../bitmaps/bitmap_group.h"
#include "../scenario/scenario_definitions.h"
#include "../sound/sound_definitions.h"
#include "../interface/hud_messaging_definitions.h"
#include "../text/font_group.h"
#include "../text/text_group.h"

#include "cache_files.h"
#include "data_file.h"
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
    uint8_t *tag_data_cursor = (uint8_t *)tag_cache_base_address + cache_file_globals.header.tags_size;
    while((tag_index = tag_iterator_next(&iterator)) != NONE) {
        struct cache_file_tag_instance *tag_instance = cache_file_tag_instance_get(tag_index);

        // this does not need to be in the loop, gearbox
        if(tag_instance->group_tag == SCENARIO_GROUP_TAG) {
            struct scenario *scenario = scenario_get(tag_index);
            struct data_array *scenario_hs_syntax_data = scenario->hs_syntax_data.address;
            scenario_hs_syntax_data->data = scenario_hs_syntax_data + 1;
        }

        if(!TEST_FLAG(tag_instance->flags, _cache_file_tag_instance_flags_tag_in_data_file_bit)) {
            continue;
        }

        uint32_t size = 0;
        switch(tag_instance->group_tag) {
            case BITMAP_GROUP_TAG:
                size = data_file_load_tag(_data_file_type_bitmap, (uint32_t)tag_instance->base_address, tag_data_cursor);
                assert(size);

                tag_instance->base_address = tag_data_cursor;
                struct bitmap_group *bitmap_group = bitmap_group_get(tag_index);
                if(bitmap_group->bitmaps.address) {
                    bitmap_group->bitmaps.address = tag_data_cursor + (uint32_t)bitmap_group->bitmaps.address;
                }

                if(bitmap_group->sequences.address) {
                    bitmap_group->sequences.address = tag_data_cursor + (uint32_t)bitmap_group->sequences.address;
                }

                for(int sequence_index = 0; sequence_index < bitmap_group->sequences.count; sequence_index++) {
                    struct bitmap_group_sequence *sequence = bitmap_group_get_sequence(bitmap_group, sequence_index);
                    if(sequence->sprites.address) {
                        sequence->sprites.address = tag_data_cursor + (uint32_t)sequence->sprites.address;
                    }
                }

                break;
            case SOUND_DEFINITION_TAG:
                uint32_t sound_index = data_file_find_item(_data_file_type_sound, tag_get_name(tag_index));
                size = data_file_load_tag(_data_file_type_sound, sound_index, tag_data_cursor);
                assert(size);

                struct sound_definition *sound = sound_definition_get(tag_index);
                struct sound_definition *loaded_sound = (struct sound_definition *)tag_data_cursor;

                sound->sample_rate = loaded_sound->sample_rate;
                sound->encoding = loaded_sound->encoding;
                sound->compression = loaded_sound->compression;
                sound->runtime_maximum_play_time = loaded_sound->runtime_maximum_play_time;

                uint8_t *sound_data = tag_data_cursor + sizeof(struct sound_definition);
                sound->pitch_ranges.address = sound_data;

                for(int pitch_range_index = 0; pitch_range_index < sound->pitch_ranges.count; pitch_range_index++) {
                    struct sound_pitch_range *range = sound_definition_get_pitch_range(sound, pitch_range_index);
                    if(range->permutations.address) {
                        range->permutations.address = sound_data + (uint32_t)range->permutations.address;
                    }

                    for(int permutation_index = 0; permutation_index < range->permutations.count; permutation_index++) {
                        struct sound_permutation *permutation = sound_pitch_range_get_permutation(range, permutation_index);
                        permutation->runtime_tag_index = tag_index;
                        if(permutation->mouth_data.address) {
                            permutation->mouth_data.address = sound_data + (uint32_t)permutation->mouth_data.address;
                        }

                        if(permutation->subtitle_data.address) {
                            permutation->subtitle_data.address = sound_data + (uint32_t)permutation->subtitle_data.address;
                        }
                    }
                }

                break;
            case FONT_GROUP_TAG:
                size = data_file_load_tag(_data_file_type_loc, (uint32_t)tag_instance->base_address, tag_data_cursor);
                assert(size);

                tag_instance->base_address = tag_data_cursor;
                struct font_header *font = font_get_header(tag_index);
                if(font->character_tables.address) {
                    font->character_tables.address = tag_data_cursor + (uint32_t)font->character_tables.address;
                }

                for(int character_table_index = 0; character_table_index < font->character_tables.count; character_table_index++) {
                    struct font_character_tables_entry *character_table = font_get_character_tables_entry(font, character_table_index);
                    if(character_table->table.address) {
                        character_table->table.address = tag_data_cursor + (uint32_t)character_table->table.address;
                    }
                }

                if(font->characters.address) {
                    font->characters.address = tag_data_cursor + (uint32_t)font->characters.address;
                }

                if(font->pixels.address) {
                    font->pixels.address = tag_data_cursor + (uint32_t)font->pixels.address;
                }

                break;
            case HUD_MESSAGE_TEXT_DEFINITION_TAG:
                size = data_file_load_tag(_data_file_type_loc, (uint32_t)tag_instance->base_address, tag_data_cursor);
                assert(size);

                tag_instance->base_address = tag_data_cursor;
                struct hud_state_messages *hud_messages = hud_state_messages_get(tag_index);
                if(hud_messages->text_data.address) {
                    hud_messages->text_data.address = tag_data_cursor + (uint32_t)hud_messages->text_data.address;
                }

                if(hud_messages->elements.address) {
                    hud_messages->elements.address = tag_data_cursor + (uint32_t)hud_messages->elements.address;
                }

                if(hud_messages->messages.address) {
                    hud_messages->messages.address = tag_data_cursor + (uint32_t)hud_messages->messages.address;
                }

                break;
            case UNICODE_STRING_LISTS_GROUP_TAG:
                size = data_file_load_tag(_data_file_type_loc, (uint32_t)tag_instance->base_address, tag_data_cursor);
                assert(size);

                tag_instance->base_address = tag_data_cursor;
                struct unicode_string_list_group_header *unicode_strings = unicode_string_list_get_header(tag_index);
                if(unicode_strings->string_references.address) {
                    unicode_strings->string_references.address = tag_data_cursor + (uint32_t)unicode_strings->string_references.address;
                }

                for(int string_index = 0; string_index < unicode_strings->string_references.count; string_index++) {
                    struct unicode_string_list_string_reference *string_reference = unicode_string_list_get_string_reference(unicode_strings, string_index);
                    if(string_reference->string.address) {
                        string_reference->string.address = tag_data_cursor + (uint32_t)string_reference->string.address;
                    }
                }

                break;
            default:
                [[maybe_unused]] char group[16];
                vhalt(csprintf(temporary, "external data is not supported for tag group '%s' (tag instance %d)",
                    tag_to_string(tag_instance->group_tag, group), DATUM_INDEX_TO_ABSOLUTE_INDEX(tag_index)));
        }

        tag_data_cursor += size;
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

void *tag_get([[maybe_unused]] tag expected_group_tag, int32_t tag_index) {
#ifdef DEBUG_BUILD
    struct cache_file_tag_instance *tag_instance = cache_file_tag_instance_get(tag_index);
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

    vassert(tag_instance->base_address, csprintf(temporary, "can't get() a tag with a base address!"));

    return tag_instance->base_address;
#else
    return global_tag_instances[DATUM_INDEX_TO_ABSOLUTE_INDEX(tag_index)].base_address;
#endif
}

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
