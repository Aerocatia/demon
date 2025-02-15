#include <stdio.h>
#include <stdarg.h>

extern void demon_terminal_put(const void *priority, const char *message);

// Rust doesn't support C-style variadic arguments, so we're going to do everything here
void demon_terminal_printf_trampoline(const void *color, const char *message, ...) {
    char buffer[1024];
    va_list args;
    va_start(args, message);
    vsnprintf(buffer, sizeof(buffer), message, args);
    va_end(args);

    demon_terminal_put(color, buffer);
}
