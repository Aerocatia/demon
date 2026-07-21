#ifndef DEMON_ACTOR_DEFINITIONS_H
#define DEMON_ACTOR_DEFINITIONS_H

#include "../cseries/cseries.h"
#include "../tag_files/tag_groups.h"
#include "../units/unit_metagame.h"

enum {
    ACTOR_DEFINITION_TAG = 0x61637472, // 'actr'
    ACTOR_DEFINITION_VERSION = 2,
    ACTOR_VARIANT_DEFINITION_TAG = 0x61637476, // 'actv'
    ACTOR_VARIANT_DEFINITION_VERSION = 1
};

/* actor */

enum {
    _actor_definition_can_see_in_darkness_bit,
    _actor_definition_sneak_uncovering_target_bit,
    _actor_definition_sneak_uncovering_pursuit_position_bit,
    _actor_definition_unused_bit,
    _actor_definition_shoot_at_targets_last_location_bit,
    _actor_definition_crouch_try_not_to_move_bit,
    _actor_definition_crouch_when_noncombat_bit,
    _actor_definition_crouch_when_guarding_bit,
    _actor_definition_unused2_bit,
    _actor_definition_must_crouch_to_fire_bit,
    _actor_definition_panic_on_surprise_bit,
    _actor_definition_always_charge_bit,
    _actor_definition_helps_players_in_vehicles_bit,
    _actor_definition_start_firing_before_aligned_bit,
    _actor_definition_standing_must_move_forward_bit,
    _actor_definition_crouching_must_move_forward_bit,
    _actor_definition_defensive_crouch_while_charging_bit,
    _actor_definition_stalking_behavior_bit,
    _actor_definition_stalking_freeze_when_exposed_bit,
    _actor_definition_always_berserk_in_attacking_mode_bit,
    _actor_definition_berserk_use_panic_movement_bit,
    _actor_definition_flying_bit,
    _actor_definition_panicked_by_unopposable_enemy_bit,
    _actor_definition_crouch_when_hiding_from_unopposable_enemy_bit,
    _actor_definition_charge_in_attacking_mode_bit,
    _actor_definition_dive_off_ledges_bit,
    _actor_definition_swarm_actor_bit,
    _actor_definition_suicidal_melee_attack_bit,
    _actor_definition_cannot_move_crouching_bit,
    _actor_definition_fixed_crouch_facing_bit,
    _actor_definition_crouch_in_line_of_fire_bit,
    _actor_definition_avoid_friend_line_of_fire_bit,
    NUMBER_OF_ACTOR_DEFINITION_FLAGS
};

enum {
    _actor_definition_flags2_avoid_all_enemy_attack_vectors_bit,
    _actor_definition_flags2_must_stand_to_fire_bit,
    _actor_definition_flags2_must_stop_to_fire_bit,
    _actor_definition_flags2_disallow_vehicle_combat_bit,
    _actor_definition_flags2_pathfinding_ignores_danger_bit,
    _actor_definition_flags2_panic_in_groups_bit,
    _actor_definition_flags2_no_corpse_shooting_bit,
    NUMBER_OF_ACTOR_DEFINITION_FLAGS2
};

struct actor_perception_properties {
    real maximum_vision_distance;
    real central_vision_angle;
    real maximum_vision_angle;
    uint32_t unused;
    real peripheral_vision_angle;
    real peripheral_distance;
    uint32_t unused4;
    real_vector3d gun_offset_stand;
    real_vector3d gun_offset_crouch;
    real hearing_distance;
    real notice_projectile_chance;
    real notice_vehicle_chance;
    uint32_t unused2[2];
    real combat_awareness_time;
    real guard_awareness_time;
    real non_combat_awareness_time;
    real runtime_awareness_delta_combat;
    real runtime_awareness_delta_guard;
    real runtime_awareness_delta_non_combat;
    uint32_t unused3[2];
};
static_assert(sizeof(struct actor_perception_properties) == 104);

struct actor_moving_properties {
    real cover_dive_chance;
    real cover_emerge_chance;
    real grenade_dive_chance;
    real pathfinding_radius;
    real glass_ignorance_chance;
    real stationary_moving_distance;
    real free_flying_sidestep_distance;
    real begin_movement_angle;
    real runtime_begin_movement_angle_cosine;
};
static_assert(sizeof(struct actor_moving_properties) == 36);

