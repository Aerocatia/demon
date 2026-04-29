#include <string.h>

#include "../cseries/cseries.h"

#include "data.h"
#include "lruv_cache.h"

#define LRUV_CACHE_SIGNATURE 0x77656565 // 'weee'
#define OLD_BLOCK_FRAME_COUNT 30

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

struct lruv_cache_hole {
    int32_t previous_block_index;
    uint32_t last_used_frame_index;
    int32_t first_page_index;
    int32_t page_count;
};

static int32_t lruv_cache_bytes_to_pages(struct lruv_cache *cache, int32_t size_in_bytes);

#define lruv_cache_block_get(cache, index) DATUM_GET((cache)->blocks, index, struct lruv_cache_block)

#ifdef DEBUG
    static void lruv_cache_verify(struct lruv_cache *cache, bool verify_blocks);
#else
    #define lruv_cache_verify(cache, verify_blocks) ((void)0)
#endif

struct lruv_cache *lruv_new(const char *name, int32_t page_count, int32_t page_size_bits, int32_t maximum_block_count, lruv_delete_block_proc delete_block_proc, lruv_locked_block_proc locked_block_proc) {
    struct lruv_cache *cache = malloc(lruv_allocation_size(maximum_block_count));
    if(cache) {
        lruv_initialize(cache, name, page_count, page_size_bits, maximum_block_count, delete_block_proc, locked_block_proc);
    }

    return cache;
}

int32_t lruv_allocation_size(int32_t maximum_block_count) {
    return sizeof(struct lruv_cache) + data_allocation_size(maximum_block_count, sizeof(struct lruv_cache_block));
}

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

void lruv_update_function_pointers(struct lruv_cache *cache, lruv_delete_block_proc delete_block_proc, lruv_locked_block_proc locked_block_proc) {
    assert(cache);

    cache->delete_block_proc = delete_block_proc;
    cache->locked_block_proc = locked_block_proc;
}

void lruv_delete(struct lruv_cache *cache) {
    lruv_cache_verify(cache, true);

    // The Xbox NTSC and PAL versions call data_dispose() here resulting in an invalid call to free()
    // Xbox demo and Halo PC removed that call. We can use data_destroy() here instead.
    // Note this will be optimized out unless using the debug_memory wrapper.
    data_destroy(cache->blocks);
    memset(cache, 0, sizeof(struct lruv_cache));
    free(cache);
}

void lruv_idle(struct lruv_cache *cache) {
    lruv_cache_verify(cache, false);

    cache->frame_index += 1;
}

void lruv_flush(struct lruv_cache *cache) {
    lruv_cache_verify(cache, true);

    struct data_iterator iterator;
    struct lruv_cache_block *block;

    data_iterator_new(&iterator, cache->blocks);
    while((block = data_iterator_next(&iterator))) {
        lruv_block_delete(cache, iterator.index);
    }
}

#define MAXIMUM_CACHE_HOLES 256
#define INCREMENT_HOLE_INDEX(index) ((index) == MAXIMUM_CACHE_HOLES -1 ? 0 : ((index) +1))

