#include "../cseries/cseries.h"
#include "terminal.h"

// halo_cache_symbols.exe
void (*terminal_printf)(const real_argb_color *color, const char *format, ...) = (void *)0x00648650;
