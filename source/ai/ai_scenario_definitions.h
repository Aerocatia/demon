#ifndef DEMON_AI_SCENARIO_DEFINITIONS_H
#define DEMON_AI_SCENARIO_DEFINITIONS_H

#include "../cseries/cseries.h"
#include "../tag_files/tag_groups.h"
#include "../scenario/scenario_definitions.h"

struct structure_bsp;

struct actor_palette_entry {
    struct tag_reference reference;
};
static_assert(sizeof(struct actor_palette_entry) == 16);

enum {
    _actor_starting_location_required_bit,
    NUMBER_OF_ACTOR_STARTING_LOCATION_FLAGS
};

enum {
    _actor_default_state_none,
    _actor_default_state_asleep,
    _actor_default_state_alert,
    _actor_default_state_moving_repeat_position,
    _actor_default_state_moving_loop,
    _actor_default_state_moving_loop_back_and_forth,
    _actor_default_state_moving_loop_randomly,
    _actor_default_state_moving_randomly,
    _actor_default_state_guarding,
    _actor_default_state_guarding_at_guard_point,
    _actor_default_state_searching,
    _actor_default_state_fleeing,
    NUMBER_OF_ACTOR_DEFAULT_STATES
};

struct actor_starting_location_definition {
    real_point3d position;
    real facing;
    int16_t cluster_index;
    int8_t sequence_id;
    uint8_t flags;
    int16_t default_state;
    int16_t initial_state;
    int16_t actor_palette_index;
    int16_t command_list_index;
};
static_assert(sizeof(struct actor_starting_location_definition) == 28);

struct move_position_definition {
    real_point3d position;
    real facing;
    real weight;
    real time_lower_bound;
    real time_upper_bound;
    int16_t animation_reference_index;
    int8_t sequence_id;
    uint8_t pad2;
    int32_t unused[2];
    int16_t cluster_index;
    uint16_t pad;
    int32_t unused2[8];
    int32_t surface_index;
};
static_assert(sizeof(struct move_position_definition) == 80);

enum {
    _platoon_flee_upon_maneuver_bit,
    _platoon_advancing_maneuver_bit,
    _platoon_initially_defending_bit,
    NUMBER_OF_PLATOON_FLAGS
};

enum {
    _platoon_rule_never,
    _platoon_rule_75_strength,
    _platoon_rule_50_strength,
    _platoon_rule_25_strength,
    _platoon_rule_anybody_dead,
    _platoon_rule_25_dead,
    _platoon_rule_50_dead,
    _platoon_rule_75_dead,
    _platoon_rule_all_but_one_dead,
    _platoon_rule_all_dead,
    NUMBER_OF_PLATOON_RULE_TYPES
};

struct platoon_rule {
    int16_t rule_type;
    int16_t platoon_index;
    int32_t pad;
};

struct platoon_definition {
    char name[TAG_STRING_LENGTH + 1];
    uint32_t flags;
    uint32_t unused1[3];
    struct platoon_rule attacking_defending_rule;
    uint32_t unused2;
    struct platoon_rule maneuvering_rule;
    uint32_t unused3;
    uint32_t unused4[16];
    struct tag_block unused_blocks[3];
};
static_assert(sizeof(struct platoon_definition) == 172);

enum {
    _firing_position_group_attacking,
    _firing_position_group_attacking_search,
    _firing_position_group_attacking_guard,
    _firing_position_group_defending,
    _firing_position_group_defending_search,
    _firing_position_group_defending_guard,
    _firing_position_group_pursuing,
    NUMBER_OF_FIRING_POSITION_GROUPS,
    MAXIMUM_NUMBER_OF_FIRING_POSITION_GROUPS = 8
};

enum {
    _squad_major_upgrade_normal,
    _squad_major_upgrade_few,
    _squad_major_upgrade_many,
    SQUAD_RANDOM_MAJOR_UPGRADE_MAX_TYPE = _squad_major_upgrade_many,
    _squad_major_upgrade_none,
    _squad_major_upgrade_all,
    NUMBER_OF_SQUAD_MAJOR_UPGRADE_TYPES
};

