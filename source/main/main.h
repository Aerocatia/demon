#ifndef DEMON_MAIN_H
#define DEMON_MAIN_H

#include "../cseries/cseries.h"

void main_skip(int16_t ticks);

void main_crash(const char *str);
void main_print_version();

void main_stop_time();
void main_start_time();

#endif
