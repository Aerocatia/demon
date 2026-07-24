#include "../cseries/cseries.h"

void dummy() {}

typedef void (*game_state_before_save_proc)();
static game_state_before_save_proc before_save_procs[] = {
    dummy
};

static inline void game_state_call_before_save_procs() {
    for(size_t i = 0; i < countof(before_save_procs); i++) {
        before_save_procs[i]();
    }
}
