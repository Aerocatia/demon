#include <stdint.h>
#include <uchar.h>

#include "../cseries/cseries.h"

#include "text_group.h"

const char16_t *unicode_string_list_get_string(int32_t tag_index, int16_t string_index) {
    const char16_t *string = L"<missing string>";
    if(tag_index == NONE) {
        return string;
    }

    struct unicode_string_list_group_header *header = unicode_string_list_get_header(tag_index);
    if(string_index >= 0 && string_index < header->string_references.count) {
        struct unicode_string_list_string_reference *reference = unicode_string_list_get_string_reference(header, string_index);
        if(reference->string.size > 0) {
            string = (char16_t *)reference->string.address;
            if(string[(reference->string.size/sizeof(char16_t)) -1] != L'\0') {
                string = L"<invalid string>";
            }
        }
    }

    return string;
}
