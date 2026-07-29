#ifndef DEMON_GAME_ENGINE_H
#define DEMON_GAME_ENGINE_H

#include "../cseries/cseries.h"

#include "../game/game.h"

struct weapon_datum;
struct scenario_player;
struct message_delta_processor_mode;
struct network_game_client;
struct message_delta_processor_header;

enum get_score_type {
    _get_score_individual,
    _get_score_team,
};

struct game_engine {
    const char *name;
    uint32_t type;
    void (*dispose)();
    bool (*initialize)();
    void (*dispose_from_old_map)();
    void (*player_added)(int32_t player_index);
    void (*player_removed)(int32_t player_index);
    void (*game_ending)();
    void (*game_starting)();
    void (*statistics_append)(struct game_statistics *permanent_statistics, struct game_statistics *game_statistics);
    void (*handle_client_message)(int32_t player_index, void *encoded_message, int16_t encoded_message_size);
    void (*handle_server_message)(void *encoded_message, int16_t encoded_message_size);
    void (*pregame_post_rasterize)();
    void (*post_rasterize)();
    void (*player_update)(int32_t player_index);
    void (*weapon_update)(int32_t weapon_index, struct weapon_datum *weapon);
    bool (*weapon_pickup)(int32_t weapon_index, int32_t player_index);
    void (*weapon_drop)(int32_t weapon_index);
    void (*update)();
    int32_t (*get_score)(int32_t player_index, enum get_score_type get_score_type);
    int32_t (*get_team_score)(int32_t team_index);
    char16_t *(*get_score_string)(int32_t player_index, char16_t *buffer);
    char16_t *(*get_score_header_string)(char16_t *buffer);
    char16_t *(*get_team_score_string)(int16_t team_index, char16_t *buffer);
    bool (*allow_pick_up)(int32_t unit_index, int32_t item_index);
    void (*player_damaged_player)(int32_t killing_player_index, int32_t dead_player_index, bool friendly_fire);
    void (*player_killed_player)(int32_t killing_player_index, int32_t killing_object_index, int32_t dead_player_index, bool friendly_fire);
    bool (*rasterize_score)(int32_t player_index, int32_t message, int32_t message_data, char16_t *buffer, int32_t buffer_size);
    float (*starting_location_rating)(int32_t player_index, struct scenario_player *starting_location);
    void (*prespawn_player_update)(int32_t player_index);
    bool (*postspawn_player_update)(int32_t player_index);
    int32_t (*game_engine_player_get_team_index)(int32_t player_index);
    bool (*goal_matches_player)(int32_t player_index, int32_t goal_index);
    bool (*game_engine_test_flag)(int32_t flag);
    bool (*game_engine_test_trait)(int32_t player_index, int32_t trait);
    int32_t (*game_engine_did_player_win)(int32_t player_index);
    void (*replicate_game_mode_state_to_network)(struct message_delta_processor_mode mode, const int32_t machine_index);
    void (*replicate_game_mode_state_from_network)(struct message_delta_processor_header *header, struct network_game_client *client);
    void (*player_changed_team)(int32_t player_index, bool new_team);
};
static_assert(sizeof(struct game_engine) == 156);

/* public functions */

bool game_engine_running();

#endif
