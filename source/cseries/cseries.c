#include <stdint.h>
#include <stdio.h>
#include <stdarg.h>
#include <string.h>
#include <stdcountof.h>
#include <ctype.h>

#include "../memory/byte_swapping.h"

#include "cseries.h"
#include "errors.h"

#include "../main/exe_functions.h"

#if DEBUG_BUILD
char temporary[256];
#endif

#define NUMBER_OF_GLOBAL_COLORS 17

static const real_argb_color private_real_argb_colors[] = {
    {{1.0f, 1.0f, 1.0f, 1.0f}},
    {{1.0f, 0.5f, 0.5f, 0.5f}},
    {{1.0f, 0.0f, 0.0f, 0.0f}},
    {{1.0f, 1.0f, 0.0f, 0.0f}},
    {{1.0f, 0.0f, 1.0f, 0.0f}},
    {{1.0f, 0.0f, 0.0f, 1.0f}},
    {{1.0f, 0.0f, 1.0f, 1.0f}},
    {{1.0f, 1.0f, 1.0f, 0.0f}},
    {{1.0f, 1.0f, 0.0f, 1.0f}},
    {{1.0f, 1.0f, 0.41f, 0.7f}},
    {{1.0f, 0.39f, 0.58f, 0.93f}},
    {{1.0f, 1.0f, 0.5f, 0.0f}},
    {{1.0f, 0.44f, 0.05f, 0.43f}},
    {{1.0f, 0.5f, 1.0f, 0.83f}},
    {{1.0f, 0.0f, 0.39f, 0.0f}},
    {{1.0f, 1.0f, 0.63f, 0.48f}},
    {{1.0f, 0.81f, 0.13f, 0.56f}}
};
static_assert(countof(private_real_argb_colors) == NUMBER_OF_GLOBAL_COLORS);

const real_argb_color *global_real_argb_white     = &private_real_argb_colors[0];
const real_argb_color *global_real_argb_grey      = &private_real_argb_colors[1];
const real_argb_color *global_real_argb_black     = &private_real_argb_colors[2];
const real_argb_color *global_real_argb_red       = &private_real_argb_colors[3];
const real_argb_color *global_real_argb_green     = &private_real_argb_colors[4];
const real_argb_color *global_real_argb_blue      = &private_real_argb_colors[5];
const real_argb_color *global_real_argb_cyan      = &private_real_argb_colors[6];
const real_argb_color *global_real_argb_yellow    = &private_real_argb_colors[7];
const real_argb_color *global_real_argb_magenta   = &private_real_argb_colors[8];
const real_argb_color *global_real_argb_pink      = &private_real_argb_colors[9];
const real_argb_color *global_real_argb_lightblue = &private_real_argb_colors[10];
const real_argb_color *global_real_argb_orange    = &private_real_argb_colors[11];
const real_argb_color *global_real_argb_purple    = &private_real_argb_colors[12];
const real_argb_color *global_real_argb_aqua      = &private_real_argb_colors[13];
const real_argb_color *global_real_argb_darkgreen = &private_real_argb_colors[14];
const real_argb_color *global_real_argb_salmon    = &private_real_argb_colors[15];
const real_argb_color *global_real_argb_violet    = &private_real_argb_colors[16];

const real_rgb_color *global_real_rgb_white       = &private_real_argb_colors[0].rgb;
const real_rgb_color *global_real_rgb_grey        = &private_real_argb_colors[1].rgb;
const real_rgb_color *global_real_rgb_black       = &private_real_argb_colors[2].rgb;
const real_rgb_color *global_real_rgb_red         = &private_real_argb_colors[3].rgb;
const real_rgb_color *global_real_rgb_green       = &private_real_argb_colors[4].rgb;
const real_rgb_color *global_real_rgb_blue        = &private_real_argb_colors[5].rgb;
const real_rgb_color *global_real_rgb_cyan        = &private_real_argb_colors[6].rgb;
const real_rgb_color *global_real_rgb_yellow      = &private_real_argb_colors[7].rgb;
const real_rgb_color *global_real_rgb_magenta     = &private_real_argb_colors[8].rgb;
const real_rgb_color *global_real_rgb_pink        = &private_real_argb_colors[9].rgb;
const real_rgb_color *global_real_rgb_lightblue   = &private_real_argb_colors[10].rgb;
const real_rgb_color *global_real_rgb_orange      = &private_real_argb_colors[11].rgb;
const real_rgb_color *global_real_rgb_purple      = &private_real_argb_colors[12].rgb;
const real_rgb_color *global_real_rgb_aqua        = &private_real_argb_colors[13].rgb;
const real_rgb_color *global_real_rgb_darkgreen   = &private_real_argb_colors[14].rgb;
const real_rgb_color *global_real_rgb_salmon      = &private_real_argb_colors[15].rgb;
const real_rgb_color *global_real_rgb_violet      = &private_real_argb_colors[16].rgb;

