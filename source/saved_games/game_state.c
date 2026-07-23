#include "../cseries/cseries.h"

#include "../cache/physical_memory_map.h"
#include "../memory/crc.h"
#include "../memory/data.h"
#include "../memory/lruv_cache.h"
#include "../tag_files/tag_files.h"

#include "game_state.h"

#define GAMESTATE_FILENAME "gamestate.txt"

/* globals */

struct game_state_header {
    uint32_t allocation_size_checksum;
    char map_name[TAG_FILE_NAME_LENGTH + 1];
    char build_number[32];
    int16_t player_count;
    int16_t difficulty;
    uint32_t cache_file_checksum;
    int32_t unused[7];
    uint32_t checksum;
};
static_assert(sizeof(struct game_state_header) == 332);

struct game_state_global_data {
    void *base_address;
    int32_t cpu_allocation_size;
    int32_t gpu_allocation_size;
    uint32_t allocation_size_checksum;
    bool locked;
    bool saved_game_valid;
    int32_t revert_time;
    struct game_state_header *header;
};
static_assert(sizeof(struct game_state_global_data) == 28);

asm(".set _game_state_globals, 0x00F14600");
extern struct game_state_global_data game_state_globals;

/* forward declarations */

static void game_state_allocation_record(const char *name, const char *type, int32_t size, int32_t total, int32_t allocated, bool gpu);

/* public functions */

void game_state_initialize() {
    static_assert(GAME_STATE_SIZE <= MAXIMUM_GAME_STATE_SIZE);
    crc_new(&game_state_globals.allocation_size_checksum);

    game_state_globals.base_address = game_state_allocate_buffer(GAME_STATE_BASE_ADDRESS, GAME_STATE_CPU_SIZE, GAME_STATE_GPU_SIZE);
    game_state_create_or_open_file();
    game_state_globals.header = game_state_malloc("header", nullptr, sizeof(struct game_state_header));
}

void *game_state_malloc([[maybe_unused]] const char *name, [[maybe_unused]] const char *type, int32_t size) {
    assert(!(size & 3));
    assert(!game_state_globals.locked);
    assert(game_state_globals.cpu_allocation_size + size <= GAME_STATE_CPU_SIZE);

#ifdef DEBUG_LOGGING
    game_state_allocation_record(name, type, size, GAME_STATE_CPU_SIZE, game_state_globals.cpu_allocation_size, false);
#endif

    void *result = (uint8_t *)game_state_globals.base_address + game_state_globals.cpu_allocation_size;
    game_state_globals.cpu_allocation_size += size;

    crc_checksum_buffer(&game_state_globals.allocation_size_checksum, &size, sizeof(typeof(size)));

    return result;
}

void *game_state_gpu_malloc([[maybe_unused]] const char *name, [[maybe_unused]] const char *type, int32_t size) {
    assert(!(size & 3));
    assert(!game_state_globals.locked);
    assert(game_state_globals.gpu_allocation_size + size <= GAME_STATE_GPU_SIZE);

#ifdef DEBUG_LOGGING
    game_state_allocation_record(name, type, size, GAME_STATE_GPU_SIZE, game_state_globals.gpu_allocation_size, true);
#endif

    game_state_globals.gpu_allocation_size += size;
    void *result = (uint8_t *)game_state_globals.base_address + GAME_STATE_SIZE - game_state_globals.gpu_allocation_size;

    crc_checksum_buffer(&game_state_globals.allocation_size_checksum, &size, sizeof(typeof(size)));

    return result;
}

struct data_array *game_state_data_new(const char *name, int16_t maximum_count, int16_t size) {
    int32_t buffer_size = data_allocation_size(maximum_count, size);
    void *buffer = game_state_malloc(name, "data array", buffer_size);
    data_initialize(buffer, name, maximum_count, size);

    return buffer;
}

struct lruv_cache *game_state_lruv_cache_new(const char *name, int32_t page_count, int32_t page_size_bits, int32_t maximum_block_count, lruv_delete_block_proc delete_block_proc, lruv_locked_block_proc locked_block_proc) {
    int32_t buffer_size = lruv_allocation_size(maximum_block_count);
    void *buffer = game_state_malloc(name, "lruv cache", buffer_size);
    lruv_initialize(buffer, name, page_count, page_size_bits, maximum_block_count, delete_block_proc, locked_block_proc);

    return buffer;
}

/* private functions */

[[maybe_unused]] static void game_state_allocation_record(const char *name, const char *type, int32_t size, int32_t total, int32_t allocated, bool gpu) {
    assert(name);
    static FILE *file;
    if(!file) {
        file = fopen(GAMESTATE_FILENAME, "wt");
        if(!file) {
            return;
        }
    }

    auto used = allocated + size;
    auto remaining = total - used;
    fprintf(file, "%40s,%20s,% 10d,% 10d,% 10d%s\n", name, type ? type : "<unknown>", size, used, remaining, gpu ? "*" : "");
    fflush(file);
}
