#include <stdint.h>
#include <stddef.h>

// halo_cache_symbols.exe
void *(*exe_debug_malloc)(size_t size, bool clear, const char *source_file, int32_t source_line) = (void *)0x005509F0;
void (*exe_debug_free)(void *pointer, const char *source_file, int32_t source_line) = (void *)0x00550860;

void *debug_malloc(size_t size, bool clear, const char *source_file, int32_t source_line) {
    return exe_debug_malloc(size, clear, source_file, source_line);
}

void debug_free(void *pointer, const char *source_file, int32_t source_line) {
    exe_debug_free(pointer, source_file, source_line);
}
