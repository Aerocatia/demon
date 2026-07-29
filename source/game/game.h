#ifndef DEMON_GAME_H
#define DEMON_GAME_H

#include "../cseries/cseries.h"

enum {
    _statistics_category_players,
    _statistics_category_ai,
    _statistics_category_vehicles,
    _statistics_category_emplacements,
    NUMBER_OF_STATISTICS_CATEGORIES
};

struct ctf_statistics {
    int16_t flag_grabs;
    int16_t flag_returns;
    int16_t flag_scores;
};
static_assert(sizeof(struct ctf_statistics) == 6);

struct oddball_statistics {
    int16_t time_with_the_ball;
    int16_t ball_carrier_kills;
    int16_t kills_as_ball_carrier;
};
static_assert(sizeof(struct oddball_statistics) == 6);

struct king_statistics {
    int16_t time_on_hill;
};
static_assert(sizeof(struct king_statistics) == 2);

struct race_statistics {
    int16_t last_lap_time;
    int16_t laps;
    int16_t best_lap_time;
};
static_assert(sizeof(struct race_statistics) == 6);

struct slayer_statistics {
    int16_t ignored;
};
static_assert(sizeof(struct slayer_statistics) == 2);

union multiplayer_statistics {
    struct slayer_statistics slayer_statistics;
    struct ctf_statistics ctf_statistics;
    struct oddball_statistics oddball_statistics;
    struct king_statistics king_statistics;
    struct race_statistics race_statistics;
};
static_assert(sizeof(union multiplayer_statistics) == 6);

struct game_statistics {
    int16_t sort_key;
    int16_t games_played;
    int16_t games_won;
    int16_t kills_in_a_row;
    int16_t multiple_kills;
    int16_t last_kill_time;
    int16_t kills[NUMBER_OF_STATISTICS_CATEGORIES];
    int16_t assists[NUMBER_OF_STATISTICS_CATEGORIES];
    int16_t friendly_fire_kills;
    int16_t deaths;
    int16_t suicides;
    int32_t shots_fired;
    int32_t shots_hit;
    int32_t seconds_online;
    int16_t killed_teammate_since_last_death;
    int16_t custom_data_size;
    union multiplayer_statistics multiplayer_statistics;
};
static_assert(sizeof(struct game_statistics) == 60);

#endif
