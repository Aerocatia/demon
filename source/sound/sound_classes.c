#include "../cseries/cseries.h"
#include "../saved_games/game_state.h"

#include "sound_classes.h"

struct sound_class_datum {
    real desired_gain;
    real gain;
    int16_t ticks;
};

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

asm(".set _sound_class_data, 0x00F1BD14");
extern struct sound_class_datum *sound_class_data;

static struct sound_class_datum *sound_class_datum_get(int16_t index);

void sound_classes_initialize() {
    sound_class_data = game_state_malloc("sound classes", nullptr, NUMBER_OF_SOUND_CLASSES * sizeof(struct sound_class_datum));
}

static struct sound_class_datum *sound_class_datum_get(int16_t index) {
    assert(index >= 0 && index < NUMBER_OF_SOUND_CLASSES);
    assert(sound_class_data);

    return &sound_class_data[index];
}
