#include <stdint.h>
#include <stddef.h>

#include "../main/exe_functions.h"

void *debug_malloc(size_t size, bool clear, const char *source_file, int32_t source_line) {
    return RUN_EXE_FUNCTION(debug_malloc, size, clear, source_file, source_line);
}

void debug_free(void *pointer, const char *source_file, int32_t source_line) {
    RUN_EXE_FUNCTION(debug_free, pointer, source_file, source_line);
}
