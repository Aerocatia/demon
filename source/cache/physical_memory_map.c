#include "physical_memory_map.h"

/* globals */

asm(".set _physical_memory_map_globals, 0x00AFF024");
extern struct {
    void *game_state_base_address;
    void *tag_cache_base_address;
    void *texture_cache_base_address;
    void *sound_cache_base_address;
} physical_memory_map_globals;

/* public functions */

void *physical_memory_get_game_state_base_address(void) {
    return physical_memory_map_globals.game_state_base_address;
}

void *physical_memory_get_tag_cache_base_address(void) {
#ifdef REQUIRE_CACHE_FILE
    return physical_memory_map_globals.tag_cache_base_address;
#else
    return nullptr;
#endif
}

uint32_t physical_memory_get_tag_cache_size(void) {
#ifdef REQUIRE_CACHE_FILE
    return TAG_CACHE_SIZE;
#else
    return 0;
#endif
}

void *physical_memory_get_texture_cache_base_address(void) {
    return physical_memory_map_globals.texture_cache_base_address;
}

void *physical_memory_get_sound_cache_base_address(void) {
    return physical_memory_map_globals.sound_cache_base_address;
}