enum {
    _squad_unused_bit,
    _squad_never_search_bit,
    _squad_timer_starts_immediately_bit,
    _squad_delay_forever_bit,
    _squad_magic_sight_after_timer_bit,
    _squad_automatic_migration_bit,
    NUMBER_OF_SQUAD_FLAGS
};

enum {
    _leader_normal,
    _leader_none,
    _leader_random,
    _leader_sergeant_johnson,
    _leader_sergeant_lehto,
    NUMBER_OF_UNIQUE_LEADER_TYPES
};

struct squad_definition {
    char name[TAG_STRING_LENGTH + 1];
    int16_t actor_palette_index;
    int16_t platoon_index;
    int16_t initial_state;
    int16_t default_state;
    uint32_t flags;
    int16_t unique_leader_type;
    uint16_t pad;
    uint32_t unused1[7];
    int16_t pad5;
    int16_t maneuver_squad_index;
    real squad_delay_timer;
    uint32_t firing_position_groups[MAXIMUM_NUMBER_OF_FIRING_POSITION_GROUPS];
    uint32_t pad2[2];
    int16_t min_count;
    int16_t max_count;
    int16_t major_upgrade;
    uint16_t pad3;
    int16_t respawn_min_actors;
    int16_t respawn_max_actors;
    int16_t respawn_total_count;
    uint16_t pad4;
    real respawn_time_lower_bound;
    real respawn_time_upper_bound;
    uint32_t unused3[12];
    struct tag_block move_positions;
    struct tag_block starting_locations;
    struct tag_block unused_block;
};
static_assert(sizeof(struct squad_definition) == 232);

enum {
    CUSTOM_SAPIEN_FIRING_POINT_FIELD = 0x63667066, // 'cfpf'
    NUMBER_OF_FIRING_POSITION_GROUP_INDICES = 26,
    ALL_FIRING_POSITION_GROUPS = ((1 << NUMBER_OF_FIRING_POSITION_GROUP_INDICES) - 1),
};

struct firing_position_definition {
    real_point3d position;
    int16_t group_index;
    int16_t cluster_index;
    int32_t pad;
    int32_t surface_index;
};
static_assert(sizeof(struct firing_position_definition) == 24);

enum {
    _encounter_not_initially_placed_bit,
    _encounter_respawn_enable_bit,
    _encounter_blind_bit,
    _encounter_deaf_bit,
    _encounter_braindead_bit,
    _encounter_3d_firing_positions_bit,
    _encounter_manual_structure_bsp_bit,
    NUMBER_OF_ENCOUNTER_FLAGS
};

enum {
    _encounter_version_0,
    _encounter_version_1,
    _encounter_version_current = _encounter_version_1
};

enum {
    _encounter_search_normal,
    _encounter_search_never,
    _encounter_search_tenacious,
    NUMBER_OF_ENCOUNTER_SEARCHING_BEHAVIORS
};

struct encounter_definition {
    char name[TAG_STRING_LENGTH + 1];
    uint32_t flags;
    int16_t team_index;
    int16_t version;
    int16_t searching;
    int16_t manual_structure_bsp_reference_index;
    real respawn_time_lower_bound;
    real respawn_time_upper_bound;
    uint32_t unused[18];
    uint16_t pad2;
    int16_t runtime_structure_bsp_reference_index;
    struct tag_block squads;
    struct tag_block platoons;
    struct tag_block firing_positions;
    struct tag_block player_starting_locations;
};
static_assert(sizeof(struct encounter_definition) == 176);

