#ifndef DEMON_SCENARIO_DEFINITIONS_H
#define DEMON_SCENARIO_DEFINITIONS_H

#include "../cseries/cseries.h"
#include "../tag_files/tag_files.h"

enum {
    SCENARIO_GROUP_TAG = 0x73636E72, // 'scnr'
    SCENARIO_VERSION = 2,
    SCENARIO_CUSTOM_DECAL_ID = 0x626C6168 // 'blah'
};

enum {
    MAXIMUM_SCENARIO_PLAYERS_PER_BLOCK = 256,
    MAXIMUM_TRIGGER_VOLUMES_PER_SCENARIO = 256,
    MAXIMUM_FUNCTIONS_PER_SCENARIO = 32,
    MAXIMUM_OBJECT_NAMES_PER_SCENARIO = 512,
    MAXIMUM_DEVICE_GROUPS_PER_SCENARIO = 128,
    MAXIMUM_SCENARIO_OBJECT_PALETTE_ENTRIES_PER_BLOCK = 100,
    MAXIMUM_CUTSCENE_FLAGS_PER_SCENARIO = 512,
    MAXIMUM_CUTSCENE_CAMERA_POINTS_PER_SCENARIO = 512,
    MAXIMUM_CUTSCENE_TITLES_PER_SCENARIO = 64,
    MAXIMUM_SCENARIO_NETGAME_FLAGS_PER_SCENARIO = 200,
    MAXIMUM_SCENARIO_NETGAME_EQUIPMENT_PER_SCENARIO = 200,
    MAXIMUM_SCENARIO_STARTING_EQUIPMDNG_PER_SCENARIO = 200,
    MAXIMUM_STRUCTURE_BSPS_PER_SCENARIO = 16,
    MAXIMUM_CHILD_SCENARIOS_PER_SCENARIO = 16,
    MAXIMUM_SKIES_PER_SCENARIO = 8,
    MAXIMUM_DECALS_PER_SCENARIO = 64 * KIB,
    MAXIMUM_DECAL_PALETTES_PER_SCENARIO = 128,
    MAXIMUM_SCENERY_DATUMS_PER_SCENARIO = 2000,
    MAXIMUM_SOUND_SCENERY_DATUMS_PER_SCENARIO = 256,
    MAXIMUM_WEAPON_DATUMS_PER_SCENARIO = 128,
    MAXIMUM_EQUIPMENT_DATUMS_PER_SCENARIO = 256,
    MAXIMUM_BIPED_DATUMS_PER_SCENARIO = 128,
    MAXIMUM_VEHICLE_DATUMS_PER_SCENARIO = 80,
    MAXIMUM_MACHINE_DATUMS_PER_SCENARIO = 400,
    MAXIMUM_CONTROL_DATUMS_PER_SCENARIO = 100,
    MAXIMUM_LIGHT_FIXTURE_DATUMS_PER_SCENARIO = 500,
    MAXIMUM_EDITOR_SCENARIO_DATA_SIZE = 64 * KIB,
    MAXIMUM_EDITOR_COMMENTS = KIB,
    MAXIMUM_EDITOR_COMMENT_LENGTH = 16 * KIB
};

enum {
    _scenario_type_solo,
    _scenario_type_multiplayer,
    _scenario_type_main_menu,
    NUMBER_OF_SCENARIO_TYPES
};

enum {
    _scenario_cortana_hack_bit,
    _scenario_demo_ui_bit,
    NUMBER_OF_SCENARIO_FLAGS
};

