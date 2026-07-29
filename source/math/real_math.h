#ifndef DEMON_REAL_MATH_H
#define DEMON_REAL_MATH_H

#include <math.h>
#include <float.h>

#include "../cseries/cseries.h"

/* constants */

constexpr real REAL_MAX = FLT_MAX;
constexpr real REAL_MIN = -FLT_MAX;
constexpr real _2pi = 6.28318530718f;
constexpr real _pi = 3.14159265359f;
constexpr real _half_pi = 1.570796326795f;
constexpr real _quarter_pi = 0.7853981633974f;
constexpr real _e = 2.71828182845f;
constexpr real _log2 = 0.69314718056f;
constexpr real _root2 = 1.41421356237f;
constexpr real _root3 = 1.73205080757f;
constexpr real _one_over_root2 = 0.70710678119f;
constexpr real _cosine30 = 0.866025403784f;
constexpr real _sine30 = 0.5f;
constexpr real _cosine45 = _one_over_root2;
constexpr real _sine45 = _one_over_root2;
constexpr real _cosine60 = _sine30;
constexpr real _sine60 = _cosine30;
constexpr real _deg2rad = 0.01745329251994f;
constexpr real _rad2deg = 57.29577951308f;
constexpr real _full_circle = _2pi;
constexpr real _half_circle = _pi;
constexpr real _quarter_circle = _half_pi;
constexpr real _eighth_circle = _quarter_pi;
constexpr real _real_epsilon = 0.0001f;
constexpr real _valid_real_epsilon = 0.001f;

/* math assert macro */

#ifdef MATH_ASSERTS
    #define math_assert(expr) assert(expr)
#else
    #define math_assert(expr) ((void)0)
#endif

/* inline functions (fix this later) */

static inline bool realcmp(real x, real y) {
    return fabs(x - y) < _real_epsilon;
}

real square_root(real x);
real reciprocal_square_root(real x);

real sine(real angle);
real cosine(real angle);
real tangent(real angle);
real arctangent(real y, real x);
real arccosine(real x);
real arcsine(real x);

real_point3d *set_real_point3d(real_point3d *p, real x, real y, real z);
real magnitude_squared2d(const real_vector2d *v);

real_vector3d *scale_vector3d(const real_vector3d *a, real c, real_vector3d *result);
real magnitude_squared3d(const real_vector3d *v);
real magnitude3d(const real_vector3d *v);
real normalize3d(real_vector3d *v);
real dot_product3d(const real_vector3d *a, const real_vector3d *b);
real_vector3d *cross_product3d(const real_vector3d *a, const real_vector3d *b, real_vector3d *result);
real_vector3d *negate_vector3d(const real_vector3d *a, real_vector3d *result);

bool valid_realcmp(real x, real y);
bool valid_real(real n);
bool valid_real_point2d(const real_point2d *p);
bool valid_real_vector2d(const real_vector2d *v);
bool valid_real_point3d(const real_point3d *p);
bool valid_real_vector3d(const real_vector3d *v);
bool valid_real_sine_cosine(real s, real c);
bool valid_real_normal2d(const real_vector2d *n);
bool valid_real_normal3d(const real_vector3d *n);
bool valid_real_plane2d(const real_plane2d *p);
bool valid_real_plane3d(const real_plane3d *p);
bool valid_real_quaternion(const real_quaternion *q);
bool valid_real_vector3d_axes2(const real_vector3d *f, const real_vector3d *u);
bool valid_real_vector3d_axes3(const real_vector3d *f, const real_vector3d *l, const real_vector3d *u);
bool valid_real_matrix4x3(const real_matrix4x3 *m);

/* public functions */

real_vector3d *perpendicular3d(const real_vector3d *a, real_vector3d *result);

void yaw_vectors(real_vector3d *forward, const real_vector3d *up, real sine, real cosine);

static inline void roll_vectors(const real_vector3d *forward, real_vector3d *up , real sine, real cosine) {
    yaw_vectors(up, forward, sine, cosine);
}

real_vector3d *rotate_vector_about_axis(real_vector3d *v, const real_vector3d *axis, real sine, real cosine);

void quaternion_to_angle_and_vector(const real_quaternion *q, real *a, real_vector3d *v);

/* matrix math */

