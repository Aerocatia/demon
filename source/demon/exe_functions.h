#ifndef DEMON_EXE_FUNCTIONS_H
#define DEMON_EXE_FUNCTIONS_H

enum {
    _exe_function_cache_file_open,
    _exe_function_cache_file_read,
    _exe_function_data_file_find_item,
    _exe_function_data_file_load_tag,
    _exe_function_debug_malloc,
    _exe_function_debug_free,
    _exe_function_errors_debug_file_path,
    _exe_function_game_state_allocate_buffer,
    _exe_function_game_state_create_or_open_file,
    _exe_function_hud_load,
    _exe_function_keystone_dispose,
    _exe_function_real_random_range,
    _exe_function_sound_cache_open,
    _exe_function_stack_walk_dispose,
    _exe_function_stack_walk_initialize,
    _exe_function_strncmp_case_insensitive,
    _exe_function_system_milliseconds,
    _exe_function_tags_header_register_vertex_and_index_buffers,
    _exe_function_texture_cache_open,
    NUMBER_OF_EXE_FUNCTIONS
};

extern const void *exe_function_table[];

#define EXE_FUNCTION_GET(function) (typeof(function) *)exe_function_table[_exe_function_##function]
#define RUN_EXE_FUNCTION(function, ...) (EXE_FUNCTION_GET(function))(__VA_ARGS__)

#endif
