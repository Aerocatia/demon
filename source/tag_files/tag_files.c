#include <string.h>

#include "../cseries/cseries.h"

const char *tag_name_strip_path(const char *name) {
    assert(name);

    const char *name_without_path = strrchr(name, '\\');
    if(name_without_path) {
        return name_without_path + 1;
    }

    return name;
}
