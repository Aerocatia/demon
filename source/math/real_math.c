#include <math.h>

#include "../cseries/cseries.h"

#include "real_math.h"

real_vector3d *perpendicular3d(const real_vector3d *a, real_vector3d *result) {
    real i = fabs(a->i);
    real j = fabs(a->j);
    real k = fabs(a->k);
    if(i <= j && i <= k) {
        result->i = 0.0f;
        result->j = a->k;
        result->k = -a->j;
    }
    else if(j <= k) {
        result->i = -a->k;
        result->j = 0.0f;
        result->k = a->i;
    }
    else {
        result->i = a->j;
        result->j = -a->i;
        result->k = 0.0f;
    }

    return result;
}

void yaw_vectors(real_vector3d *forward, const real_vector3d *up, real sine, real cosine) {
    math_assert(valid_real_vector3d_axes2(forward, up));
    math_assert(valid_real_sine_cosine(sine, cosine));

    real_vector3d cross;
    cross_product3d(up, forward, &cross);

    forward->i = cosine * forward->i + sine * cross.i;
    forward->j = cosine * forward->j + sine * cross.j;
    forward->k = cosine * forward->k + sine * cross.k;
}

real_vector3d *rotate_vector_about_axis(real_vector3d *v, const real_vector3d *n, real sine, real cosine) {
    math_assert(valid_real_normal3d(n));
    math_assert(valid_real_sine_cosine(sine, cosine));

    real one_minus_cosine_times_v_dot_n = (1.0f - cosine) * (v->i * n->i + v->j * n->j + v->k * n->k);
    real v_cross_n_i = v->j * n->k - v->k * n->j;
    real v_cross_n_j = v->k * n->i - v->i * n->k;
    real v_cross_n_k = v->i * n->j - v->j * n->i;

    v->i = cosine*  v->i + one_minus_cosine_times_v_dot_n * n->i - sine * v_cross_n_i;
    v->j = cosine * v->j + one_minus_cosine_times_v_dot_n * n->j - sine * v_cross_n_j;
    v->k = cosine * v->k + one_minus_cosine_times_v_dot_n * n->k - sine * v_cross_n_k;

    return v;
}

void quaternion_to_angle_and_vector(const real_quaternion *q, real *a, real_vector3d *v) {
    math_assert(valid_real_quaternion(q));

    *v = q->v;
    *a = 2.0f * arctangent(normalize3d(v), q->w);
    if(*a > _half_circle) {
        negate_vector3d(v, v);
        *a = _full_circle - *a;
    }
}
