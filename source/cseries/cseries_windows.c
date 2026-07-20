#include <stdint.h>

#include "cseries_windows.h"
#include "cseries.h"

#include "../main/exe_functions.h"

#ifdef DEBUG_BUILD
void display_debug_string(char *string) {
    OutputDebugString(string);
}
#endif

uint32_t system_milliseconds() {
    return RUN_EXE_FUNCTION(system_milliseconds);
}

void system_sleep(uint32_t milliseconds) {
    Sleep(milliseconds);
}
