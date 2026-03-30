#include <stdint.h>
#include <stdio.h>
#include <stdarg.h>

#include "cseries.h"
#include "errors.h"

#if DEBUG
char temporary[256];
#endif

#ifdef DEBUG
char *csprintf(char *buffer, char *format, ...) {
    va_list arglist;
    va_start(arglist, format);
    vsprintf(buffer, format, arglist);
    va_end(arglist);

    return buffer;
}

void display_assert(char *information, char *file, int32_t line, bool fatal) {
    error(_error_silent, "EXCEPTION %s in %s,#%d: %s", fatal ? "halt" : "warn", file, line,
        information ? information : "<no reason given>");
}
#endif

void system_exit(int code) {
    exit(code);
}
