#include "../cseries/cseries_windows.h"

void (*exe_texture_cache_open)() = (void *)0x0051FD20;

void texture_cache_open() {
    exe_texture_cache_open();
}
