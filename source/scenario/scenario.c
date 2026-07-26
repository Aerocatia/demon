#include "../cseries/cseries.h"
#include "scenario_definitions.h"

#include "scenario.h"

#include "../demon/exe_globals.h"

/* globals */

#ifndef DEMON_EXE_GLOBALS
static struct scenario *global_scenario;
static int16_t global_structure_bsp_index;
#endif

/* public functions */

struct scenario *global_scenario_get() {
    assert(global_scenario);
    return global_scenario;
}

struct scenario *global_scenario_try_and_get() {
    return global_scenario;
}

int16_t global_structure_bsp_index_get() {
    return global_structure_bsp_index;
}
