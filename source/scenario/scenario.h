#ifndef DEMON_SCENARIO_H
#define DEMON_SCENARIO_H

#include "../cseries/cseries.h"
#include "scenario_definitions.h"

struct scenario *global_scenario_get();
struct scenario *global_scenario_try_and_get();

int16_t global_structure_bsp_index_get();

#endif
