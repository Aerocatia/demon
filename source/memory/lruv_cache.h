#ifndef __LRUV_CACHE_H__
#define __LRUV_CACHE_H__

struct lruv_cache;

typedef void (*lruv_delete_block_proc)(int32_t block_index);
typedef bool (*lruv_locked_block_proc)(int32_t block_index);

struct lruv_cache *lruv_new(const char *name, int32_t page_count, int32_t page_size_bits, int32_t maximum_block_count, lruv_delete_block_proc delete_block_proc, lruv_locked_block_proc locked_block_proc);
void lruv_delete(struct lruv_cache *cache);

void lruv_update_function_pointers(struct lruv_cache *cache, lruv_delete_block_proc delete_block_proc, lruv_locked_block_proc locked_block_proc);

int32_t lruv_allocation_size(int32_t maximum_block_count);
void lruv_initialize(struct lruv_cache *cache, const char *name, int32_t page_count, int32_t page_size_bits, int32_t maximum_block_count, lruv_delete_block_proc delete_block_proc, lruv_locked_block_proc locked_block_proc);

void lruv_idle(struct lruv_cache *cache);
void lruv_flush(struct lruv_cache *cache);

int32_t lruv_block_new(struct lruv_cache *cache, int32_t size);
void lruv_block_delete(struct lruv_cache *cache, int32_t block_index);

void lruv_block_touch(struct lruv_cache *cache, int32_t block_index);
bool lruv_block_touched(struct lruv_cache *cache, int32_t block_index);
uint32_t lruv_block_get_address(struct lruv_cache *cache, int32_t block_index);

bool lruv_has_locked_proc(const struct lruv_cache *cache);

#endif
