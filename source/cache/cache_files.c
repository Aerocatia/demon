#include <stdint.h>
#include <string.h>

#include "../cseries/build_number.h"
#include "../cseries/cseries.h"
#include "../tag_files/tag_groups.h"

#include "cache_files.h"

static bool cache_file_valid_version(int32_t version) {
    if(version == CACHE_FILE_VERSION_RETAIL || version == CACHE_FILE_VERSION_CUSTOM_EDITION) {
        return true;
    }

    return false;
}

bool cache_file_header_verify(struct cache_file_header *header, [[maybe_unused]] const char *name, bool fatal) {
    if(header->header_signature != CACHE_FILE_HEADER_SIGNATURE ||
        header->footer_signature != CACHE_FILE_FOOTER_SIGNATURE ||
        header->size < 0 ||
        header->size > CACHE_FILE_MAXIMUM_SIZE ||
        strlen(header->name) > TAG_STRING_LENGTH
    ) {
        if(fatal) {
            vhalt(csprintf(temporary, "'%s' does not appear to be a cache file", name));
        };
    }
    else if(!cache_file_valid_version(header->version)) {
        if(fatal) {
            vhalt(csprintf(temporary, "the cache file '%s' is an unsupported version (%d)", name, header->version));
        };
    }
    else {
        return true;
    }

    return false;
}
