#include <stdint.h>
#include <stddef.h>

void *(*debug_malloc)(size_t size, bool clear, const char *source_file, int32_t source_line) = (void *)0x005509F0;
