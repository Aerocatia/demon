#include "cseries_windows.h"

#include "../demon/exe_functions.h"

#ifdef DEBUG_BUILD
void display_debug_string(char *string) {
    OutputDebugString(string);
}
#endif

uint32_t system_milliseconds() {
    return RUN_EXE_FUNCTION(system_milliseconds);
}

uint32_t system_seconds() {
    return RUN_EXE_FUNCTION(system_seconds);
}

void system_sleep(uint32_t milliseconds) {
    Sleep(milliseconds);
}
