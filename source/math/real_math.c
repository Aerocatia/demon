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

void quaternion_to_angle_and_vector(const real_quaternion *q, real *a, real_vector3d *v) {
    math_assert(valid_real_quaternion(q));

    *v = q->v;
    *a = 2.0f * arctangent(normalize3d(v), q->w);

    if(*a > _half_circle) {
        negate_vector3d(v, v);
        *a = _full_circle - *a;
    }
}
