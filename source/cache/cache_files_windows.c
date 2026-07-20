#include "../cseries/cseries_windows.h"
#include "cache_files.h"

#include "../main/exe_functions.h"

bool cache_file_open(const char *scenario_name, struct cache_file_header *header) {
    return RUN_EXE_FUNCTION(cache_file_open, scenario_name, header);
}

int16_t cache_file_read(int32_t tag_index, int32_t offset, int32_t size, void *buffer, struct cache_file_read_request_params *params, bool blocking, bool data_file) {
    return RUN_EXE_FUNCTION(cache_file_read, tag_index, offset, size, buffer, params, blocking, data_file);
}

void tags_header_register_vertex_and_index_buffers(struct cache_file_tags_header *tags_header) {
    RUN_EXE_FUNCTION(tags_header_register_vertex_and_index_buffers, tags_header);
}