struct actor_idle_looking {
    real idle_facing_time_lower_bound;
    real idle_facing_time_upper_bound;
    real idle_aim_time_lower_bound;
    real idle_aim_time_upper_bound;
    real idle_look_time_lower_bound;
    real idle_look_time_upper_bound;
};
static_assert(sizeof(struct actor_idle_looking) == 24);

struct actor_looking_properties {
    real_euler_angles2d maximum_aiming_deviation;
    real_euler_angles2d maximum_looking_deviation;
    real noncombat_looking_delta_angles[2];
    real combat_looking_delta_angles[2];
    real_euler_angles2d idle_aiming_deviation;
    real_euler_angles2d idle_looking_deviation;
    real event_look_modifier_lower_bound;
    real event_look_modifier_upper_bound;
    struct actor_idle_looking idle_look_noncombat;
    struct actor_idle_looking idle_look_guard;
    struct actor_idle_looking idle_look_search_combat;
    real unused[2];
    real_euler_angles2d runtime_maximum_aiming_deviation_cosine;
    real_euler_angles2d runtime_maximum_looking_deviation_cosine;
};
static_assert(sizeof(struct actor_looking_properties) == 152);

struct actor_unopposable_properties {
    int16_t trigger_unreachable_level;
    int16_t trigger_vehicle_level;
    int16_t trigger_player_level;
    uint16_t pad;
    real trigger_time_lower_bound;
    real trigger_time_upper_bound;
    int16_t casualties_trigger;
    int16_t retreating_friends_trigger;
    uint32_t unused_trigger[3];
    real retreat_time_lower_bound;
    real retreat_time_upper_bound;
    uint32_t unused[2];
};
static_assert(sizeof(struct actor_unopposable_properties) == 48);

struct actor_panic_properties {
    real cower_time_lower_bound;
    real cower_time_upper_bound;
    real panic_chance_friend_killed;
    int16_t panic_leader_type;
    uint16_t pad;
    real panic_chance_leader_type_killed;
    real panic_damage_threshold;
    real surprise_distance;
    uint32_t unused[7];
};
static_assert(sizeof(struct actor_panic_properties) == 56);

enum {
    _defensive_crouch_none,
    _defensive_crouch_danger,
    _defensive_crouch_shield_low,
    _defensive_crouch_hide_behind_shield,
    _defensive_crouch_any_target,
    _defensive_crouch_flood_shamble,
    NUMBER_OF_DEFENSIVE_CROUCH_TYPES
};

struct actor_defensive_properties {
    real hide_time_lower_bound;
    real hide_time_upper_bound;
    real hide_target_not_visible_time;
    real shield_fraction_hide;
    real shield_fraction_emerge_attack;
    real shield_fraction_emerge_pursue;
    uint32_t unused[4];
    int16_t defensive_crouch_type;
    uint16_t pad;
    real defensive_threshold_attacking;
    real defensive_threshold_defending;
    real defensive_crouch_min_stand_time;
    real defensive_crouch_min_crouch_time;
    real defending_hide_time_modifier;
    real evasion_danger_threshold_attacking;
    real evasion_danger_threshold_defending;
    real evasion_seek_cover_chance;
    real evasion_delay_timer;
    real cover_max_distance;
    real cover_damage_threshold;
    real stalking_discovery_time;
    real stalking_max_distance;
    real stationary_facing_angle;
    real change_facing_stand_time;
    uint32_t unused3[1];
};
static_assert(sizeof(struct actor_defensive_properties) == 108);

struct actor_pursuit_properties {
    real uncover_delay_time_lower_bound;
    real uncover_delay_time_upper_bound;
    real target_location_time_lower_bound;
    real target_location_time_upper_bound;
    real pursuit_position_time_lower_bound;
    real pursuit_position_time_upper_bound;
    int16_t coordinated_pursuit_positions;
    int16_t uncoordinated_pursuit_positions;
    uint32_t unused[8];
};
static_assert(sizeof(struct actor_pursuit_properties) == 60);

struct actor_berserk_properties {
    real melee_attack_delay_timer;
    real melee_attack_range;
    real melee_attack_charge_timer;
    real melee_leap_range_lower_bound;
    real melee_leap_range_upper_bound;
    real melee_leap_velocity;
    real melee_leap_chance;
    real melee_leap_ballistic;
    real damage_berserk_amount;
    real damage_berserk_threshold;
    real proximity_berserk_distance;
    real suicide_sensing_distance;
    real grenade_attached_chance;
    uint32_t unused_berserk[3];
};
static_assert(sizeof(struct actor_berserk_properties) == 64);

