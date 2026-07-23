#ifndef DEMON_PHYSICAL_MEMORY_MAP_H

#include "../cseries/cseries.h"

#define PHYSICAL_MEMORY_BASE_ADDRESS 0x40000000

#define GAME_STATE_BASE_ADDRESS PHYSICAL_MEMORY_BASE_ADDRESS
#define GAME_STATE_CPU_SIZE (4352 * KIB)
#define GAME_STATE_GPU_SIZE 0
#define GAME_STATE_SIZE (GAME_STATE_CPU_SIZE + GAME_STATE_GPU_SIZE)

#define TAG_CACHE_BASE_ADDRESS (GAME_STATE_BASE_ADDRESS + GAME_STATE_SIZE)
#define TAG_CACHE_SIZE (64 * MIB) // was (23 * MIB)

static_assert(TAG_CACHE_BASE_ADDRESS == 0x40440000);

void *physical_memory_get_game_state_base_address();
void *physical_memory_get_tag_cache_base_address();
uint32_t physical_memory_get_tag_cache_size();

void *physical_memory_get_texture_cache_base_address();
void *physical_memory_get_sound_cache_base_address();

#endif
