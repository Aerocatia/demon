#include <stdint.h>

#include "cseries_windows.h"
#include "cseries.h"

#ifdef DEBUG
void display_debug_string(char *string) {
    OutputDebugString(string);
}
#endif

uint32_t system_milliseconds(void) {
    uint32_t (*exe_system_milliseconds)(void) = (void *)0x0054F1E0; //halo_cache_symbols.exe
    return exe_system_milliseconds();
}
