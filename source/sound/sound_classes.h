#ifndef DEMON_SOUND_CLASSES_H
#define DEMON_SOUND_CLASSES_H

#include "../cseries/cseries.h"

enum {
    MAXIMUM_SOUND_INSTANCES_PER_DEFINITION = 16,
    MAXIMUM_SOUND_INSTANCES_PER_OBJECT_PER_DEFINITION = 16
};

enum {
    _sound_class_projectile_impact,
    _sound_class_projectile_detonation,
    _sound_class_projectile_unused0,
    _sound_class_projectile_unused1,
    _sound_class_weapon_fire,
    _sound_class_weapon_ready,
    _sound_class_weapon_reload,
    _sound_class_weapon_empty,
    _sound_class_weapon_charge,
    _sound_class_weapon_overheat,
    _sound_class_weapon_idle,
    _sound_class_weapon_unused0,
    _sound_class_weapon_unused1,
    _sound_class_object_impacts,
    _sound_class_particle_impacts,
    _sound_class_slow_impacts,
    _sound_class_effect_unused2,
    _sound_class_effect_unused3,
    _sound_class_footstep,
    _sound_class_unit_dialog,
    _sound_class_unit_unused0,
    _sound_class_unit_unused1,
    _sound_class_vehicle_impact,
    _sound_class_vehicle_engine,
    _sound_class_vehicle_unused0,
    _sound_class_vehicle_unused1,
    _sound_class_device_door,
    _sound_class_device_force_field,
    _sound_class_device_machinery,
    _sound_class_device_nature,
    _sound_class_device_computers,
    _sound_class_device_unused1,
    _sound_class_music,
    _sound_class_ambient_nature,
    _sound_class_ambient_machinery,
    _sound_class_ambient_computers,
    _sound_class_marty_unused1,
    _sound_class_marty_unused2,
    _sound_class_marty_unused3,
    _sound_class_player_hurt,
    _sound_class_player_unused0,
    _sound_class_player_unused1,
    _sound_class_player_unused2,
    _sound_class_player_unused3,
    _sound_class_scripted_dialog_to_player,
    _sound_class_scripted_other,
    _sound_class_scripted_dialog_to_other,
    _sound_class_scripted_dialog_force_unspatialized,
    _sound_class_scripted_unused2,
    _sound_class_scripted_unused3,
    _sound_class_game_event,
    NUMBER_OF_SOUND_CLASSES
};

enum {
    _sound_cache_miss_mode_discard,
    _sound_cache_miss_mode_postpone,
    NUMBER_OF_SOUND_CACHE_MISS_MODES
};

struct sound_class_definition {
    int16_t maximum_number_per_definition;
    int16_t maximum_number_per_object;
    int32_t preemption_time;
    bool speech;
    int16_t priority;
    int16_t cache_miss_mode;
    real reverb_damping_factor;
    real effect_damping_factor;
    real minimum_distance;
    real maximum_distance;
    real gain_lower_bound;
    real gain_upper_bound;
    bool disabled;
};
static_assert(sizeof(struct sound_class_definition) == 44);

extern struct sound_class_definition sound_classes[NUMBER_OF_SOUND_CLASSES];
extern const char *sound_class_names[NUMBER_OF_SOUND_CLASSES];

#ifdef DEBUG_BUILD
void debug_sound_classes_enable(const char *substring, bool enabled);
void debug_sound_classes_set_distances(const char *substring, real minimum_distance, real maximum_distance);
void debug_sound_classes_set_wet(const char *substring, real wet);
#endif

static inline struct sound_class_definition *sound_class_get(int16_t class_index) {
    assert(class_index >= 0 && class_index < NUMBER_OF_SOUND_CLASSES);
    assert(sound_class_names[class_index][0]);

    struct sound_class_definition *definition = &sound_classes[class_index];
    assert(definition->maximum_number_per_definition <= MAXIMUM_SOUND_INSTANCES_PER_DEFINITION);
    assert(definition->maximum_number_per_object <= MAXIMUM_SOUND_INSTANCES_PER_OBJECT_PER_DEFINITION);

    return definition;
}

#endif
