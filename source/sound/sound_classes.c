#include "../cseries/cseries.h"
#include "../saved_games/game_state.h"

#include "sound_definitions.h"
#include "sound_classes.h"

#include "../demon/exe_globals.h"

struct sound_class_datum {
    real desired_gain;
    real gain;
    int16_t ticks;
};
static_assert(sizeof(struct sound_class_datum) == 12);

struct sound_class_definition sound_classes[NUMBER_OF_SOUND_CLASSES] = {
    [_sound_class_projectile_impact]                   = {6, 4, 100,  false, 4, _sound_cache_miss_mode_discard,  0.5f, 0.0f, 1.4f, 8.0f,   1.0f, 1.0f},
    [_sound_class_projectile_detonation]               = {4, 1, 200,  false, 5, _sound_cache_miss_mode_postpone, 0.5f, 0.0f, 8.0f, 120.0f, 1.0f, 1.0f},
    [_sound_class_weapon_fire]                         = {4, 1, 0,    false, 4, _sound_cache_miss_mode_postpone, 0.5f, 0.0f, 4.0f, 70.0f,  1.0f, 1.0f},
    [_sound_class_weapon_ready]                        = {4, 1, 500,  false, 4, _sound_cache_miss_mode_postpone, 0.5f, 0.0f, 1.0f, 9.0f,   1.0f, 1.0f},
    [_sound_class_weapon_reload]                       = {4, 1, 500,  false, 4, _sound_cache_miss_mode_postpone, 0.5f, 0.0f, 1.0f, 9.0f,   1.0f, 1.0f},
    [_sound_class_weapon_empty]                        = {4, 1, 60,   false, 4, _sound_cache_miss_mode_postpone, 0.5f, 0.0f, 1.0f, 9.0f,   1.0f, 1.0f},
    [_sound_class_weapon_charge]                       = {4, 1, 500,  false, 4, _sound_cache_miss_mode_postpone, 0.5f, 0.0f, 1.0f, 9.0f,   1.0f, 1.0f},
    [_sound_class_weapon_overheat]                     = {4, 1, 500,  false, 4, _sound_cache_miss_mode_postpone, 0.5f, 0.0f, 1.0f, 9.0f,   1.0f, 1.0f},
    [_sound_class_weapon_idle]                         = {4, 1, 500,  false, 4, _sound_cache_miss_mode_postpone, 0.5f, 0.0f, 1.0f, 9.0f,   1.0f, 1.0f},
    [_sound_class_object_impacts]                      = {4, 1, 100,  false, 3, _sound_cache_miss_mode_postpone, 0.5f, 0.0f, 0.5f, 3.0f,   0.0f, 1.0f},
    [_sound_class_particle_impacts]                    = {4, 1, 100,  false, 3, _sound_cache_miss_mode_discard,  0.5f, 0.0f, 0.5f, 3.0f,   0.0f, 1.0f},
    [_sound_class_slow_impacts]                        = {4, 1, 1000, false, 3, _sound_cache_miss_mode_discard,  0.5f, 0.0f, 0.5f, 3.0f,   0.0f, 1.0f},
    [_sound_class_footstep]                            = {4, 1, 200,  false, 3, _sound_cache_miss_mode_discard,  0.5f, 0.0f, 0.9f, 10.0f,  1.0f, 1.0f},
    [_sound_class_unit_dialog]                         = {4, 1, 100,  true,  3, _sound_cache_miss_mode_postpone, 0.8f, 0.0f, 3.0f, 20.0f,  0.0f, 1.0f},
    [_sound_class_vehicle_impact]                      = {4, 2, 400,  false, 3, _sound_cache_miss_mode_discard,  0.5f, 0.0f, 1.4f, 8.0f,   1.0f, 1.0f},
    [_sound_class_vehicle_engine]                      = {4, 2, 100,  false, 3, _sound_cache_miss_mode_postpone, 0.9f, 0.0f, 1.4f, 8.0f,   1.0f, 1.0f},
    [_sound_class_device_door]                         = {4, 1, 100,  false, 2, _sound_cache_miss_mode_postpone, 0.5f, 0.0f, 0.9f, 5.0f,   1.0f, 1.0f},
    [_sound_class_device_force_field]                  = {4, 1, 100,  false, 2, _sound_cache_miss_mode_postpone, 0.5f, 0.0f, 0.9f, 5.0f,   1.0f, 1.0f},
    [_sound_class_device_machinery]                    = {4, 1, 100,  false, 1, _sound_cache_miss_mode_postpone, 0.5f, 0.0f, 0.9f, 5.0f,   1.0f, 1.0f},
    [_sound_class_device_nature]                       = {4, 1, 100,  false, 1, _sound_cache_miss_mode_postpone, 0.5f, 0.0f, 0.9f, 5.0f,   1.0f, 1.0f},
    [_sound_class_device_computers]                    = {4, 1, 100,  false, 1, _sound_cache_miss_mode_postpone, 0.5f, 0.0f, 0.5f, 3.0f,   1.0f, 1.0f},
    [_sound_class_music]                               = {4, 4, 100,  false, 2, _sound_cache_miss_mode_postpone, 1.0f, 0.0f, 0.9f, 5.0f,   0.0f, 1.0f},
    [_sound_class_ambient_nature]                      = {4, 1, 100,  false, 1, _sound_cache_miss_mode_postpone, 1.0f, 0.0f, 0.9f, 5.0f,   0.0f, 1.0f},
    [_sound_class_ambient_machinery]                   = {4, 1, 100,  false, 1, _sound_cache_miss_mode_postpone, 1.0f, 0.0f, 0.9f, 5.0f,   0.0f, 1.0f},
    [_sound_class_ambient_computers]                   = {4, 1, 100,  false, 1, _sound_cache_miss_mode_postpone, 1.0f, 0.0f, 0.5f, 3.0f,   0.0f, 1.0f},
    [_sound_class_player_hurt]                         = {4, 1, 100,  false, 4, _sound_cache_miss_mode_postpone, 1.0f, 0.0f, 0.5f, 3.0f,   1.0f, 1.0f},
    [_sound_class_scripted_dialog_to_player]           = {4, 4, 100,  true,  6, _sound_cache_miss_mode_postpone, 0.8f, 0.0f, 3.0f, 20.0f,  0.0f, 1.0f},
    [_sound_class_scripted_other]                      = {4, 4, 100,  false, 3, _sound_cache_miss_mode_postpone, 0.8f, 0.0f, 2.0f, 5.0f,   0.0f, 1.0f},
    [_sound_class_scripted_dialog_to_other]            = {4, 4, 100,  true,  5, _sound_cache_miss_mode_postpone, 0.8f, 0.0f, 3.0f, 20.0f,  0.0f, 1.0f},
    [_sound_class_scripted_dialog_force_unspatialized] = {4, 4, 100,  true,  6, _sound_cache_miss_mode_postpone, 0.8f, 0.0f, 3.0f, 20.0f,  0.0f, 1.0f},
    [_sound_class_game_event]                          = {4, 1, 100,  false, 5, _sound_cache_miss_mode_postpone, 1.0f, 0.0f, 3.0f, 20.0f,  1.0f, 1.0f},
};

