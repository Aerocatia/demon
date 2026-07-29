// These should be static inline and in real_math.h (or something), but are not in the debug exe

#include "../cseries/cseries.h"
#include "real_math.h"

real square_root(real x) {
    return (real)sqrt(x);
}

real reciprocal_square_root(real x) {
    return 1.0f / square_root(x);
}

real sine(real angle) {
    return (real)sin(angle);
}

real cosine(real angle) {
    return (real)cos(angle);
}

real tangent(real angle) {
    return (real)tan(angle);
}

real arctangent(real y, real x) {
    return (real)atan2(y, x);
}

real arccosine(real x) {
    return (real)acos(x);
}

real arcsine(real x) {
    return (real)asin(x);
}

/* vector math */

real_point3d *set_real_point3d(real_point3d *p, real x, real y, real z) {
    p->x = x;
    p->y = y;
    p->z = z;

    return p;
}

real magnitude_squared2d(const real_vector2d *v) {
    return v->i * v->i + v->j * v->j;
}

real_vector3d *scale_vector3d(const real_vector3d *a, real c, real_vector3d *result) {
    result->i = c * a->i;
    result->j = c * a->j;
    result->k = c * a->k;

    return result;
}

real magnitude_squared3d(const real_vector3d *v) {
    return v->i * v->i + v->j * v->j + v->k * v->k;
}

real magnitude3d(const real_vector3d *v) {
    return square_root(magnitude_squared3d(v));
}

real normalize3d(real_vector3d *v) {
    real magnitude = magnitude3d(v);

    if(!realcmp(magnitude, 0.0f)) {
        scale_vector3d(v, 1.0f / magnitude, v);
    }
    else {
        magnitude = 0.0f;
    }

    return magnitude;
}

real dot_product3d(const real_vector3d *a, const real_vector3d *b) {
    return a->i * b->i + a->j * b->j + a->k * b->k;
}

real_vector3d *cross_product3d(const real_vector3d *a, const real_vector3d *b, real_vector3d *result) {
    real k = a->i * b->j - a->j * b->i;
    real j = a->k * b->i - a->i * b->k;

    result->i = a->j * b->k - a->k * b->j;
    result->j = j;
    result->k = k;

    return result;
}

real_vector3d *negate_vector3d(const real_vector3d *a, real_vector3d *result) {
    result->i = -a->i;
    result->j = -a->j;
    result->k = -a->k;

    return result;
}

/* random math */

void set_random_seed(uint32_t seed) {
    *get_global_random_seed_address() = seed;
}

uint16_t random() {
    return seed_random(get_global_random_seed_address());
}

int16_t random_range(int16_t lower_bound, int16_t upper_bound) {
    return seed_random_range(get_global_random_seed_address(), lower_bound, upper_bound);
}

bool random_boolean() {
    return seed_random(get_global_random_seed_address()) > 0x8000;
}

real real_random() {
    return real_seed_random(get_global_random_seed_address());
}

real real_random_range(real lower_bound, real upper_bound) {
    return real_seed_random_range(get_global_random_seed_address(), lower_bound, upper_bound);
}

real_vector3d *random_direction3d(real_vector3d *direction) {
    return seed_random_direction3d(get_global_random_seed_address(), direction);
}

void random_orientation(real_vector3d *forward, real_vector3d *up) {
    seed_random_orientation(get_global_random_seed_address(), forward, up);
}

real_vector3d *random_vector_in_cone3d(const real_vector3d *axis, real inner_cone_angle, real outer_cone_angle, real_vector3d *result) {
    return seed_random_vector_in_cone3d(get_global_random_seed_address(), axis, inner_cone_angle, outer_cone_angle, result);
}

uint16_t local_random() {
    return seed_random(get_global_local_random_seed_address());
}

int16_t local_random_range(int16_t lower_bound, int16_t upper_bound) {
    return seed_random_range(get_global_local_random_seed_address(), lower_bound, upper_bound);
}

real real_local_random() {
    return real_seed_random(get_global_local_random_seed_address());
}

real real_local_random_range(real lower_bound, real upper_bound) {
    return real_seed_random_range(get_global_local_random_seed_address(), lower_bound, upper_bound);
}

bool local_random_boolean() {
    return seed_random(get_global_local_random_seed_address()) > 0x8000;
}

real_vector3d *local_random_direction3d(real_vector3d *direction) {
    return seed_random_direction3d(get_global_local_random_seed_address(), direction);
}

void local_random_orientation(real_vector3d *forward, real_vector3d *up) {
    seed_random_orientation(get_global_local_random_seed_address(), forward, up);
}

real_vector3d *local_random_vector_in_cone3d(const real_vector3d *axis, real inner_cone_angle, real outer_cone_angle, real_vector3d *result) {
    return seed_random_vector_in_cone3d(get_global_local_random_seed_address(), axis, inner_cone_angle, outer_cone_angle, result);
}

/* validity functions */

bool valid_realcmp(real x, real y) {
    real d = x - y;

    return valid_real(d) && fabs(d) < _valid_real_epsilon;
}

bool valid_real(real n) {
    return isfinite(n);
}

bool valid_real_point2d(const real_point2d *p) {
    return valid_real(p->x) && valid_real(p->y);
}

bool valid_real_vector2d(const real_vector2d *v) {
    return valid_real(v->i) && valid_real(v->j);
}

bool valid_real_point3d(const real_point3d *p) {
    return valid_real(p->x) && valid_real(p->y) && valid_real(p->z);
}

bool valid_real_vector3d(const real_vector3d *v) {
    return valid_real(v->i) && valid_real(v->j) && valid_real(v->k);
}

bool valid_real_sine_cosine(real s, real c) {
    return valid_realcmp(s * s + c * c, 1.0f);
}

bool valid_real_normal2d(const real_vector2d *n) {
    return valid_realcmp(magnitude_squared2d(n), 1.0f);
}

bool valid_real_normal3d(const real_vector3d *n) {
    return valid_realcmp(magnitude_squared3d(n), 1.0f);
}

bool valid_real_plane2d(const real_plane2d *p) {
    return valid_real_vector2d(&p->n) && valid_real(p->d);
}

bool valid_real_plane3d(const real_plane3d *p) {
    return valid_real_normal3d(&p->n) && valid_real(p->d);
}

bool valid_real_quaternion(const real_quaternion *q) {
    return valid_realcmp(magnitude_squared3d(&q->v) + q->w * q->w, 1.0f);
}

bool valid_real_vector3d_axes2(const real_vector3d *f, const real_vector3d *u) {
    return valid_real_normal3d(f) && valid_real_normal3d(u) && valid_realcmp(dot_product3d(f, u), 0.0f);
}

bool valid_real_vector3d_axes3(const real_vector3d *f, const real_vector3d *l, const real_vector3d *u) {
    return valid_real_normal3d(f) &&
        valid_real_normal3d(l) &&
        valid_real_normal3d(u) &&
        valid_realcmp(dot_product3d(f, l), 0.0f) &&
        valid_realcmp(dot_product3d(l, u), 0.0f) &&
        valid_realcmp(dot_product3d(u, f), 0.0f);
}

bool valid_real_matrix4x3(const real_matrix4x3 *m) {
    return valid_real(m->scale) && valid_real_vector3d_axes3(&m->forward, &m->left, &m->up) && valid_real_point3d(&m->position);
}
