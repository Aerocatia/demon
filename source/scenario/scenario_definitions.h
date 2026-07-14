#ifndef DEMON_SCENARIO_DEFINITIONS_H
#define DEMON_SCENARIO_DEFINITIONS_H

#include "../tag_files/tag_files.h"

enum {
    SCENARIO_GROUP_TAG = 0x73636E72, // 'scnr'
    SCENARIO_VERSION = 2,
    SCENARIO_CUSTOM_DECAL_ID = 0x626C6168 // 'blah'
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

static inline struct scenario *scenario_get(int32_t index) {
    return TAG_GET(SCENARIO_GROUP_TAG, index, struct scenario);
}

#endif
