#ifndef DEMON_OBJECT_DEFINITIONS_H
#define DEMON_OBJECT_DEFINITIONS_H

#include "../cseries/cseries.h"

#include "../tag_files/tag_groups.h"
#include "../math/periodic_functions.h"

enum {
    OBJECT_DEFINITION_TAG = 0x6F626A65, // 'obje'
    OBJECT_DEFINITION_VERSION = 1,
    NUMBER_OF_OUTGOING_OBJECT_FUNCTIONS = 4,
    NUMBER_OF_INCOMING_OBJECT_FUNCTIONS = 4,
    MAXIMUM_NUMBER_OF_ATTACHMENTS_PER_OBJECT = 8,
    MAXIMUM_REGIONS_PER_OBJECT = 8
};

enum {
    _object_function_reference_none,
    _object_function_reference_a,
    _object_function_reference_b,
    _object_function_reference_c,
    _object_function_reference_d,
    NUMBER_OF_OBJECT_FUNCTION_REFERENCES
};

enum {
    _object_function_invert_function_bit,
    _object_function_additive_bit,
    _object_function_does_not_deactivate_below_lower_bound_bit,
    NUMBER_OF_OBJECT_FUNCTION_DEFINITION_FLAGS
};

enum {
    _object_function_clip_to_bounds,
    _object_function_clip_to_bounds_and_normalize,
    _object_function_scale_to_fit_bounds,
    NUMBER_OF_OBJECT_FUNCTION_BOUNDS_MODES
};

enum {
    _object_function_scale_by_none,
    _object_function_scale_by_first_incoming_function,
    _object_function_scale_by_last_incoming_function = _object_function_scale_by_first_incoming_function + NUMBER_OF_INCOMING_OBJECT_FUNCTIONS - 1,
    _object_function_scale_by_first_outgoing_function,
    _object_function_scale_by_last_outgoing_function = _object_function_scale_by_first_outgoing_function + NUMBER_OF_OUTGOING_OBJECT_FUNCTIONS - 1,
    NUMBER_OF_OBJECT_FUNCTION_SCALE_BYS
};

struct object_function_definition {
    uint32_t flags;
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
    int16_t add_function_index;
    int16_t scale_result_by_function_index;
    int16_t bounds_mode;
    real lower_bound;
    real upper_bound;
    real pad2;
    uint16_t pad3;
    int16_t turn_off_with_function_index;
    real scale_by;
    int32_t unused1[63];
    real runtime_reciprocal_bounds_range;
    real runtime_reciprocal_sawtooth_count;
    real runtime_reciprocal_step_count;
    real runtime_one_over_period;
    char usage[TAG_STRING_LENGTH + 1];
};
static_assert(sizeof(struct object_function_definition) == 360);

struct object_attachment_definition {
    struct tag_reference type;
    char marker_name[TAG_STRING_LENGTH + 1];
    int16_t primary_scale_function_reference;
    int16_t secondary_scale_function_reference;
    int16_t change_color_reference;
    uint16_t pad;
    int32_t unused[4];
};
static_assert(sizeof(struct object_attachment_definition) == 72);

enum {
    _object_change_color_a,
    _object_change_color_b,
    _object_change_color_c,
    _object_change_color_d,
    NUMBER_OF_OBJECT_CHANGE_COLORS
};

enum {
    _change_color_reference_none,
    _change_color_reference_a,
    _change_color_reference_b,
    _change_color_reference_c,
    _change_color_reference_d,
    NUMBER_OF_CHANGE_COLOR_REFERENCES
};

struct object_change_color_permutation {
    real weight;
    real_rgb_color color_lower_bound;
    real_rgb_color color_upper_bound;
};
static_assert(sizeof(struct object_change_color_permutation) == 28);

