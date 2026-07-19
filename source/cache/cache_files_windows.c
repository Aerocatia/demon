#include "../cseries/cseries_windows.h"
#include "cache_files.h"

bool cache_file_open(const char *scenario_name, struct cache_file_header *header) {
    typeof(cache_file_open) *FIXME_EXE_FUNCTION_POINTER = (void *)0x005173C0;
    return FIXME_EXE_FUNCTION_POINTER(scenario_name, header);
}

int16_t cache_file_read(int32_t tag_index, int32_t offset, int32_t size, void *buffer, struct cache_file_read_request_params *params, bool blocking, bool data_file) {
    typeof(cache_file_read) *FIXME_EXE_FUNCTION_POINTER = (void *)0x005175D0;
    return FIXME_EXE_FUNCTION_POINTER(tag_index, offset, size, buffer, params, blocking, data_file);
}

void tags_header_register_vertex_and_index_buffers(struct cache_file_tags_header *tags_header) {
    typeof(tags_header_register_vertex_and_index_buffers) *FIXME_EXE_FUNCTION_POINTER = (void *)0x0051A100;
    FIXME_EXE_FUNCTION_POINTER(tags_header);
}