enum {
    _ai_atom_pause,
    _ai_atom_go_to,
    _ai_atom_go_to_and_face,
    _ai_atom_move_direction,
    _ai_atom_look,
    _ai_atom_animation_mode,
    _ai_atom_crouch,
    _ai_atom_shoot,
    _ai_atom_grenade,
    _ai_atom_vehicle,
    _ai_atom_running_jump,
    _ai_atom_targeted_jump,
    _ai_atom_script,
    _ai_atom_animate,
    _ai_atom_recording,
    _ai_atom_action,
    _ai_atom_vocalize,
    _ai_atom_targeting,
    _ai_atom_initiative,
    _ai_atom_wait,
    _ai_atom_loop,
    _ai_atom_die,
    _ai_atom_move_immediate,
    _ai_atom_look_random,
    _ai_atom_look_player,
    _ai_atom_look_object,
    _ai_atom_set_radius,
    _ai_atom_teleport,
    NUMBER_OF_AI_ATOM_TYPES
};

enum {
    _ai_atom_go_to_modifier_stop_at_point,
    _ai_atom_go_to_modifier_keep_moving,
    NUMBER_OF_AI_ATOM_GO_TO_MODIFIERS
};

enum {
    _ai_atom_move_direction_modifier_forward,
    _ai_atom_move_direction_modifier_left,
    _ai_atom_move_direction_modifier_right,
    _ai_atom_move_direction_modifier_backward,
    _ai_atom_move_direction_modifier_most_convenient,
    NUMBER_OF_AI_ATOM_MOVE_DIRECTION_MODIFIERS
};

enum {
    _ai_atom_move_immediate_modifier_forward,
    _ai_atom_move_immediate_modifier_left,
    _ai_atom_move_immediate_modifier_right,
    _ai_atom_move_immediate_modifier_backward,
    NUMBER_OF_AI_ATOM_MOVE_IMMEDIATE_MODIFIERS
};

enum {
    _ai_atom_look_modifier_idle_aim,
    _ai_atom_look_modifier_idle_turn_around,
    _ai_atom_look_modifier_idle_look,
    _ai_atom_look_modifier_force_facing,
    _ai_atom_look_modifier_force_aim_weapon,
    NUMBER_OF_AI_ATOM_LOOK_MODIFIERS
};

enum {
    _ai_atom_animation_mode_modifier_noncombat,
    _ai_atom_animation_mode_modifier_asleep,
    _ai_atom_animation_mode_modifier_combat,
    _ai_atom_animation_mode_modifier_panic,
    NUMBER_OF_AI_ATOM_ANIMATION_MODE_MODIFIERS
};

enum {
    _ai_atom_crouch_modifier_disable,
    _ai_atom_crouch_modifier_enable,
    NUMBER_OF_AI_ATOM_CROUCH_MODIFIERS
};

enum {
    _actor_atom_grenade_modifier_toss,
    _actor_atom_grenade_modifier_lob,
    _actor_atom_grenade_modifier_bounce,
    NUMBER_OF_AI_ATOM_GRENADE_MODIFIERS
};

enum {
    _ai_atom_vehicle_modifier_any_non_driver,
    _ai_atom_vehicle_modifier_gunner,
    _ai_atom_vehicle_modifier_passenger,
    _ai_atom_vehicle_modifier_driver,
    _ai_atom_vehicle_modifier_any_seat,
    NUMBER_OF_AI_ATOM_VEHICLE_MODIFIERS
};

enum {
    _ai_atom_animate_modifier_relative_movement,
    _ai_atom_animate_modifier_absolute_movement,
    _ai_atom_animate_modifier_absolute_movement_no_collision,
    _ai_atom_animate_modifier_no_interpolation_relative_movement,
    _ai_atom_animate_modifier_no_interpolation_absolute_movement,
    _ai_atom_animate_modifier_no_interpolation_absolute_movement_no_collision,
    NUMBER_OF_AI_ATOM_ANIMATE_MODIFIERS
};

enum {
    _ai_atom_action_modifier_berserk,
    _ai_atom_action_modifier_surprise_front,
    _ai_atom_action_modifier_surprise_back,
    _ai_atom_action_modifier_evade_left,
    _ai_atom_action_modifier_evade_right,
    _ai_atom_action_modifier_dive_forward,
    _ai_atom_action_modifier_dive_back,
    _ai_atom_action_modifier_dive_left,
    _ai_atom_action_modifier_dive_right,
    _ai_atom_action_modifier_vehicle_woohoo,
    _ai_atom_action_modifier_vehicle_scared,
    NUMBER_OF_AI_ATOM_ACTION_MODIFIERS
};

