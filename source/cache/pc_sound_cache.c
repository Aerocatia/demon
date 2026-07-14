#include "../cseries/cseries_windows.h"

void (*exe_sound_cache_open)() = (void *)0x0051DF90;

void sound_cache_open() {
    exe_sound_cache_open();
}
