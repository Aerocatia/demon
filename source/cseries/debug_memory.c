#include <stdint.h>
#include <stddef.h>

void *(*debug_malloc)(size_t size, bool clear, const char *source_file, int32_t source_line) = (void *)0x005509F0;
void *(*debug_free)(void *pointer, const char *source_file, int32_t source_line) = (void *)0x00550860;
