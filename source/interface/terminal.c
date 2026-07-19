#include "../cseries/cseries.h"
#include "terminal.h"

// FIXME_EXE_FUNCTION_POINTER
void (*terminal_printf)(const real_argb_color *color, const char *format, ...) = (void *)0x00648650;