void matrix4x3_identity(real_matrix4x3 *matrix);
void matrix4x3_transpose(real_matrix4x3 *matrix);
void matrix4x3_inverse(const real_matrix4x3 *matrix, real_matrix4x3 *result);
void matrix4x3_scale(real_matrix4x3 *matrix, real scale);
void matrix4x3_translation(real_matrix4x3 *matrix, const real_point3d *point);
void matrix4x3_rotation_from_axis_and_angle(real_matrix4x3 *matrix, const real_vector3d *axis, real sine, real cosine);
void matrix4x3_rotation_from_vectors(real_matrix4x3 *matrix, const real_vector3d *forward, const real_vector3d *up);
void matrix4x3_rotation_from_angles(real_matrix4x3 *matrix, real yaw, real pitch, real roll);
void matrix4x3_rotation_to_angles(real_matrix4x3 *matrix, real_euler_angles3d *angles);
void matrix4x3_rotation_from_quaternion(real_matrix4x3 *matrix, const real_quaternion *quaternion);
void matrix4x3_rotation_to_quaternion(const real_matrix4x3 *matrix, real_quaternion *quaternion);
void matrix4x3_from_point_and_vectors(real_matrix4x3 *matrix, const real_point3d *point, const real_vector3d *forward, const real_vector3d *up);
void matrix4x3_from_point_and_quaternion(real_matrix4x3 *matrix, const real_point3d *point, const real_quaternion *quaternion);
void matrix4x3_from_orientation(real_matrix4x3 *matrix, const real_orientation *orientation);
void matrix4x3_from_plane(real_matrix4x3 *matrix, const real_plane3d *plane);
void matrix4x3_to_point_and_vectors(const real_matrix4x3 *matrix, real_point3d *point, real_vector3d *forward, real_vector3d *up);
real_vector3d *vector_from_matrices4x3(const real_matrix4x3 *a, const real_matrix4x3 *b, real_vector3d *rotation);
real_point3d *matrix4x3_transform_point(const real_matrix4x3 *matrix, const real_point3d *point, real_point3d *result);
real_vector3d *matrix4x3_transform_vector(const real_matrix4x3 *matrix, const real_vector3d *vector, real_vector3d *result);
real_vector3d *matrix4x3_transform_normal(const real_matrix4x3 *matrix, const real_vector3d *normal, real_vector3d *result);
real_plane3d *matrix4x3_transform_plane(const real_matrix4x3 *matrix, const real_plane3d *plane, real_plane3d *result);
real_point3d *matrix4x3_inverse_transform_point(const real_matrix4x3 *matrix, const real_point3d *point, real_point3d *result);
real_vector3d *matrix4x3_inverse_transform_vector(const real_matrix4x3 *matrix, const real_vector3d *vector, real_vector3d *result);
real_vector3d *matrix4x3_inverse_transform_normal(const real_matrix4x3 *matrix, const real_vector3d *normal, real_vector3d *result);
real_plane3d *matrix4x3_inverse_transform_plane(const real_matrix4x3 *matrix, const real_plane3d *plane, real_plane3d *result);
void matrix4x3_multiply(const real_matrix4x3 *a, const real_matrix4x3 *b, real_matrix4x3 *result);
real matrix3x3_determinant(const real_matrix3x3 *matrix);
real_matrix3x3 *matrix3x3_transpose(const real_matrix3x3 *matrix, real_matrix3x3 *result);
real_matrix3x3 *matrix3x3_inverse(const real_matrix3x3 *matrix, real determinant, real_matrix3x3 *result);
real_matrix3x3 *matrix3x3_from_forward_and_up(real_matrix3x3 *matrix, const real_vector3d *forward, const real_vector3d *up);
real_matrix3x3 *matrix3x3_from_axis_and_angle(real_matrix3x3 *matrix, const real_vector3d *axis, real sine, real cosine);
real_matrix3x3 *matrix3x3_multiply(const real_matrix3x3 *a, const real_matrix3x3 *b, real_matrix3x3 *result);
real_vector3d *matrix3x3_transform_vector(const real_matrix3x3 *matrix, const real_vector3d *vector, real_vector3d *result);
real_quaternion *matrix3x3_rotation_to_quaternion(const real_matrix3x3 *matrix, real_quaternion *quaternion);

/* random math */

int32_t get_number_suitable_for_initializing_random_seed();
void lock_global_random_seed();
void unlock_global_random_seed();
uint32_t *get_global_random_seed_address();
uint32_t *get_global_local_random_seed_address();

uint16_t seed_random(uint32_t *seed);
int16_t seed_random_range(uint32_t *seed, int16_t lower_bound, int16_t upper_bound);

real real_seed_random(uint32_t *seed);
real real_seed_random_range(uint32_t *seed, real lower_bound, real upper_bound);

real_vector3d *seed_random_direction3d(uint32_t *seed, real_vector3d *direction);
void seed_random_orientation(uint32_t *seed, real_vector3d *forward, real_vector3d *up);
real_vector3d *seed_random_vector_in_cone3d(uint32_t *seed, const real_vector3d *axis, real inner_cone_angle, real outer_cone_angle, real_vector3d *result);

uint32_t get_random_seed();
void set_random_seed(uint32_t seed);

// global random (deterministic)
uint16_t random();
int16_t random_range(int16_t lower_bound, int16_t upper_bound);
bool random_boolean();
real real_random();
real real_random_range(real lower_bound, real upper_bound);
real_vector3d *random_direction3d(real_vector3d *direction);
void random_orientation(real_vector3d *forward, real_vector3d *up);
real_vector3d *random_vector_in_cone3d(const real_vector3d *axis, real inner_cone_angle, real outer_cone_angle, real_vector3d *result);

// local random (non-deterministic)
uint16_t local_random();
int16_t local_random_range(int16_t lower_bound, int16_t upper_bound);
real real_local_random();
real real_local_random_range(real lower_bound, real upper_bound);
bool local_random_boolean();
real_vector3d *local_random_direction3d(real_vector3d *direction);
void local_random_orientation(real_vector3d *forward, real_vector3d *up);
real_vector3d *local_random_vector_in_cone3d(const real_vector3d *axis, real inner_cone_angle, real outer_cone_angle, real_vector3d *result);

#endif