const char *sound_class_names[NUMBER_OF_SOUND_CLASSES] = {
    "projectile_impact",
    "projectile_detonation",
    "",
    "",
    "weapon_fire",
    "weapon_ready",
    "weapon_reload",
    "weapon_empty",
    "weapon_charge",
    "weapon_overheat",
    "weapon_idle",
    "",
    "",
    "object_impacts",
    "particle_impacts",
    "slow_particle_impacts",
    "",
    "",
    "unit_footsteps",
    "unit_dialog",
    "",
    "",
    "vehicle_collision",
    "vehicle_engine",
    "",
    "",
    "device_door",
    "device_force_field",
    "device_machinery",
    "device_nature",
    "device_computers",
    "",
    "music",
    "ambient_nature",
    "ambient_machinery",
    "ambient_computers",
    "",
    "",
    "",
    "first_person_damage",
    "",
    "",
    "",
    "",
    "scripted_dialog_player",
    "scripted_effect",
    "scripted_dialog_other",
    "scripted_dialog_force_unspatialized",
    "",
    "",
    "game_event"
};

/* globals */

#ifndef DEMON_EXE_GLOBALS
static struct sound_class_datum *sound_class_data;
#endif

/* forward declarations */

static struct sound_class_datum *sound_class_datum_get(int16_t index);

