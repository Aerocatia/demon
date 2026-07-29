#include "../cseries/cseries.h"
#include "game_engine.h"

// #include "../demon/exe_globals.h"

/* globals */

#ifndef DEMON_EXE_GLOBALS
struct game_engine *game_engine = nullptr;
#endif

/* public functions */

bool game_engine_running() {
    return game_engine != nullptr;
}
