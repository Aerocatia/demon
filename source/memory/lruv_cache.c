#include <string.h>

#include "../cseries/cseries.h"

#include "data.h"
#include "lruv_cache.h"

#define LRUV_CACHE_SIGNATURE 0x77656565 // 'weee'

struct lruv_cache {
    char name[32];
    lruv_delete_block_proc delete_block_proc;
    lruv_locked_block_proc locked_block_proc;
    int32_t page_count;
    int32_t page_size_bits;
    uint32_t frame_index;
    int32_t first_block_index;
    int32_t last_block_index;
    struct data_array *blocks;
    uint32_t signature;
};
static_assert(sizeof(struct lruv_cache) == 0x44);

struct lruv_cache_block {
    struct datum_header;
    bool unused_flags[2];
    int32_t page_count;
    int32_t first_page_index;
    int32_t next_block_index;
    int32_t previous_block_index;
    uint32_t last_used_frame_index;
    uint32_t unused;
};
static_assert(sizeof(struct lruv_cache_block) == 0x1C);

#define lruv_cache_block_get(cache, index) DATUM_GET((cache)->blocks, index, struct lruv_cache_block)

#ifdef DEBUG
    static void lruv_cache_verify(struct lruv_cache *cache, bool verify_blocks);
#else
    #define lruv_cache_verify(cache, verify_blocks) ((void)0)
#endif

void lruv_initialize(struct lruv_cache *cache, const char *name, int32_t page_count, int32_t page_size_bits, int32_t maximum_block_count, lruv_delete_block_proc delete_block_proc, lruv_locked_block_proc locked_block_proc) {
    assert(name);
    assert(page_count > 0);
    assert(page_size_bits > 0 && page_size_bits < SHORT_BITS);
    assert(maximum_block_count > 0);

    struct data_array *blocks = (struct data_array *)(cache + 1);
    data_initialize((struct data_array *)(cache + 1), name, maximum_block_count, sizeof(struct lruv_cache_block));
    data_make_valid(blocks);

    memset(cache, 0, sizeof(struct lruv_cache));
    strncpy(cache->name, name, sizeof(cache->name) - 1);

    cache->delete_block_proc = delete_block_proc;
    cache->locked_block_proc = locked_block_proc;
    cache->page_count = page_count;
    cache->page_size_bits = page_size_bits;
    cache->blocks = blocks;
    cache->signature = LRUV_CACHE_SIGNATURE;
    cache->first_block_index = NONE;
    cache->last_block_index = NONE;
    cache->frame_index = 1;

    lruv_cache_verify(cache, true);
}

#ifdef DEBUG
static void lruv_cache_verify(struct lruv_cache *cache, bool verify_blocks) {
    assert(cache);
    assert(cache->signature == LRUV_CACHE_SIGNATURE);
    data_verify(cache->blocks);

    if(!verify_blocks) {
        return;
    }

    int32_t block_index = cache->first_block_index;
    while(block_index != NONE) {
        struct lruv_cache_block *block = lruv_cache_block_get(cache, block_index);
        if(block->previous_block_index == NONE) {
            assert(cache->first_block_index == block_index);
        }
        else {
            struct lruv_cache_block *previous_block = lruv_cache_block_get(cache, block->previous_block_index);
            assert(previous_block->next_block_index == block_index);
            assert(previous_block->first_page_index < block->first_page_index);
            assert(previous_block->first_page_index + previous_block->page_count <= block->first_page_index);
        }

        if(block->next_block_index == NONE) {
            assert(cache->last_block_index == block_index);
        }
        else {
            struct lruv_cache_block *next_block = lruv_cache_block_get(cache, block->next_block_index);
            assert(next_block->previous_block_index == block_index);
            assert(next_block->first_page_index > block->first_page_index);
            assert(block->first_page_index + block->page_count <= next_block->first_page_index);
        }

        block_index = block->next_block_index;
    }
}
#endif
