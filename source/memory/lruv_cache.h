#ifndef __LRUV_CACHE_H__
#define __LRUV_CACHE_H__

struct lruv_cache;

typedef void (*lruv_delete_block_proc)(int32_t block_index);
typedef bool (*lruv_locked_block_proc)(int32_t block_index);

void lruv_initialize(struct lruv_cache *cache, const char *name, int32_t page_count, int32_t page_size_bits, int32_t maximum_block_count, lruv_delete_block_proc delete_block_proc, lruv_locked_block_proc locked_block_proc);

#endif
