#include <stdio.h>
#include <stdarg.h>

extern void demon_error_catcher(short priority, const char *message);

// Rust doesn't support C-style variadic arguments, so we're going to do everything here
void demon_error_trampoline(short priority, const char *message, ...) {
    char buffer[1024];
    va_list args;
    va_start(args, message);
    vsnprintf(buffer, sizeof(buffer), message, args);
    va_end(args);

    demon_error_catcher(priority, buffer);
}
