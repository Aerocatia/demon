#ifndef DEMON_PHYSICAL_MEMORY_MAP_H

#include <stdint.h>

#include "../cseries/cseries.h"

#define TAG_CACHE_SIZE (64 * MEG) // was (23 * MEG)

uint32_t tag_cache_size(void);

#endif
