#include <stdint.h>
#include <stddef.h>

void *debug_malloc(size_t size, bool clear, const char *source_file, int32_t source_line) {
    typeof(debug_malloc) *FIXME_EXE_FUNCTION_POINTER = (void *)0x005509F0;
    return FIXME_EXE_FUNCTION_POINTER(size, clear, source_file, source_line);
}

void debug_free(void *pointer, const char *source_file, int32_t source_line) {
    typeof(debug_free) *FIXME_EXE_FUNCTION_POINTER = (void *)0x00550860;
    FIXME_EXE_FUNCTION_POINTER(pointer, source_file, source_line);
}
