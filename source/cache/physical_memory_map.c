#include <stdint.h>

#include "../cseries/cseries_windows.h"
#include "physical_memory_map.h"

/* globals */

struct physical_memory_map_global_data {
    void *game_state_base_address;
    void *tag_cache_base_address;
    void *texture_cache_base_address;
    void *sound_cache_base_address;
};
static_assert(sizeof(struct physical_memory_map_global_data) == 16);

asm(".set _physical_memory_map_globals, 0x00AFF024");
extern struct physical_memory_map_global_data physical_memory_map_globals;

/* public functions */

void physical_memory_free() {
    if(physical_memory_map_globals.game_state_base_address != nullptr) {
        VirtualFree(physical_memory_map_globals.game_state_base_address, 0, MEM_RELEASE);
        physical_memory_map_globals.game_state_base_address = nullptr;
    }

#ifdef CACHE_FILE_BUILD
    if(physical_memory_map_globals.tag_cache_base_address != nullptr) {
        // halo_cache_symbols.exe frees this, but Halo PC changed it to be part of the game state allocation so this is invalid
        // VirtualFree(physical_memory_map_globals.tag_cache_base_address, 0, MEM_RELEASE);
        physical_memory_map_globals.tag_cache_base_address = nullptr;
    }
#endif

    if(physical_memory_map_globals.texture_cache_base_address != nullptr) {
        VirtualFree(physical_memory_map_globals.texture_cache_base_address, 0, MEM_RELEASE);
        physical_memory_map_globals.texture_cache_base_address = nullptr;
    }

    if(physical_memory_map_globals.sound_cache_base_address != nullptr) {
        VirtualFree(physical_memory_map_globals.sound_cache_base_address, 0, MEM_RELEASE);
        physical_memory_map_globals.sound_cache_base_address = nullptr;
    }
}

void *physical_memory_get_game_state_base_address() {
    return physical_memory_map_globals.game_state_base_address;
}

void *physical_memory_get_tag_cache_base_address() {
#ifdef CACHE_FILE_BUILD
    return physical_memory_map_globals.tag_cache_base_address;
#else
    return nullptr;
#endif
}

uint32_t physical_memory_get_tag_cache_size() {
#ifdef CACHE_FILE_BUILD
    return TAG_CACHE_SIZE;
#else
    return 0;
#endif
}

void *physical_memory_get_texture_cache_base_address() {
    return physical_memory_map_globals.texture_cache_base_address;
}

void *physical_memory_get_sound_cache_base_address() {
    return physical_memory_map_globals.sound_cache_base_address;
}