enum {
    _ai_atom_targeting_modifier_enable,
    _ai_atom_targeting_modifier_disable,
    NUMBER_OF_AI_ATOM_TARGETING_MODIFIERS
};

enum {
    _ai_atom_initiative_modifier_enable,
    _ai_atom_initiative_modifier_disable,
    NUMBER_OF_AI_ATOM_INITIATIVE_MODIFIERS
};

enum {
    _ai_atom_wait_modifier_alerted,
    _ai_atom_wait_modifier_visible_enemy,
    _ai_atom_wait_modifier_told_to_advance,
    NUMBER_OF_AI_ATOM_WAIT_MODIFIERS
};

enum {
    _ai_atom_loop_modifier_always,
    _ai_atom_loop_modifier_until_told_to_advance,
    NUMBER_OF_AI_ATOM_LOOP_MODIFIERS
};

enum {
    _ai_atom_die_modifier_normal,
    _ai_atom_die_modifier_silent,
    NUMBER_OF_AI_ATOM_DIE_MODIFIERS
};

enum {
    _ai_command_list_allow_initiative_bit,
    _ai_command_list_allow_targeting_bit,
    _ai_command_list_disable_looking_bit,
    _ai_command_list_disable_communication_bit,
    _ai_command_list_disable_falling_damage_bit,
    _ai_command_list_manual_structure_bsp_bit,
    NUMBER_OF_AI_COMMAND_LIST_FLAGS
};

struct ai_command_definition {
    int16_t atom_type;
    int16_t atom_modifier;
    real parameter1;
    real parameter2;
    int16_t point1_index;
    int16_t point2_index;
    int16_t animation_reference_index;
    int16_t script_reference_index;
    int16_t recording_reference_index;
    int16_t command_index;
    int16_t object_name_index;
    uint16_t pad;
    uint32_t unused;
};
static_assert(sizeof(struct ai_command_definition) == 32);

struct ai_command_point_definition {
    real_point3d position;
    int32_t surface_index;
    uint32_t unused;
};
static_assert(sizeof(struct ai_command_point_definition) == 20);

struct ai_command_list_definition {
    char name[TAG_STRING_LENGTH + 1];
    uint32_t flags;
    uint32_t unused[2];
    int16_t manual_structure_bsp_reference_index;
    int16_t runtime_structure_bsp_reference_index;
    struct tag_block commands;
    struct tag_block points;
    struct tag_block unused_blocks[2];
};
static_assert(sizeof(struct ai_command_list_definition) == 96);

struct ai_animation_reference_definition {
    char animation_name[TAG_STRING_LENGTH + 1];
    struct tag_reference animation_graph;
    uint32_t unused[3];
};
static_assert(sizeof(struct ai_animation_reference_definition) == 60);

struct ai_script_reference_definition {
    char script_name[TAG_STRING_LENGTH + 1];
    uint32_t unused[2];
};
static_assert(sizeof(struct ai_script_reference_definition) == 40);

struct ai_recording_reference_definition {
    char recording_name[TAG_STRING_LENGTH + 1];
    uint32_t unused[2];
};
static_assert(sizeof(struct ai_recording_reference_definition) == 40);

#define MAXIMUM_DIALOGUE_VARIANTS_PER_CONVERSATION_PARTICIPANT 6

enum {
    _ai_conversation_stop_if_anyone_dies_bit,
    _ai_conversation_stop_if_damaged_bit,
    _ai_conversation_stop_if_visible_enemy_bit,
    _ai_conversation_stop_if_alerted_to_enemy_bit,
    _ai_conversation_player_must_be_visible_bit,
    _ai_conversation_stop_other_actions_bit,
    _ai_conversation_keep_trying_to_play_bit,
    _ai_conversation_player_must_be_looking_at_bit,
    NUMBER_OF_CONVERSATION_DEFINITION_FLAGS
};