struct actor_firing_position_properties {
    real guard_position_time_lower_bound;
    real guard_position_time_upper_bound;
    real combat_position_time_lower_bound;
    real combat_position_time_upper_bound;
    real old_avoidance_radius;
    real friend_avoidance_radius;
    uint32_t unused_firing_position[10];
};
static_assert(sizeof(struct actor_firing_position_properties) == 64);

struct actor_communication_properties {
    real idle_noncombat_time_lower_bound;
    real idle_noncombat_time_upper_bound;
    real idle_combat_time_lower_bound;
    real idle_combat_time_upper_bound;
    uint32_t unused_communication[12];
};
static_assert(sizeof(struct actor_communication_properties) == 64);

struct actor_definition {
    uint32_t flags;
    uint32_t flags2;
    uint32_t unused3[3];
    int16_t type;
    uint16_t pad;
    struct actor_perception_properties perception;
    struct actor_moving_properties moving;
    struct actor_looking_properties looking;
    struct tag_reference unused_ranged_weapon_reference;
    uint32_t unused_ranged[67];
    struct tag_reference unused_grenade_reference;
    struct actor_unopposable_properties unopposable;
    struct actor_panic_properties panic;
    struct actor_defensive_properties defensive;
    struct actor_pursuit_properties pursuit;
    struct actor_berserk_properties berserk;
    struct actor_firing_position_properties firing_position;
    struct actor_communication_properties communication;
    uint32_t unused[32];
    struct tag_reference unused_major_upgrade_reference;
    uint32_t unused2[12];
};
static_assert(sizeof(struct actor_definition) == 1272);

/* actor variant */

enum {
    _actor_variant_definition_can_shoot_while_flying_bit,
    _actor_variant_definition_color_interpolate_hsv_bit,
    _actor_variant_definition_has_unlimited_grenades_bit,
    _actor_variant_definition_movement_switching_try_to_stay_with_friends_bit,
    _actor_variant_definition_active_camouflage_bit,
    _actor_variant_definition_super_active_camouflage_bit,
    _actor_variant_definition_cannot_use_ranged_weapons_bit,
    _actor_variant_definition_prefer_passenger_seat_bit,
    NUMBER_OF_ACTOR_VARIANT_DEFINITION_FLAGS
};

enum {
    _actor_grenade_trajectory_toss,
    _actor_grenade_trajectory_lob,
    _actor_grenade_trajectory_bounce,
    NUMBER_OF_ACTOR_GRENADE_TRAJECTORIES
};

enum {
    _actor_grenade_stimulus_none,
    _actor_grenade_stimulus_visibletarget,
    _actor_grenade_stimulus_seekcover,
    NUMBER_OF_ACTOR_GRENADE_STIMULI
};

enum {
    _actor_movement_switching_always_run,
    _actor_movement_switching_always_crouch,
    _actor_movement_switching_switch_types,
    NUMBER_OF_MOVEMENT_SWITCHING_TYPES
};

enum {
    _actor_special_fire_mode_none,
    _actor_special_fire_mode_overcharge,
    _actor_special_fire_mode_secondary,
    NUMBER_OF_ACTOR_SPECIAL_FIRE_MODES
};

enum {
    _actor_special_fire_situation_never,
    _actor_special_fire_situation_enemy_visible,
    _actor_special_fire_situation_enemy_out_of_sight,
    _actor_special_fire_situation_strafing,
    NUMBER_OF_ACTOR_SPECIAL_FIRE_SITUATIONS
};

struct actor_burst_geometry {
    real burst_origin_radius;
    real burst_origin_angle;
    real burst_return_length_lower_bound;
    real burst_return_length_upper_bound;
    real burst_return_angle;
    real burst_duration_lower_bound;
    real burst_duration_upper_bound;
    real burst_separation_lower_bound;
    real burst_separation_upper_bound;
    real burst_maximum_angular_velocity;
    uint32_t unused;
};
static_assert(sizeof(struct actor_burst_geometry) == 44);

struct actor_firing_pattern {
    real burst_duration_modifier;
    real burst_separation_modifier;
    real rate_of_fire_modifier;
    real error_angle_modifier;
    uint32_t unused[2];
};
static_assert(sizeof(struct actor_firing_pattern) == 24);

