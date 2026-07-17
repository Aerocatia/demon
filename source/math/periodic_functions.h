#ifndef DEMON_PERIODIC_FUNCTIONS_H
#define DEMON_PERIODIC_FUNCTIONS_H

#include "../cseries/cseries.h"

enum {
    _transition_function_linear,
    _transition_function_early,
    _transition_function_very_early,
    _transition_function_late,
    _transition_function_very_late,
    _transition_function_cosine,
    NUMBER_OF_TRANSITION_FUNCTIONS
};

enum {
    _periodic_function_one,
    _periodic_function_zero,
    _periodic_function_cosine,
    _periodic_function_cosine_with_random_period,
    _periodic_function_diagonal_wave,
    _periodic_function_diagonal_wave_with_random_period,
    _periodic_function_slide,
    _periodic_function_slide_with_random_period,
    _periodic_function_noise,
    _periodic_function_jitter,
    _periodic_function_wander,
    _periodic_function_spark,
    NUMBER_OF_PERIODIC_FUNCTIONS
};

#endif