struct scenario {
    struct tag_reference ugly_structure_bsp;
    struct tag_reference unloved_globals;
    struct tag_reference bad_sky;
    struct tag_block sky_references;
    int16_t type;
    uint16_t flags;
    struct tag_block scenario_references;
    real local_north;
    uint32_t header_unused[5];
    int32_t reference_unused[34];
    struct tag_block predicted_ui_resources;
    struct tag_block functions;
    struct tag_data editor_scenario_data;
    struct tag_block comments;
    int32_t user_edit_unused[56];
    struct tag_block object_names;
    struct tag_block scenery;
    struct tag_block scenery_palette;
    struct tag_block bipeds;
    struct tag_block biped_palette;
    struct tag_block vehicles;
    struct tag_block vehicle_palette;
    struct tag_block equipment;
    struct tag_block equipment_palette;
    struct tag_block weapons;
    struct tag_block weapon_palette;
    struct tag_block device_groups;
    struct tag_block machines;
    struct tag_block machine_palette;
    struct tag_block controls;
    struct tag_block control_palette;
    struct tag_block light_fixtures;
    struct tag_block light_fixtures_palette;
    struct tag_block sound_scenery;
    struct tag_block sound_scenery_palette;
    struct tag_block unused_blocks[7];
    struct tag_block starting_profiles;
    struct tag_block players;
    struct tag_block trigger_volumes;
    struct tag_block recorded_animations;
    struct tag_block netgame_flags;
    struct tag_block netgame_equipment;
    struct tag_block scenario_starting_equipment;
    struct tag_block bsp_switch_trigger_volumes;
    struct tag_block decals;
    struct tag_block decal_palette;
    struct tag_block detail_object_collection_palette;
    int32_t render_unused[21];
    struct tag_block ai_actor_palette;
    struct tag_block ai_encounters;
    struct tag_block ai_command_lists;
    struct tag_block ai_animation_references;
    struct tag_block ai_script_references;
    struct tag_block ai_recording_references;
    struct tag_block ai_conversations;
    struct tag_data hs_syntax_data;
    struct tag_data hs_string_constants;
    struct tag_block hs_scripts;
    struct tag_block hs_globals;
    struct tag_block hs_references;
    struct tag_block hs_source_files;
    int32_t scripting_unused[6];
    struct tag_block cutscene_flags;
    struct tag_block cutscene_camera_points;
    struct tag_block cutscene_chapter_titles;
    int32_t rapidly_dwindling_unused_space[27];
    struct tag_reference custom_object_names;
    struct tag_reference ingame_help_text;
    struct tag_reference hud_messages;
    struct tag_block structure_bsp_references;
};
static_assert(sizeof(struct scenario) == 1456);

struct scenario_structure_bsp_reference {
    int32_t offset;
    int32_t size;
    void *address;
    uint32_t unused[1];
    struct tag_reference structure_bsp;
};
static_assert(sizeof(struct scenario_structure_bsp_reference) == 32);

struct scenario_child_scenario_reference {
    struct tag_reference scenario;
    uint32_t unused[4];
};
static_assert(sizeof(struct scenario_child_scenario_reference) == 32);

enum {
    _scenario_function_scripted_bit,
    _scenario_function_invert_function_bit,
    _scenario_function_additive_bit,
    _scenario_function_does_not_deactivate_below_lower_bound_bit,
    NUMBER_OF_SCENARIO_FUNCTION_DEFINITION_FLAGS
};

enum {
    _scenario_function_clip_to_bounds,
    _scenario_function_clip_to_bounds_and_normalize,
    _scenario_function_scale_to_fit_bounds,
    NUMBER_OF_SCENARIO_FUNCTION_BOUNDS_MODES
};

struct scenario_function {
    uint32_t flags;
    char name[TAG_STRING_LENGTH + 1];
    real period;
    int16_t scale_period_by_function_index;
    int16_t function_type;
    int16_t scale_function_by_function_index;
    int16_t wobble_function_type;
    real wobble_period;
    real wobble_magnitude;
    real square_wave_threshold;
    int16_t step_count;
    int16_t map_result_to_transition_function;
    int16_t sawtooth_count;
    uint16_t pad1;
    int16_t scale_result_by_function_index;
    int16_t bounds_mode;
    real lower_bound;
    real upper_bound;
    real pad2;
    uint16_t pad3;
    int16_t turn_off_with_function_index;
    int32_t unused1[4];
    real runtime_reciprocal_bounds_range;
    real runtime_reciprocal_sawtooth_count;
    real runtime_reciprocal_step_count;
    real runtime_one_over_period;
};
static_assert(sizeof(struct scenario_function) == 120);

struct editor_comment_definition {
    real_point3d position;
    int32_t unused[4];
    struct tag_data comment;
};
static_assert(sizeof(struct editor_comment_definition) == 48);

enum {
    _netgame_flag_ctf_flag,
    _netgame_flag_ctf_vehicle,
    _netgame_flag_oddball_ball_spawn,
    _netgame_flag_race_track,
    _netgame_flag_race_vehicle,
    _netgame_flag_vegas_bank,
    _netgame_flag_teleporter_source,
    _netgame_flag_teleporter_target,
    _netgame_flag_hill,
    NUMBER_OF_NETGAME_FLAG_TYPES
};

struct scenario_netgame_flag {
    real_point3d position;
    real facing;
    int16_t type;
    int16_t team_index;
    int32_t unused[32];
};
static_assert(sizeof(struct scenario_netgame_flag) == 148);

enum {
    _netgame_equipment_flag_float,
    NUMBER_OF_NETGAME_EQUIPMENT_FLAGS
};