struct actor_ranged_combat_properties {
    struct tag_reference reference;
    real maximum_firing_range;
    real rate_of_fire;
    real projectile_error_angle;
    real first_burst_delay_lower_bound;
    real first_burst_delay_upper_bound;
    real new_target_pattern_time;
    real surprise_delay_time;
    real surprise_wildfire_time;
    real death_wildfire_chance;
    real death_wildfire_time;
    real combat_range_lower_bound;
    real combat_range_upper_bound;
    real_vector3d gun_offset_stand;
    real_vector3d gun_offset_crouch;
    real target_track_fraction;
    real target_lead_fraction;
    real weapon_damage_modifier;
    real damage_per_second;
    struct actor_burst_geometry burst_geometry;
    real special_damage_modifier;
    real special_projectile_error;
    struct actor_firing_pattern new_target_pattern;
    struct actor_firing_pattern moving_pattern;
    struct actor_firing_pattern berserk_pattern;
    real weapon_super_ballistic_range;
    real weapon_bombardment_range;
    real modified_vision_range;
    int16_t special_fire_mode;
    int16_t special_fire_situation;
    real special_fire_chance;
    real special_fire_delay;
    real melee_range;
    real melee_abort_range;
    real berserk_firing_range_lower_bound;
    real berserk_firing_range_upper_bound;
    real berserk_melee_range;
    real berserk_melee_abort_range;
    uint32_t unused3[2];
};
static_assert(sizeof(struct actor_ranged_combat_properties) == 284);

struct actor_grenade_combat_properties {
    int16_t grenade_type;
    int16_t trajectory_type;
    int16_t stimulus_type;
    int16_t minimum_enemy_count;
    real enemy_radius;
    real grenade_unused;
    real grenade_maximum_velocity;
    real grenade_range_lower_bound;
    real grenade_range_upper_bound;
    real collateral_damage_radius;
    real throw_grenade_chance;
    real throw_grenade_delay;
    real encounter_grenade_timeout;
    uint32_t unused2[5];
};
static_assert(sizeof(struct actor_grenade_combat_properties) == 64);

struct actor_item_properties {
    struct tag_reference equipment_reference;
    int16_t grenades_lower_bound;
    int16_t grenades_upper_bound;
    real dont_drop_grenades_chance;
    real weapon_loaded_lower_bound;
    real weapon_loaded_upper_bound;
    int16_t weapon_ammo_lower_bound;
    int16_t weapon_ammo_upper_bound;
    uint32_t weapon_unused[3];
    uint32_t unused[4];
};
static_assert(sizeof(struct actor_item_properties) == 64);

struct actor_unit_properties {
    real maximum_body_vitality;
    real maximum_shield_vitality;
    real shield_sapping_radius;
    int16_t forced_shader_permutation_index;
    uint16_t pad;
    uint32_t unused[4];
};
static_assert(sizeof(struct actor_unit_properties) == 32);

struct actor_variant_change_colors {
    real_rgb_color color_lower_bound;
    real_rgb_color color_upper_bound;
    uint32_t unused[2];
};
static_assert(sizeof(struct actor_variant_change_colors) == 32);

struct actor_movement_switching_properties {
    int16_t movement_type;
    uint16_t pad;
    real initial_crouch_chance;
    real crouch_time_lower_bound;
    real crouch_time_upper_bound;
    real run_time_lower_bound;
    real run_time_upper_bound;
};
static_assert(sizeof(struct actor_movement_switching_properties) == 24);

struct actor_variant_definition {
    uint32_t flags;
    struct tag_reference actor_reference;
    struct tag_reference unit_reference;
    struct tag_reference major_upgrade_reference;
    struct unit_metagame_properties metagame_properties;
    uint32_t unused[5];
    struct actor_movement_switching_properties movement_switching;
    struct actor_ranged_combat_properties ranged_combat;
    struct actor_grenade_combat_properties grenade_combat;
    struct actor_item_properties items;
    struct actor_unit_properties unit;
    uint32_t unused2[3];
    struct tag_block change_colors;
};
static_assert(sizeof(struct actor_variant_definition) == 568);

/* actor definition functions */

static inline struct actor_definition *actor_definition_get(int32_t tag_index) {
    return tag_get(ACTOR_DEFINITION_TAG, tag_index);
}

static inline struct actor_variant_definition *actor_variant_definition_get(int32_t tag_index) {
    return tag_get(ACTOR_VARIANT_DEFINITION_TAG, tag_index);
}

static inline struct actor_variant_change_colors *actor_variant_definition_get_change_color(struct actor_variant_definition *actor_variant, int32_t change_color_index) {
    return tag_block_get_element_with_size(&actor_variant->change_colors, change_color_index, sizeof(struct actor_variant_change_colors));
}

#endif
