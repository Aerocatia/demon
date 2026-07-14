#include "../cseries/cseries_windows.h"
#include "cache_files.h"

bool (*exe_cache_file_open)(const char *scenario_name, struct cache_file_header *header) = (void *)0x005173C0;
bool cache_file_open(const char *scenario_name, struct cache_file_header *header) {
    return exe_cache_file_open(scenario_name, header);
}

int16_t (*exe_cache_file_read)(int32_t tag_index, int32_t offset, int32_t size, void *buffer, struct cache_file_read_request_params *params, bool blocking, bool data_file) = (void *)0x005175D0;
int16_t cache_file_read(int32_t tag_index, int32_t offset, int32_t size, void *buffer, struct cache_file_read_request_params *params, bool blocking, bool data_file) {
    return exe_cache_file_read(tag_index, offset, size, buffer, params, blocking, data_file);
}

void (*exe_tags_header_register_vertex_and_index_buffers)(struct cache_file_tags_header *tags_header) = (void *)0x0051A100;
void tags_header_register_vertex_and_index_buffers(struct cache_file_tags_header *tags_header) {
    exe_tags_header_register_vertex_and_index_buffers(tags_header);
}
