#include <stdint.h>
#include "../cseries/build_number.h"

#ifdef DEMON_COUNT_THUNKS
bool demon_count_thunks = true;
#endif

#ifdef REQUIRE_CACHE_FILE
// Cache build (halo_cache_symbols.exe)
const char demon_replacements_json[] = {
    #embed "../../replacements.json"
    , 0
};

// blake3
const uint8_t demon_thunk_checksum[] = {
    0xE2, 0xD6, 0x45, 0x65, 0xA8, 0xAF, 0x40, 0x7E,
    0xF3, 0xAD, 0x12, 0xD6, 0x1E, 0xA0, 0x71, 0x80,
    0xE7, 0xD6, 0x02, 0xDB, 0x6B, 0xA8, 0x90, 0x38,
    0x67, 0x61, 0xA3, 0x73, 0x8F, 0x00, 0x17, 0x40
};

const void *demon_thunk_address = (void *)0x00401000;
const char demon_target_exe_name[] = BUILD_NUMBER " cache build";
#else
// Tag build (halo_tag_symbols.exe), maybe in the future
#error "The demon can only target the cache build at this time"
#endif
