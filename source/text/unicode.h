#ifndef DEMON_UNICODE_H
#define DEMON_UNICODE_H

#include <uchar.h>

char *wide_to_ascii(char16_t *unicode, char *ascii, size_t ascii_length_bytes);
char16_t *ascii_to_wide(char *ascii, char16_t *unicode, size_t unicode_length_bytes);

#endif
