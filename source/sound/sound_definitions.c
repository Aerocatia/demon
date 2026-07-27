#include "../cseries/cseries.h"
#include "../cseries/errors.h"

#include "../math/real_math.h"

#include "sound_classes.h"
#include "sound_definitions.h"

/* constants */

constexpr int16_t MAXIMUM_PERMUTATION_TESTS = 16;
constexpr real oo_unsigned_char_max = 1.0f / UINT8_MAX;

/* globals */

const int32_t sound_sample_rate_samples_per_second[NUMBER_OF_SOUND_SAMPLE_RATES] = {
    22050,
    44100
};

/* forward declarations */

static inline void try_to_reset_permutations(struct sound_pitch_range *range);

/* public functions */

real sound_definition_get_maximum_distance(int32_t sound_definition_index) {
    auto definition = sound_definition_get(sound_definition_index);
    if(!definition->maximum_distance) {
        return sound_class_get(definition->class_index)->maximum_distance;
    }

    return definition->maximum_distance;
}

real sound_definition_get_minimum_distance(int32_t sound_definition_index) {
    auto definition = sound_definition_get(sound_definition_index);
    if(!definition->minimum_distance) {
        return sound_class_get(definition->class_index)->minimum_distance;
    }

    return definition->minimum_distance;
}

real sound_permutation_get_real_mouth_aperture(const struct sound_permutation *permutation, int16_t estimated_tick_index) {
    if(!permutation->mouth_data.size) {
        error(_error_silent, "but how can you speak if you have no mouth data? (permutation %s)", permutation->name);
        return 0.0f;
    }

    estimated_tick_index = PIN(estimated_tick_index, 0, permutation->mouth_data.size - 1);
    return *sound_permutation_get_mouth_aperture(permutation, estimated_tick_index) * oo_unsigned_char_max;
}

uint8_t *sound_permutation_get_mouth_aperture(const struct sound_permutation *permutation, int16_t tick_index) {
    assert(tick_index >= 0 && tick_index < permutation->mouth_data.size);

    uint8_t *mouth_data = tag_data_get_address(&permutation->mouth_data);
    return mouth_data + tick_index;
}

int16_t sound_definition_find_pitch_range_by_pitch(struct sound_definition *sound, real pitch, int16_t old_range_index) {
    if(old_range_index != NONE && old_range_index < sound->pitch_ranges.count) {
        auto range = sound_definition_get_pitch_range(sound, old_range_index);
        if(range->bend_lower_bound <= pitch && pitch <= range->bend_upper_bound && range->permutations.count) {
            return old_range_index;
        }
    }

    int16_t best_range_index = NONE;
    real best_range_error = REAL_MAX;
    for(int range_index = 0; range_index < sound->pitch_ranges.count; range_index++) {
        auto range = sound_definition_get_pitch_range(sound, range_index);
        if(!range->permutations.count) {
            continue;
        }

        if(range->bend_lower_bound <= pitch && pitch <= range->bend_upper_bound) {
            best_range_index = range_index;
            break;
        }
        else {
            real error;
            if(range->bend_upper_bound < pitch) {
                error = pitch / range->bend_upper_bound;
            }
            else {
                error = range->bend_lower_bound / pitch;
            }

            if(error < best_range_error) {
                best_range_index = range_index;
                best_range_error = error;
            }
        }
    }

    return best_range_index;
}

int16_t sound_definition_next_permutation(struct sound_definition *sound, int16_t pitch_range_index, int16_t looping_last_permutation_index) {
    auto range = sound_definition_get_pitch_range(sound, pitch_range_index);
    assert(range->permutations.count);

    int16_t permutation_index;
    if(range->runtime_discarded_permutation_index != NONE) {
        permutation_index = range->runtime_discarded_permutation_index;
        range->runtime_discarded_permutation_index = NONE;
        range->runtime_last_permutation_index = permutation_index;
    }
    else if(TEST_FLAG(sound->flags, _sound_definition_linked_permutations_bit) && looping_last_permutation_index != NONE) {
        auto last_permutation = sound_pitch_range_get_permutation(range, looping_last_permutation_index);
        permutation_index = last_permutation->next_permutation_index;
    }
    else {
        permutation_index = local_random_range(0, range->actual_permutation_count);
        int16_t failure_count = 0;
        while(true) {
            try_to_reset_permutations(range);
            if(!TEST_FLAG(range->runtime_permutation_flags, permutation_index)) {
                SET_FLAG(range->runtime_permutation_flags, permutation_index, true);
                if(failure_count++ == MAXIMUM_PERMUTATION_TESTS || real_local_random() >= sound_pitch_range_get_permutation(range, permutation_index)->skip_fraction) {
                    break;
                }
            }

            if(++permutation_index == range->actual_permutation_count) {
                permutation_index = 0;
            }
        }

        range->runtime_last_permutation_index = permutation_index;
    }

    return permutation_index;
}

/* private functions */

static inline void try_to_reset_permutations(struct sound_pitch_range *range) {
    if(!TEST_FLAG_RANGE(~range->runtime_permutation_flags, 0, range->actual_permutation_count - 1)) {
        range->runtime_permutation_flags = 0;
        if(range->actual_permutation_count > 1) {
            SET_FLAG(range->runtime_permutation_flags, range->runtime_last_permutation_index, true);
        }
    }
}