enum {
    _ai_conversation_participant_optional_bit,
    _ai_conversation_participant_has_alternate_bit,
    _ai_conversation_participant_is_alternate_bit,
    NUMBER_OF_CONVERSATION_PARTICIPANT_DEFINITION_FLAGS
};

enum {
    _ai_conversation_selection_friendly_actor,
    _ai_conversation_selection_disembodied,
    _ai_conversation_selection_in_player_vehicle,
    _ai_conversation_selection_not_in_vehicle,
    _ai_conversation_selection_sargeant,
    _ai_conversation_selection_any_actor,
    _ai_conversation_selection_radio,
    _ai_conversation_selection_radio_sargeant,
    NUMBER_OF_CONVERSATION_SELECTION_TYPES
};

enum {
    _ai_conversation_line_addressee_look_back_bit,
    _ai_conversation_line_everyone_look_at_speaker_bit,
    _ai_conversation_line_everyone_look_at_addressee_bit,
    _ai_conversation_line_wait_after_until_told_to_advance_bit,
    _ai_conversation_line_wait_until_speaker_nearby_bit,
    _ai_conversation_line_wait_until_everyone_nearby_bit,
    NUMBER_OF_CONVERSATION_LINE_FLAGS
};

enum {
    _ai_conversation_address_none,
    _ai_conversation_address_player,
    _ai_conversation_address_participant,
    NUMBER_OF_CONVERSATION_ADDRESS_TYPES
};

struct ai_conversation_participant {
    uint16_t pad;
    uint16_t flags;
    int16_t selection_type;
    int16_t actor_type;
    int16_t preexisting_object_name_index;
    int16_t new_attach_object_name_index;
    uint32_t unused[3];
    int16_t dialogue_variants[MAXIMUM_DIALOGUE_VARIANTS_PER_CONVERSATION_PARTICIPANT];
    char ai_index_name[TAG_STRING_LENGTH + 1];
    int32_t runtime_ai_index;
    uint32_t unused2[3];
};
static_assert(sizeof(struct ai_conversation_participant) == 84);

struct ai_conversation_line {
    uint16_t flags;
    int16_t participant_index;
    int16_t address_type;
    int16_t address_participant_index;
    int32_t unused;
    real delay_time;
    uint32_t unused2[3];
    struct tag_reference dialogue[MAXIMUM_DIALOGUE_VARIANTS_PER_CONVERSATION_PARTICIPANT];
};
static_assert(sizeof(struct ai_conversation_line) == 124);

struct ai_conversation {
    char name[TAG_STRING_LENGTH + 1];
    uint16_t flags;
    uint16_t pad;
    real trigger_dist;
    real run_to_player_dist;
    uint32_t unused[9];
    struct tag_block participants;
    struct tag_block lines;
    struct tag_block unused_block;
};
static_assert(sizeof(struct ai_conversation) == 116);

/* ai scenario definition functions */

void ai_scenario_attach_structure_bsp(struct scenario *scenario, int16_t structure_bsp_reference_index, struct structure_bsp *structure_bsp, bool editing);
int16_t choose_random_array_element(void *array, int16_t element_size, int16_t element_count, int16_t weight_field_offset, uint32_t *used_bit_vector);
int32_t scenario_get_encounter_by_name(struct scenario *scenario, const char *encounter_name);
int32_t encounter_definition_get_squad_by_name(struct encounter_definition *encounter, const char *squad_name);
int32_t encounter_definition_get_platoon_by_name(struct encounter_definition *encounter, const char *platoon_name);

static inline struct encounter_definition *scenario_get_encounter(struct scenario *scenario, int32_t encounter_index) {
    return tag_block_get_element_with_size(&scenario->ai_encounters, encounter_index, sizeof(struct encounter_definition));
}

static inline struct actor_palette_entry *scenario_get_actor_palette_entry(struct scenario *scenario, int32_t palette_index) {
    return tag_block_get_element_with_size(&scenario->ai_actor_palette, palette_index, sizeof(struct actor_palette_entry));
}

static inline struct scenario_player *encounter_definition_get_player_starting_location(struct encounter_definition *encounter, int32_t player_starting_location_index) {
    return tag_block_get_element_with_size(&encounter->player_starting_locations, player_starting_location_index, sizeof(struct scenario_player));
}

