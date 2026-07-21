#include <stdcountof.h>

#include "exe_functions.h"

#ifdef CACHE_FILE_BUILD
const void *exe_function_table[] = {
    [_exe_function_cache_file_open] = (void *)0x005173C0,
    [_exe_function_cache_file_read] = (void *)0x005175D0,
    [_exe_function_data_file_find_item] = (void *)0x0051B290,
    [_exe_function_data_file_load_tag] = (void *)0x0051BE90,
    [_exe_function_debug_malloc] = (void *)0x005509F0,
    [_exe_function_debug_free] = (void *)0x00550860,
    [_exe_function_errors_debug_file_path] = (void *)0x0075BB30,
    [_exe_function_keystone_dispose] = (void *)0x0086D320,
    [_exe_function_real_random_range] = (void *)0x0042F360,
    [_exe_function_sound_cache_open] = (void *)0x0051DF90,
    [_exe_function_stack_walk_dispose] = (void *)0x005593F0,
    [_exe_function_stack_walk_initialize] = (void *)0x00559480,
    [_exe_function_strncmp_case_insensitive] = (void *)0x0054C1B0,
    [_exe_function_system_milliseconds] = (void *)0x0054F1E0,
    [_exe_function_tags_header_register_vertex_and_index_buffers] = (void *)0x0051A100,
    [_exe_function_texture_cache_open] = (void *)0x0051FD20
};
static_assert(countof(exe_function_table) == NUMBER_OF_EXE_FUNCTIONS);
#endif
