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
    for(int index = 0; index < element_count; index++) {
        if(!BIT_VECTOR_TEST_FLAG(used_bit_vector, index)) {
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
    for(int index = 0; index < element_count; index++) {
        if(!BIT_VECTOR_TEST_FLAG(used_bit_vector, index)) {
            total_weight += *(real *)weight_cursor;
            if(random_weight <= total_weight) {
                return index;
            }
        }

        weight_cursor += element_size;
    }

    return NONE;
}
