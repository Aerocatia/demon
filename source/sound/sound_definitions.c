#include "../cseries/cseries.h"
#include "../cseries/errors.h"

#include "sound_classes.h"
#include "sound_definitions.h"

/* constants */

#define MAXIMUM_PERMUTATION_TESTS 16

/* globals */

const int32_t sound_sample_rate_samples_per_second[NUMBER_OF_SOUND_SAMPLE_RATES] = {
    22050,
    44100
};

constexpr real oo_unsigned_char_max = 1.0f / UINT8_MAX;

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
