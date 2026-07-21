#include "../cseries/cseries.h"

#include "ai_scenario_definitions.h"
#include "../math/real_math.h"

const char *global_ai_default_state_names[NUMBER_OF_ACTOR_DEFAULT_STATES] = {
    "none",
    "sleep",
    "alert",
    "move_repeat",
    "move_loop",
    "move_loop_back_and_forth",
    "move_loop_random",
    "move_random",
    "guard",
    "guard_at_position",
    "search",
    "flee"
};

int16_t choose_random_array_element(void *array, int16_t element_size, int16_t element_count, int16_t weight_field_offset, uint32_t *used_bit_vector) {
    real total_weight = 0.0f;
    uint8_t *first_weight = (uint8_t *)array + weight_field_offset;
    uint8_t *weight_cursor = first_weight;
    for(int i = 0; i < element_count; i++) {
        if(!BIT_VECTOR_TEST_FLAG(used_bit_vector, i)) {
            total_weight += *(real *)weight_cursor;
        }

        weight_cursor += element_size;
    }

    if(total_weight <= 0.0f) {
        return NONE;
    }

    real random_weight = real_random_range(0.0f, total_weight);

    total_weight = 0.0f;
    weight_cursor = first_weight;
    for(int i = 0; i < element_count; i++) {
        if(!BIT_VECTOR_TEST_FLAG(used_bit_vector, i)) {
            total_weight += *(real *)weight_cursor;
            if(random_weight <= total_weight) {
                return i;
            }
        }

        weight_cursor += element_size;
    }

    return NONE;
}

int32_t scenario_get_encounter_by_name(struct scenario *scenario, const char *encounter_name) {
    for(int i = 0; i < scenario->ai_encounters.count; i++) {
        auto encounter_definition = scenario_get_encounter(scenario, i);
        if(strncmp_case_insensitive(encounter_definition->name, encounter_name, TAG_STRING_LENGTH + 1) == 0) {
            return i;
        }
    }

    return NONE;
}

int32_t encounter_definition_get_squad_by_name(struct encounter_definition *encounter, const char *squad_name) {
    for(int i = 0; i < encounter->squads.count; i++) {
        auto squad_definition = encounter_definition_get_squad(encounter, i);
        if(strncmp_case_insensitive(squad_definition->name, squad_name, TAG_STRING_LENGTH + 1) == 0) {
            return i;
        }
    }

    return NONE;
}

int32_t encounter_definition_get_platoon_by_name(struct encounter_definition *encounter, const char *platoon_name) {
    for(int i = 0; i < encounter->platoons.count; i++) {
        auto platoon_definition = encounter_definition_get_platoon(encounter, i);
        if(strncmp_case_insensitive(platoon_definition->name, platoon_name, TAG_STRING_LENGTH + 1) == 0) {
            return i;
        }
    }

    return NONE;
}