int32_t lruv_block_new(struct lruv_cache *cache, int32_t size) {
    int32_t desired_page_count = lruv_cache_bytes_to_pages(cache, size);
    assert(desired_page_count > 0);

    struct lruv_cache_hole best_hole;
    bool best_hole_valid = false;

    int32_t oldest_unlocked_block_index = NONE;
    uint32_t oldest_block_last_used_frame_index;

    struct lruv_cache_hole holes[MAXIMUM_CACHE_HOLES];
    int16_t hole_read_index = 0, hole_write_index = 0;
    int32_t block_index = cache->first_block_index;
    int32_t previous_block_index = NONE;
    int32_t page_index = 0;
    while(page_index < cache->page_count) {
        uint32_t last_used_frame_index;
        int32_t page_count;
        bool locked;
        if(INCREMENT_HOLE_INDEX(hole_write_index) != hole_read_index) {
            struct lruv_cache_hole *hole = &holes[hole_write_index];
            hole->previous_block_index = previous_block_index;
            hole->first_page_index = page_index;
            hole->last_used_frame_index = 0;
            hole->page_count = 0;
            hole_write_index = INCREMENT_HOLE_INDEX(hole_write_index);
        }

        if(block_index == NONE) {
            last_used_frame_index = 0;
            page_count = cache->page_count-page_index;
            locked = false;
            page_index = cache->page_count;
        }
        else {
            struct lruv_cache_block *block = lruv_cache_block_get(cache, block_index);
            if(page_index == block->first_page_index) {
                last_used_frame_index = block->last_used_frame_index;
                page_count = block->page_count;
                locked = cache->locked_block_proc && cache->locked_block_proc(block_index);
                if(block->last_used_frame_index == cache->frame_index) {
                    locked = true;
                }

                if(!locked && (oldest_unlocked_block_index == NONE || block->last_used_frame_index < oldest_block_last_used_frame_index)) {
                    oldest_unlocked_block_index = block_index;
                    oldest_block_last_used_frame_index = block->last_used_frame_index;
                }

                page_index = block->first_page_index + block->page_count;
                previous_block_index = block_index;
                block_index = block->next_block_index;
            }
            else {
                last_used_frame_index = 0;
                page_count = block->first_page_index-page_index;
                locked = false;
                assert(page_count > 0);
                page_index = block->first_page_index;
            }
        }

        if(locked) {
            hole_read_index = hole_write_index;
            continue;
        }

        for(int16_t hole_index = hole_read_index; hole_index != hole_write_index; hole_index = INCREMENT_HOLE_INDEX(hole_index)) {
            struct lruv_cache_hole *hole = &holes[hole_index];
            if(last_used_frame_index > hole->last_used_frame_index) {
                hole->last_used_frame_index = last_used_frame_index;
            }

            hole->page_count += page_count;
            if(hole->page_count >= desired_page_count) {
                if(!best_hole_valid || hole->last_used_frame_index < best_hole.last_used_frame_index || (hole->last_used_frame_index == best_hole.last_used_frame_index && hole->page_count < best_hole.page_count)) {
                    best_hole = *hole;
                    best_hole_valid = true;
                }

                assert(hole_read_index == hole_index);
                hole_read_index = INCREMENT_HOLE_INDEX(hole_read_index);
            }
        }
    }

    if(!best_hole_valid) {
        return NONE;
    }

    struct data_iterator iterator;
    struct lruv_cache_block *block;
    data_iterator_new(&iterator, cache->blocks);
    while((block = data_iterator_next(&iterator))) {
        if(block->first_page_index < best_hole.first_page_index+desired_page_count && block->first_page_index + block->page_count > best_hole.first_page_index) {
            assert(!cache->locked_block_proc || !cache->locked_block_proc(iterator.index));
            lruv_block_delete(cache, iterator.index);
        }
    }

    if(cache->blocks->actual_count == cache->blocks->maximum_count && oldest_unlocked_block_index != NONE) {
        if(best_hole.previous_block_index == oldest_unlocked_block_index) {
            best_hole.previous_block_index = lruv_cache_block_get(cache, oldest_unlocked_block_index)->previous_block_index;
        }

        assert(lruv_cache_block_get(cache, oldest_unlocked_block_index));
        assert(!cache->locked_block_proc || !cache->locked_block_proc(oldest_unlocked_block_index));
        lruv_block_delete(cache, oldest_unlocked_block_index);
    }

    int32_t new_block_index = datum_new(cache->blocks);
    if(new_block_index == NONE) {
        return new_block_index;
    }

    struct lruv_cache_block *new_block = lruv_cache_block_get(cache, new_block_index);
    if(best_hole.previous_block_index == NONE) {
        if (cache->first_block_index == NONE) {
            assert(cache->last_block_index == NONE);
            new_block->previous_block_index = NONE;
            cache->last_block_index = new_block_index;
        }
        else {
            struct lruv_cache_block *next_block = lruv_cache_block_get(cache, cache->first_block_index);
            assert(next_block->previous_block_index == NONE);
            new_block->previous_block_index = NONE;
            next_block->previous_block_index = new_block_index;
        }
    }
    else {
        struct lruv_cache_block *previous_block = lruv_cache_block_get(cache, best_hole.previous_block_index);
        if(previous_block->next_block_index == NONE) {
            new_block->previous_block_index = cache->last_block_index;
            cache->last_block_index = new_block_index;
        }
        else {
            struct lruv_cache_block *next_block = lruv_cache_block_get(cache, previous_block->next_block_index);
            new_block->previous_block_index = next_block->previous_block_index;
            next_block->previous_block_index = new_block_index;
        }
    }

    if(best_hole.previous_block_index == NONE) {
        new_block->next_block_index = cache->first_block_index;
        cache->first_block_index = new_block_index;
    }
    else {
        struct lruv_cache_block *previous_block = lruv_cache_block_get(cache, best_hole.previous_block_index);
        new_block->next_block_index = previous_block->next_block_index;
        previous_block->next_block_index = new_block_index;
    }

    new_block->first_page_index = best_hole.first_page_index;
    new_block->page_count = desired_page_count;
    new_block->last_used_frame_index = cache->frame_index;

    lruv_cache_verify(cache, true);

    return new_block_index;
}

