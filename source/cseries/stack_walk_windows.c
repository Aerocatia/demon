#include "cseries_windows.h"
#include "errors.h"

// halo_cache_symbols.exe
void (*exe_stack_walk_initialize)(void) = (void *)0x00559480;
void (*exe_stack_walk_dispose)(void) = (void *)0x005593F0;

void stack_walk_initialize(void) {
    exe_stack_walk_initialize();
}

void stack_walk_dispose(void) {
    exe_stack_walk_dispose();
}
