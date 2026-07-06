#ifndef DEMON_PHYSICAL_MEMORY_MAP_H

#include <stdint.h>

#include "../cseries/cseries.h"

#define TAG_CACHE_SIZE (64 * MEG) // was (23 * MEG)

void *physical_memory_get_game_state_base_address(void);

void *physical_memory_get_tag_cache_base_address(void);
uint32_t physical_memory_get_tag_cache_size(void);

void *physical_memory_get_texture_cache_base_address(void);
void *physical_memory_get_sound_cache_base_address(void);

#endif