/* public functions */

void sound_classes_initialize() {
    sound_class_data = game_state_malloc("sound classes", nullptr, NUMBER_OF_SOUND_CLASSES * sizeof(struct sound_class_datum));
}

void sound_classes_initialize_for_new_map() {
    for(int class_index = 0; class_index < NUMBER_OF_SOUND_CLASSES; class_index++) {
        auto class_datum = sound_class_datum_get(class_index);
        class_datum->gain = 1.0f;
        class_datum->desired_gain = 1.0f;
        class_datum->ticks = 0;
    }
}

void sound_classes_dispose_from_old_map() {}

void sound_classes_dispose() {
    sound_class_data = nullptr;
}

void sound_classes_update(int32_t ticks_elapsed) {
    if(ticks_elapsed <= 0) {
        return;
    }

    for(int class_index = 0; class_index < NUMBER_OF_SOUND_CLASSES; class_index++) {
        auto class_datum = sound_class_datum_get(class_index);
        assert(valid_sound_gain(class_datum->gain));
        if(class_datum->ticks > ticks_elapsed) {
            class_datum->gain += (class_datum->desired_gain - class_datum->gain) * ((real)ticks_elapsed / class_datum->ticks);
            class_datum->ticks -= ticks_elapsed;
        }
        else {
            class_datum->gain = class_datum->desired_gain;
            class_datum->ticks = 0;
        }

        assert(valid_sound_gain(class_datum->gain));
    }
}

real sound_class_get_gain(int16_t class_index) {
    return sound_class_datum_get(class_index)->gain;
}

void sound_class_set_gain(const char *substring, real gain, int16_t ticks) {
    for(int class_index = 0; class_index < NUMBER_OF_SOUND_CLASSES; class_index++) {
        if(sound_class_names[class_index][0] && strstr(sound_class_names[class_index], substring)) {
            auto class_datum = sound_class_datum_get(class_index);
            class_datum->desired_gain = PIN(gain, 0.0f, 1.0f);
            class_datum->ticks = MAX(0, ticks);
        }
    }
}

#ifdef DEBUG_BUILD
void debug_sound_classes_enable(const char *substring, bool enabled) {
    for(int class_index = 0; class_index < NUMBER_OF_SOUND_CLASSES; class_index++) {
        if(sound_class_names[class_index][0] && strstr(sound_class_names[class_index], substring)) {
            sound_class_get(class_index)->disabled = !enabled;
        }
    }
}

void debug_sound_classes_set_distances(const char *substring, real minimum_distance, real maximum_distance) {
    for(int class_index = 0; class_index < NUMBER_OF_SOUND_CLASSES; class_index++) {
        if(sound_class_names[class_index][0] && strstr(sound_class_names[class_index], substring)) {
            auto sound_class = sound_class_get(class_index);
            sound_class->minimum_distance = minimum_distance;
            sound_class->maximum_distance = maximum_distance;
        }
    }
}

void debug_sound_classes_set_wet(const char *substring, real wet) {
    for(int class_index = 0; class_index < NUMBER_OF_SOUND_CLASSES; class_index++) {
        if(sound_class_names[class_index][0] && strstr(sound_class_names[class_index], substring)) {
            sound_class_get(class_index)->reverb_damping_factor = PIN(1.0f - wet, 0.0f, 1.0f);
        }
    }
}
#endif

/* private functions */

static struct sound_class_datum *sound_class_datum_get(int16_t index) {
    assert(index >= 0 && index < NUMBER_OF_SOUND_CLASSES);
    assert(sound_class_data);

    return &sound_class_data[index];
}
