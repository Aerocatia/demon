#include "../cseries/cseries.h"
#include "data_file.h"

#include "../main/exe_functions.h"

uint32_t data_file_find_item(int32_t data_file_id, const char *item_name) {
    return RUN_EXE_FUNCTION(data_file_find_item, data_file_id, item_name);
}

uint32_t data_file_load_tag(int32_t data_file_id, uint32_t item_index, void *tag_load_address) {
    return RUN_EXE_FUNCTION(data_file_load_tag, data_file_id, item_index, tag_load_address);
}
