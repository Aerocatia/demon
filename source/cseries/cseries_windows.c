#include <stdint.h>

#include "cseries_windows.h"
#include "cseries.h"

#ifdef DEBUG_BUILD
void display_debug_string(char *string) {
    OutputDebugString(string);
}
#endif

// halo_cache_symbols.exe
uint32_t (*exe_system_milliseconds)(void) = (void *)0x0054F1E0;

uint32_t system_milliseconds(void) {
    return exe_system_milliseconds();
}
