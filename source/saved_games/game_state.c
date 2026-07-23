#include "../cseries/cseries.h"

#include "../cache/physical_memory_map.h"
#include "../memory/crc.h"
#include "../tag_files/tag_files.h"

#include "game_state.h"

#define GAMESTATE_FILENAME "gamestate.txt"

/* game state globals */

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

/* game state functions */

static void game_state_allocation_record(const char *name, const char *type, int32_t size, int32_t total, int32_t allocated, bool gpu);

void *game_state_malloc([[maybe_unused]] const char *name, [[maybe_unused]] const char *type, int32_t size) {
    assert(!(size&3));
    assert(!game_state_globals.locked);
    assert(game_state_globals.cpu_allocation_size + size <= GAME_STATE_CPU_SIZE);

#ifdef DEBUG_LOGGING
    game_state_allocation_record(name, type, size, GAME_STATE_CPU_SIZE, game_state_globals.cpu_allocation_size, false);
#endif

    void *result = (uint8_t *)game_state_globals.base_address + game_state_globals.cpu_allocation_size;
    game_state_globals.cpu_allocation_size += size;

    crc_checksum_buffer(&game_state_globals.allocation_size_checksum, &size, sizeof(int32_t));

    return result;
}

[[maybe_unused]] static void game_state_allocation_record(const char *name, const char *type, int32_t size, int32_t total, int32_t allocated, bool gpu) {
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
