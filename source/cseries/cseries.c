#include <stdint.h>
#include <stdio.h>
#include <stdarg.h>
#include <string.h>

#include "../memory/byte_swapping.h"

#include "cseries.h"
#include "errors.h"

/* ---------- globals */

#if DEBUG
char temporary[256];
#endif

/* ---------- code */

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

//#ifdef DEBUG
char *csprintf(char *buffer, char *format, ...) {
    va_list arglist;
    va_start(arglist, format);
    vsprintf(buffer, format, arglist);
    va_end(arglist);

    return buffer;
}

enum {
    MAXIMUM_MEMCMP_SIZE = 512 * MEG,
    MAXIMUM_MEMCPY_MEMMOVE_SIZE = 512 * MEG,
    MAXIMUM_MEMSET_SIZE = 512 * MEG,
    MAXIMUM_STRING_SIZE = 256 * KILO
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
//#endif

//#ifdef DEBUG
void display_assert(char *information, char *file, int32_t line, bool fatal) {
    error(_error_silent, "EXCEPTION %s in %s,#%d: %s", fatal ? "halt" : "warn", file, line,
        information ? information : "<no reason given>");
}
//#endif

void system_exit(int code) {
    exit(code);
}
