#include <stdint.h>

#include "cseries_windows.h"
#include "cseries.h"

#ifdef DEBUG_BUILD
void display_debug_string(char *string) {
    OutputDebugString(string);
}
#endif

uint32_t system_milliseconds() {
    typeof(system_milliseconds) *FIXME_EXE_FUNCTION_POINTER = (void *)0x0054F1E0;
    return FIXME_EXE_FUNCTION_POINTER();
}

void system_sleep(uint32_t milliseconds) {
    Sleep(milliseconds);
}
