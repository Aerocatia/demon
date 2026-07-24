#include "../cseries/cseries.h"
#include "scenario_definitions.h"

#include "scenario.h"

/* globals */

asm(".set _global_scenario, 0x00F1A67C");
extern struct scenario *global_scenario;

asm(".set _global_structure_bsp_index, 0x00A39C68");
extern int16_t global_structure_bsp_index;

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
