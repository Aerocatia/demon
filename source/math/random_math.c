#include "../cseries/cseries.h"

#include "real_math.h"
#include "geometry.h"

#include "../game/game_engine.h"

#include "../cseries/errors.h"

/* constants */

constexpr real OO_UINT16_MAX = 1.0f / UINT16_MAX;
constexpr uint32_t RANDOM_A = 1664525;
constexpr uint32_t RANDOM_C = 1013904223;
constexpr int16_t RANDOM_DIRECTION_TABLE_GEOSPHERE_SEGMENT_COUNT = 16;

/* globals */

struct random_math_global_data {
    real_vector3d *random_direction_table;
    int16_t random_direction_table_size;
};
static_assert(sizeof(struct random_math_global_data) == 8);

static int random_seed_lock_count = 0;
static uint32_t global_random_seed = 0;
static uint32_t global_local_random_seed = 0;
static struct random_math_global_data random_math_globals = {};

/* forward declarations */

static inline uint32_t seed_next(uint32_t *seed);
static inline real_vector3d *direction3d_from_table(real_vector3d *direction, int16_t index);

/* public functions */

void lock_global_random_seed() {
    random_seed_lock_count += 1;
}

void unlock_global_random_seed() {
    vassert(random_seed_lock_count > 0, "unmatched call to unlock_random_seed() somewhere");
    random_seed_lock_count -= 1;
}

uint32_t *get_global_random_seed_address() {
#ifdef DEBUG_BUILD
    if(game_engine_running()) {
        vassert(random_seed_lock_count == 0, "you should not be using global random(); use local random() instead");
    }
#endif

    return &global_random_seed;
}

uint32_t get_random_seed() {
    return global_random_seed;
}

uint32_t *get_global_local_random_seed_address() {
    return &global_local_random_seed;
}

int32_t get_number_suitable_for_initializing_random_seed() {
    return system_milliseconds() ^ system_seconds() ^ rand();
}

void random_math_initialize() {
    *get_global_local_random_seed_address() = get_number_suitable_for_initializing_random_seed();

    auto random_direction_geosphere = geosphere_new(RANDOM_DIRECTION_TABLE_GEOSPHERE_SEGMENT_COUNT);
    assert(random_direction_geosphere);

    size_t vertices_size = random_direction_geosphere->vertex_count * sizeof(real_vector3d);
    random_math_globals.random_direction_table = malloc(vertices_size);
    random_math_globals.random_direction_table_size = random_direction_geosphere->vertex_count;
    memcpy(random_math_globals.random_direction_table, random_direction_geosphere->vertices, vertices_size);

    geosphere_dispose(random_direction_geosphere);
}

void random_math_dispose() {
    assert(random_math_globals.random_direction_table);
    free(random_math_globals.random_direction_table);
}

real real_seed_random(uint32_t *seed) {
    return OO_UINT16_MAX * seed_random(seed);
}

real real_seed_random_range(uint32_t *seed, real lower_bound, real upper_bound) {
    return lower_bound + (upper_bound - lower_bound) * real_seed_random(seed);
}

uint16_t seed_random(uint32_t *seed) {
    return seed_next(seed) >> 16;
}

int16_t seed_random_range(uint32_t *seed, int16_t lower_bound, int16_t upper_bound) {
    return lower_bound + ((upper_bound - lower_bound) * seed_random(seed) >> 16);
}

real_vector3d *seed_random_direction3d(uint32_t *seed, real_vector3d *direction) {
    return direction3d_from_table(direction, seed_random_range(seed, 0, random_math_globals.random_direction_table_size));
}

void seed_random_orientation(uint32_t *seed, real_vector3d *forward, real_vector3d *up) {
    real yaw = _full_circle * real_seed_random(seed);
    real pitch = _half_circle * real_seed_random(seed) - _quarter_circle;
    real roll = _full_circle * real_seed_random(seed);
    real cosine_yaw = cosine(yaw);
    real sine_yaw = sine(yaw);
    real cosine_pitch = cosine(pitch);
    real sine_pitch = sine(pitch);

    forward->i = cosine_pitch * cosine_yaw;
    forward->j = cosine_pitch * sine_yaw;
    forward->k = sine_pitch;

    up->i = -sine_pitch * cosine_yaw;
    up->j = -sine_pitch * sine_yaw;
    up->k = cosine_pitch;

    roll_vectors(forward, up, sine(roll), cosine(roll));
}

real_vector3d *seed_random_vector_in_cone3d(uint32_t *seed, const real_vector3d *axis, real inner_cone_angle, real outer_cone_angle, real_vector3d *result) {
    *result = *axis;

    real_vector3d random_vector;
    seed_random_direction3d(seed, &random_vector);
    real_vector3d random_axis;
    cross_product3d(axis, &random_vector, &random_axis);

    if(normalize3d(&random_axis) > 0.0f) {
        real error_angle = real_seed_random_range(seed, inner_cone_angle, outer_cone_angle);
        rotate_vector_about_axis(result, &random_axis, sine(error_angle), cosine(error_angle));
    }

    return result;
}

/* private functions */

static inline uint32_t seed_next(uint32_t *seed) {
    return *seed = *seed * RANDOM_A + RANDOM_C;
}

static inline real_vector3d *direction3d_from_table(real_vector3d *direction, int16_t index) {
    assert(random_math_globals.random_direction_table);
    assert(index >= 0 && index < random_math_globals.random_direction_table_size);

    *direction = random_math_globals.random_direction_table[index];

    return direction;
}