void lruv_block_delete(struct lruv_cache *cache, int32_t block_index) {
    lruv_cache_verify(cache, true);

    if(cache->delete_block_proc) {
        cache->delete_block_proc(block_index);
    }

    struct lruv_cache_block *block = lruv_cache_block_get(cache, block_index);
    if(block->previous_block_index != NONE) {
        struct lruv_cache_block *previous_block = lruv_cache_block_get(cache, block->previous_block_index);
        previous_block->next_block_index = block->next_block_index;
    }
    else {
        assert(cache->first_block_index == block_index);
        cache->first_block_index = block->next_block_index;
    }

    if(block->next_block_index != NONE) {
        struct lruv_cache_block *next_block = lruv_cache_block_get(cache, block->next_block_index);
        next_block->previous_block_index = block->previous_block_index;
    }
    else {
        assert(cache->last_block_index == block_index);
        cache->last_block_index = block->previous_block_index;
    }

    datum_delete(cache->blocks, block_index);
    lruv_cache_verify(cache, true);
}

void lruv_block_touch(struct lruv_cache *cache, int32_t block_index) {
    lruv_cache_verify(cache, false);

    struct lruv_cache_block *block = lruv_cache_block_get(cache, block_index);
    block->last_used_frame_index = cache->frame_index;
}

uint32_t lruv_block_get_address(struct lruv_cache *cache, int32_t block_index) {
    lruv_cache_verify(cache, false);

    struct lruv_cache_block *block = lruv_cache_block_get(cache, block_index);
    return block->first_page_index << cache->page_size_bits;
}

bool lruv_block_touched(struct lruv_cache *cache, int32_t block_index) {
    lruv_cache_verify(cache, false);

    struct lruv_cache_block *block = lruv_cache_block_get(cache, block_index);
    return block->last_used_frame_index == cache->frame_index;
}

void lruv_cache_get_page_usage(struct lruv_cache *cache, uint8_t *page_usage) {
    lruv_cache_verify(cache, true);

    memset(page_usage, 0, cache->page_count);

    struct data_iterator iterator;
    struct lruv_cache_block *block;
    data_iterator_new(&iterator, cache->blocks);
    while((block = data_iterator_next(&iterator))) {
        uint8_t block_usage = FLAG(_lruv_cache_page_usage_allocated_bit);
        if(cache->locked_block_proc && cache->locked_block_proc(iterator.index)) {
            SET_FLAG(block_usage, _lruv_cache_page_usage_locked_bit, true);
        }

        if(block->last_used_frame_index == cache->frame_index) {
            SET_FLAG(block_usage, _lruv_cache_page_usage_used_this_frame_bit, true);
        }

        if(block->last_used_frame_index + OLD_BLOCK_FRAME_COUNT < cache->frame_index) {
            SET_FLAG(block_usage, _lruv_cache_page_usage_old_bit, true);
        }

        memset(page_usage + block->first_page_index, block_usage, block->page_count);
    }
}

void lruv_resize(struct lruv_cache *cache, int32_t new_page_count) {
    assert(new_page_count > 0);
    lruv_cache_verify(cache, true);

    struct data_iterator iterator;
    struct lruv_cache_block *block;
    data_iterator_new(&iterator, cache->blocks);
    while((block = data_iterator_next(&iterator))) {
        if(block->first_page_index + block->page_count > new_page_count) {
            lruv_block_delete(cache, iterator.index);
        }
    }

    cache->page_count = new_page_count;
}

bool lruv_has_locked_proc(const struct lruv_cache *cache) {
	assert(cache);

    return (cache->locked_block_proc != nullptr);
}

static int32_t lruv_cache_bytes_to_pages(struct lruv_cache *cache, int32_t size_in_bytes) {
    int32_t page_count = size_in_bytes >> cache->page_size_bits;
    if(size_in_bytes & ((1 << cache->page_size_bits) -1)) {
        page_count += 1;
    }

    return page_count;
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