static inline struct squad_definition *encounter_definition_get_squad(struct encounter_definition *encounter, int32_t squad_index) {
    return tag_block_get_element_with_size(&encounter->squads, squad_index, sizeof(struct squad_definition));
}

static inline struct platoon_definition *encounter_definition_get_platoon(struct encounter_definition *encounter, int32_t platoon_index) {
    return tag_block_get_element_with_size(&encounter->platoons, platoon_index, sizeof(struct platoon_definition));
}

static inline struct firing_position_definition *encounter_definition_get_firing_position(struct encounter_definition *encounter, int32_t firing_position_index) {
    return tag_block_get_element_with_size(&encounter->firing_positions, firing_position_index, sizeof(struct firing_position_definition));
}

static inline struct actor_starting_location_definition *squad_definition_get_starting_location(struct squad_definition *squad, int32_t starting_location_index) {
    return tag_block_get_element_with_size(&squad->starting_locations, starting_location_index, sizeof(struct actor_starting_location_definition));
}

static inline struct move_position_definition *squad_definition_get_move_position(struct squad_definition *squad, int32_t move_position_index) {
    return tag_block_get_element_with_size(&squad->move_positions, move_position_index, sizeof(struct move_position_definition));
}

static inline int16_t squad_definition_choose_random_move_position(struct squad_definition *squad, uint32_t *used_bit_vector) {
    return choose_random_array_element(
        squad->move_positions.address,
        sizeof(struct move_position_definition),
        squad->move_positions.count,
        offsetof(struct move_position_definition, weight),
        used_bit_vector);
}

static inline struct ai_command_list_definition *scenario_get_command_list(struct scenario *scenario, int32_t ai_command_list_index) {
    return tag_block_get_element_with_size(&scenario->ai_command_lists, ai_command_list_index, sizeof(struct ai_command_list_definition));
}

static inline struct ai_command_definition *command_list_get_command(struct ai_command_list_definition *command_list, int32_t command_index) {
    return tag_block_get_element_with_size(&command_list->commands, command_index, sizeof(struct ai_command_definition));
}

static inline struct ai_command_point_definition *command_list_get_point(struct ai_command_list_definition *command_list, int32_t point_index) {
    return tag_block_get_element_with_size(&command_list->points, point_index, sizeof(struct ai_command_point_definition));
}

static inline struct ai_animation_reference_definition *scenario_get_animation_reference(struct scenario *scenario, int32_t ai_animation_reference_index) {
    return tag_block_get_element_with_size(&scenario->ai_animation_references, ai_animation_reference_index, sizeof(struct ai_animation_reference_definition));
}

static inline struct ai_script_reference_definition *scenario_get_script_reference(struct scenario *scenario, int32_t ai_script_reference_index) {
    return tag_block_get_element_with_size(&scenario->ai_script_references, ai_script_reference_index, sizeof(struct ai_script_reference_definition));
}

static inline struct ai_recording_reference_definition *scenario_get_recording_reference(struct scenario *scenario, int32_t ai_recording_reference_index) {
    return tag_block_get_element_with_size(&scenario->ai_recording_references, ai_recording_reference_index, sizeof(struct ai_recording_reference_definition));
}

static inline struct ai_conversation *scenario_get_conversation(struct scenario *scenario, int32_t ai_conversation_index) {
    return tag_block_get_element_with_size(&scenario->ai_conversations, ai_conversation_index, sizeof(struct ai_conversation));
}

static inline struct ai_conversation_participant *conversation_get_participant(struct ai_conversation *conversation, int32_t participant_index) {
    return tag_block_get_element_with_size(&conversation->participants, participant_index, sizeof(struct ai_conversation_participant));
}

static inline struct ai_conversation_line *conversation_get_line(struct ai_conversation *conversation, int32_t line_index) {
    return tag_block_get_element_with_size(&conversation->lines, line_index, sizeof(struct ai_conversation_line));
}

#endif