struct scenario_netgame_equipment {
    int32_t flags;
    int16_t game_type[4];
    int16_t team_index;
    int16_t spawn_time;
    int32_t run_time_spawned_item_index;
    int32_t unused1[11];
    real_point3d position;
    real facing;
    struct tag_reference item_collection;
    int32_t unused2[12];
};
static_assert(sizeof(struct scenario_netgame_equipment) == 144);

enum {
    _netgame_starting_equipment_flag_no_grenades_bit,
    _netgame_starting_equipment_flag_plasma_greandes_bit,
    NUMBER_OF_NETGAME_STARTING_EQUIPMENT_FLAGS
};

struct scenario_starting_equipment {
    int32_t flags;
    int16_t game_type[4];
    int32_t unused1[12];
    struct tag_reference item_collection[6];
    int32_t unused2[12];
};
static_assert(sizeof(struct scenario_starting_equipment) == 204);

struct starting_weapon_info {
    struct tag_reference weapon;
    int16_t rounds_loaded;
    int16_t rounds_total;
};
static_assert(sizeof(struct starting_weapon_info) == 20);

struct scenario_starting_profile {
    char name[TAG_STRING_LENGTH + 1];
    real starting_health;
    real starting_shield;
    struct starting_weapon_info starting_weapons[2];
    uint8_t starting_grenade_counts[4];
    int32_t unused[5];
};
static_assert(sizeof(struct scenario_starting_profile) == 104);

struct scenario_player {
    real_point3d position;
    real facing;
    int16_t team_index;
    int16_t bsp_index;
    int16_t game_type[4];
    int32_t unused[6];
};
static_assert(sizeof(struct scenario_player) == 52);

enum {
    _trigger_volume_type_world_aligned_bounding_box,
    _trigger_volume_type_bounding_box,
    NUMBER_OF_TRIGGER_VOLUME_TYPES
};

struct scenario_trigger_volume {
    int16_t type;
    uint16_t pad;
    char name[TAG_STRING_LENGTH + 1];
    union {
        struct {
            int32_t unused[3];
            real_vector3d forward;
            real_vector3d up;
            real_point3d position;
            real_vector3d extents;
        } bounding_box;
        struct {
            int32_t unused[9];
            real_rectangle3d rectangle;
        } world_aligned_bounding_box;
    };
};
static_assert(sizeof(struct scenario_trigger_volume) == 96);

struct scenario_bsp_switch_trigger_volume {
    int16_t trigger_volume_index;
    int16_t source_bsp_index;
    int16_t destination_bsp_index;
    int16_t safe_flag_index;
};
static_assert(sizeof(struct scenario_bsp_switch_trigger_volume) == 8);

struct scenario_decal_palette_entry {
    struct tag_reference reference;
};
static_assert(sizeof(struct scenario_decal_palette_entry) == 16);

struct scenario_decal {
    int16_t palette_index;
    int8_t yaw;
    int8_t pitch;
    real_point3d position;
};
static_assert(sizeof(struct scenario_decal) == 16);

struct scenario_cutscene_flag {
    int32_t flags;
    char name[TAG_STRING_LENGTH + 1];
    real_point3d position;
    real_euler_angles2d facing;
    int32_t unused[9];
};
static_assert(sizeof(struct scenario_cutscene_flag) == 92);

struct scenario_cutscene_camera_point {
    int32_t flags;
    char name[TAG_STRING_LENGTH + 1];
    int32_t pad;
    real_point3d position;
    real_euler_angles3d orientation;
    real field_of_view;
    int32_t unused[9];
};
static_assert(sizeof(struct scenario_cutscene_camera_point) == 104);

struct scenario_cutscene_title {
    int32_t flags;
    char name[TAG_STRING_LENGTH + 1];
    int32_t pad0;
    rectangle2d bounds;
    int16_t text_index;
    int16_t style;
    int16_t justification;
    int16_t pad1;
    uint32_t text_flags;
    pixel32 foreground_color;
    pixel32 shadow_color;
    real fade_in_time;
    real up_time;
    real fade_out_time;
    int32_t unused[4];
};
static_assert(sizeof(struct scenario_cutscene_title) == 96);

struct scenario_detail_object_collection_palette_entry {
    struct tag_reference reference;
    long unused[8];
};
static_assert(sizeof(struct scenario_detail_object_collection_palette_entry) == 48);

/* functions */

static inline struct scenario *scenario_get(int32_t tag_index) {
    return tag_get(SCENARIO_GROUP_TAG, tag_index);
}

#endif
