#ifndef DEMON_EXE_GLOBALS_H
#define DEMON_EXE_GLOBALS_H

// "Well, that does it."

#ifdef CACHE_FILE_BUILD
#define DEMON_EXE_GLOBALS
asm(".set _cache_file_globals, 0x00AF8368");
extern struct cache_file_global_data cache_file_globals;

asm(".set _global_tag_instances, 0x00AF8364");
extern struct cache_file_tag_instance *global_tag_instances;

asm(".set _error_globals, 0x00B016C8");
extern struct error_global_data error_globals;

asm(".set _game_state_globals, 0x00F14600");
extern struct game_state_global_data game_state_globals;

asm(".set _global_scenario, 0x00F1A67C");
extern struct scenario *global_scenario;

asm(".set _global_structure_bsp_index, 0x00A39C68");
extern int16_t global_structure_bsp_index;

asm(".set _main_globals, 0x00C996B0");
extern struct _main_globals main_globals;

asm(".set _physical_memory_map_globals, 0x00AFF024");
extern struct physical_memory_map_global_data physical_memory_map_globals;

asm(".set _sound_class_data, 0x00F1BD14");
extern struct sound_class_datum *sound_class_data;
#endif

#endif
