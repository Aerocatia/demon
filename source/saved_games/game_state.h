#ifndef DEMON_GAME_STATE_H
#define DEMON_GAME_STATE_H

#include "../cseries/cseries.h"

#include "../memory/data.h"
#include "../memory/lruv_cache.h"

// the exact value of MAXIMUM_GAME_STATE_SIZE got optimized out, but it is at least higher than Xbox
#define MAXIMUM_GAME_STATE_SIZE (4 * MIB + 256 * KIB)

void *game_state_malloc(const char *name, const char *type, int32_t size);
void *game_state_gpu_malloc(const char *name, const char *type, int32_t size);
struct data_array *game_state_data_new(const char *name, int16_t maximum_count, int16_t size);
struct lruv_cache *game_state_lruv_cache_new(const char *name, int32_t page_count, int32_t page_size_bits, int32_t maximum_block_count, lruv_delete_block_proc delete_block_proc, lruv_locked_block_proc locked_block_proc);

void *game_state_allocate_buffer(uint32_t address, uint32_t cpu_size, uint32_t gpu_size);
void game_state_create_or_open_file();

#endif
