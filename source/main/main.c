#include "../cseries/cseries.h"
#include "../cseries/errors.h"
#include "../cseries/build_number.h"
#include "../cseries/platform.h"

#include "console.h"

#include "../tag_files/tag_files.h"

#include "../interface/hud.h"
#include "../scenario/scenario.h"

/* constants */

enum {
    SAFE_TO_SAVE_RETRY_TIME = (TICKS_PER_SECOND / 3),
    SAFE_TO_SAVE_MAXIMUM_TIME = (8 * TICKS_PER_SECOND),
    SAFE_TO_SAVE_MINIMUM_INTERVALS = 3,
    VBLANK_MAXIMUM_SKIP_RATE = 5,
    VBLANK_FAILURE_FRAME_COUNT = 4,
    VBLANK_RESUME_INTERVAL = 15,
    VBLANK_FLIP_DELTA_COUNT = 15,
    MAIN_MENU_FADE_TIME_MILLISECONDS = 1 * MILLISECONDS_PER_SECOND,
    INTERESTING_TIMEOUT_LOCK = 10 * MILLISECONDS_PER_SECOND,
    MAIN_LOSS_LATENCY = 90
};

/* globals */

struct _main_globals {
    int64_t last_time_clocks;
    uint32_t last_time_msec;
    int64_t last_render_clocks;
    int64_t last_vblank_index;
    int64_t last_initial_vblank_index;
    int64_t last_achievable_vblank_index;
    int64_t last_present_vblank_index;
    bool did_time_overflow_occur;
    real seconds_elapsed;
    int16_t connection;
    uint16_t screenshot_identifier;
    struct bitmap_data *movie;
    int32_t recording_start_tick;
    int32_t recording_stop_tick;
    int32_t recording_frame_index;
    real recording_dt;
    bool reset_map;
    bool rename_map;
    bool revert_map;
    bool skip_cinematic;
    bool save_map;
    bool save_map_safely;
    bool save_map_timeout;
    bool saving_map;
    int32_t ticks_until_next_save_check;
    int32_t ticks_unable_to_save;
    uint32_t map_change_load_timer;
    int16_t safe_intervals;
    bool won_map;
    bool lost_map;
    bool respawn;
    bool save_core;
    bool load_core;
    bool load_core_at_startup;
    int16_t switch_to_structure_bsp_index;
    bool main_menu_scenario_loaded;
    bool want_to_be_at_main_menu;
    bool run_xdemos;
    bool fade_to_dashboard;
    bool exit_to_dashboard;
    bool want_to_exit;
    int32_t idle_timeout;
    int32_t idle_last_interesting;
    int32_t idle_last_activity;
    bool playback_last_recording;
    uint8_t halt_time_scale;
    bool restart_time;
    bool load_last_solo_level;
    bool cutscene_skip;
    int16_t skip_ticks;
    int16_t loss_timer;
    int16_t respawn_timer;
    bool queue_map;
    uint8_t pad0[3];
    bool solo_try_and_load_from_persistent_storage;
    char soloplayer_map_name[TAG_FILE_NAME_LENGTH + 1];
    char multiplayer_map_name[TAG_FILE_NAME_LENGTH + 1];
    char queued_map_name[TAG_FILE_NAME_LENGTH + 1];
    bool want_to_connect;
    char connect_address[32];
    char connect_password[9];
    char core_file_name[64];
    int16_t vblank_interval_current;
    int16_t vblank_interval_minimum;
    uint16_t vblank_interval_held;
    int16_t vblank_failure_count[VBLANK_MAXIMUM_SKIP_RATE + 1];
    int64_t vblank_last_failure_time[VBLANK_MAXIMUM_SKIP_RATE + 1];
    uint32_t *vblank_flip_counter;
    int16_t vblank_flip_delta_next_index;
    int16_t vblank_flip_deltas[VBLANK_FLIP_DELTA_COUNT];
};
static_assert(sizeof(struct _main_globals) == 1136);

// confirmed matches for halo_cache_symbols.exe
static_assert(offsetof(struct _main_globals, switch_to_structure_bsp_index) == 0x74);
static_assert(offsetof(struct _main_globals, halt_time_scale) == 0x89);
static_assert(offsetof(struct _main_globals, restart_time) == 0x8A);
static_assert(offsetof(struct _main_globals, cutscene_skip) == 0x8C);
static_assert(offsetof(struct _main_globals, skip_ticks) == 0x8E);

asm(".set _main_globals, 0x00C996B0");
extern struct _main_globals main_globals;

/* public functions */

void main_switch_structure_bsp(int16_t new_structure_bsp_index) {
    auto scenario = global_scenario_get();
    if(new_structure_bsp_index < 0 || new_structure_bsp_index >= scenario->structure_bsp_references.count) {
        console_warning("tried to switch to invalid structure-bsp %d", new_structure_bsp_index);
    }
    else if(new_structure_bsp_index == global_structure_bsp_index_get()) {
        console_warning("tried to switch to current structure-bsp %d", new_structure_bsp_index);
    }
    else {
        main_globals.switch_to_structure_bsp_index = new_structure_bsp_index;
        hud_load(true);
    }
}

void main_skip(int16_t ticks) {
    if(ticks <= TICKS_PER_SECOND / 2) {
        main_globals.skip_ticks = ticks;
        main_globals.cutscene_skip = true;
    }
    else {
        error(_error_silent, "cannot skip more than 15 frames (half a second)");
    }
}

void main_stop_time() {
    main_globals.halt_time_scale = 0;
    main_globals.restart_time = false;
}

void main_start_time() {
    main_globals.restart_time = true;
}

void main_crash([[maybe_unused]] const char *str) {
    *((char **)nullptr) = "chucky was here!  NULL belongs to me!!!!!";
    abort(); // just in case
}

void main_print_version() {
    console_printf(false, TARGET_STRING " " PLATFORM_NAME_STRING " " BUILD_NUMBER " " __DATE__ " " __TIME__);
}