tag string_to_tag(const char *s) {
    tag t = NONE;
    t = *((tag *)s);

    return SWAP4_BE(t);
}

char *tag_to_string(tag t, char *s) {
    t = SWAP4_BE(t);
    *((tag *)s) = t;
    s[sizeof(tag)] = '\0';

    return s;
}

int strncmp_case_insensitive(const char *s1, const char *s2, size_t count) {
    assert(s1 && s2);
    if(count == 0) {
        return 0;
    }

    do {
        int a = fast_ascii_tolower((unsigned char)*s1);
        int b = fast_ascii_tolower((unsigned char)*s2++);
        int c = a - b;
        if(c) {
            return c;
        }

        if(*s1++ == '\0') {
            break;
        }
    }
    while(--count != 0);

    return 0;
}

/* debug wrappers */

#ifdef DEBUG_BUILD
char *csprintf(char *buffer, char *format, ...) {
    va_list arglist;
    va_start(arglist, format);
    vsprintf(buffer, format, arglist);
    va_end(arglist);

    return buffer;
}

enum {
    MAXIMUM_MEMCMP_SIZE = 512 * MIB,
    MAXIMUM_MEMCPY_MEMMOVE_SIZE = 512 * MIB,
    MAXIMUM_MEMSET_SIZE = 512 * MIB,
    MAXIMUM_STRING_SIZE = 256 * KIB
};

int csmemcmp(const void *p1, const void *p2, size_t size) {
    assert(p1 && p2);
    assert(size <= MAXIMUM_MEMCMP_SIZE);

    return memcmp(p1, p2, size);
}

void *csmemmove(void *destination, const void *source, size_t size) {
    assert(destination && source);
    assert(size <= MAXIMUM_MEMCPY_MEMMOVE_SIZE);

    return memmove(destination, source, size);
}

void *csmemset(void *buffer, int c, size_t size){
    assert(buffer);
    assert(size <= MAXIMUM_MEMSET_SIZE);

    return memset(buffer, c, size);
}

char *csstrcat(char *s1, const char *s2) {
    assert(s1 && s2);

    return strcat(s1, s2);
}

int csstrcmp(const char *s1, const char *s2) {
    assert(s1 && s2);

    return strcmp(s1, s2);
}

char *csstrncat(char *s1, const char *s2, size_t size) {
    assert(s1 && s2);
    assert(size < MAXIMUM_STRING_SIZE);

    return strncat(s1, s2, size);
}

int csstrncmp(const char *s1, const char *s2, size_t size) {
    assert(s1 && s2);
    assert(size < MAXIMUM_STRING_SIZE);

    return strncmp(s1, s2, size);
}

char *csstrncpy(char *s1, const char *s2, size_t size) {
    assert(s1 && s2);
    assert(size < MAXIMUM_STRING_SIZE);

    return strncpy(s1, s2, size);
}

char *csstrtok(char *s1, const char *s2) {
    assert(s2);

    return strtok(s1, s2);
}

size_t csstrlen(const char *s1) {
    assert(s1);
    size_t size = strlen(s1);
    assert(size < MAXIMUM_STRING_SIZE);

    return size;
}

char *csstrcpy(char *destination, const char *source) {
    size_t source_size = strlen(source);

    assert(source_size < MAXIMUM_STRING_SIZE);
    assert(source + source_size < destination || destination + source_size < source);

    return strcpy(destination, source);
}

void *csmemcpy(void *destination, const void *source, size_t size) {
    assert(destination && source);
    assert(size < MAXIMUM_MEMCPY_MEMMOVE_SIZE);
    assert((uint8_t *)source + size <= (uint8_t *)destination || (uint8_t *)destination + size <= (uint8_t *)source);

    return memcpy(destination, source, size);
}
#endif

#ifdef DEBUG_BUILD
void display_assert(char *information, char *file, int32_t line, bool fatal) {
    error(_error_silent, "EXCEPTION %s in %s,#%d: %s", fatal ? "halt" : "warn", file, line,
        information ? information : "<no reason given>");
}
#endif

void keystone_dispose() {
    RUN_EXE_FUNCTION(keystone_dispose);
}

void system_exit(int code) {
    keystone_dispose();
    exit(code);
}
