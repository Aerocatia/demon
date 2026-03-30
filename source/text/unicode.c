#include <string.h>
#include <uchar.h>

#include "../cseries/cseries.h"

#include "unicode.h"

enum {
    MAXIMUM_STRING_SIZE = 32 * KILO
};

static size_t strlen16(const char16_t *str) {
    assert(str);
    const char16_t *end = str;
    while(*end != u'\0') {
        end++;
    }
    return end - str;
}

char *wide_to_ascii(char16_t *unicode, char *ascii, size_t ascii_length_bytes) {
    assert(unicode && ascii);
    size_t length = strlen16(unicode);
    assert(length < MAXIMUM_STRING_SIZE);
    if(ascii_length_bytes < (length + 1)) {
        return nullptr;
    }

    for(size_t i = 0; i < length; i++) {
        if(unicode[i] & 0xFF80) {
            ascii[i] = ' ';
        }
        else {
            ascii[i] = (char)unicode[i];
        }
    }
    ascii[length] = '\0';

    return ascii;
}

char16_t *ascii_to_wide(char *ascii, char16_t *unicode, size_t unicode_length_bytes) {
    assert(ascii && unicode);
    size_t length = strlen(ascii);
    assert(length < MAXIMUM_STRING_SIZE);
    if(unicode_length_bytes < (sizeof(char16_t) * (length + 1))) {
        return nullptr;
    }

    for(size_t i = 0; i < length; i++) {
        unicode[i] = ascii[i];
    }
    unicode[length] = u'\0';

    return unicode;
}
