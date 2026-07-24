#include "../cseries/cseries_windows.h"
#include "../main/exe_functions.h"

void *game_state_allocate_buffer(uint32_t address, uint32_t cpu_size, uint32_t gpu_size) {
    return RUN_EXE_FUNCTION(game_state_allocate_buffer, address, cpu_size, gpu_size);
}

void game_state_create_or_open_file() {
    RUN_EXE_FUNCTION(game_state_create_or_open_file);
}

bool game_state_write_to_file() {
    abort();
}