struct object_change_color_definition {
    int16_t darken_by;
    int16_t scaled_by;
    uint32_t scale_flags;
    real_rgb_color color_lower_bound;
    real_rgb_color color_upper_bound;
    struct tag_block permutations;
};
static_assert(sizeof(struct object_change_color_definition) == 44);

struct object_definition_widget {
    struct tag_reference type;
    int32_t unused[4];
};
static_assert(sizeof(struct object_definition_widget) == 32);

enum {
    _object_function_none,
    _object_function_body_vitality,
    _object_function_shield_vitality,
    _object_function_recent_body_damage,
    _object_function_recent_shield_damage,
    _object_function_random_constant,
    _object_function_umbrella_shield_vitality,
    _object_function_shield_stun,
    _object_function_recent_umbrella_shield_vitality,
    _object_function_umbrella_shield_stun,
    _object_function_first_region_damage,
    _object_function_last_region_damage = _object_function_first_region_damage + MAXIMUM_REGIONS_PER_OBJECT - 1,
    _object_function_alive,
    _object_function_compass,
    NUMBER_OF_OBJECT_FUNCTION_MODES
};

enum {
    _object_does_not_cast_shadow_bit,
    _object_transparency_self_occludes_bit,
    _object_artificially_bright_bit,
    _object_not_pathfinding_obstacle_bit,
    NUMBER_OF_OBJECT_DEFINITION_FLAGS
};

enum {
    _object_runtime_scaled_change_colors_bit,
    NUMBER_OF_OBJECT_DEFINITION_RUNTIME_FLAGS
};

struct _object_definition {
    int16_t type;
    uint16_t flags;
    real bounding_radius;
    real_point3d bounding_offset;
    real_point3d origin_offset;
    real acceleration_scale;
    uint32_t runtime_flags;
    struct tag_reference model;
    struct tag_reference animation_graph;
    int32_t unused3[10];
    struct tag_reference collision_model;
    struct tag_reference physics;
    struct tag_reference modifier_shader;
    struct tag_reference creation_effect;
    int32_t unused1[21];
    real render_bounding_radius;
    int16_t function_modes[NUMBER_OF_OUTGOING_OBJECT_FUNCTIONS];
    int32_t unused2[11];
    int16_t icon_text_index;
    int16_t forced_shader_permutation_index;
    struct tag_block attachments;
    struct tag_block widgets;
    struct tag_block functions;
    struct tag_block change_colors;
    struct tag_block predicted_resources;
};

struct object_definition {
    struct _object_definition object;
};
static_assert(sizeof(struct object_definition) == 380);

/* object definition functions */

static inline struct object_definition *object_definition_get(int32_t tag_index) {
    return tag_get(OBJECT_DEFINITION_TAG, tag_index);
}

static inline struct object_function_definition *object_definition_get_function(struct object_definition *object_definition, int32_t function_index) {
    return tag_block_get_element_with_size(&object_definition->object.functions, function_index, sizeof(struct object_function_definition));
}

static inline struct object_change_color_definition *object_definition_get_change_color(struct object_definition *object_definition, int32_t change_color_index) {
    return tag_block_get_element_with_size(&object_definition->object.change_colors, change_color_index, sizeof(struct object_change_color_definition));
}

static inline struct object_change_color_permutation *object_definition_get_change_color_permutation(struct object_change_color_definition *change_color, int32_t permutation_index) {
    return tag_block_get_element_with_size(&change_color->permutations, permutation_index, sizeof(struct object_change_color_permutation));
}

static inline struct object_definition_widget *object_definition_get_widget(struct object_definition *object_definition, int32_t widget_index) {
    return tag_block_get_element_with_size(&object_definition->object.widgets, widget_index, sizeof(struct object_definition_widget));
}

static inline struct object_attachment_definition *object_definition_get_attachment(struct object_definition *object_definition, int32_t attachment_index) {
    return tag_block_get_element_with_size(&object_definition->object.attachments, attachment_index, sizeof(struct object_attachment_definition));
}

#endif
