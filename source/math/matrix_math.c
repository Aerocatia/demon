#include <string.h>

#include "../cseries/cseries.h"

#include "real_math.h"

void matrix4x3_identity(real_matrix4x3 *matrix) {
    matrix->scale = 1.0f;
    matrix->n[0][0] = 1.0f; matrix->n[0][1] = 0.0f; matrix->n[0][2] = 0.0f;
    matrix->n[1][0] = 0.0f; matrix->n[1][1] = 1.0f; matrix->n[1][2] = 0.0f;
    matrix->n[2][0] = 0.0f; matrix->n[2][1] = 0.0f; matrix->n[2][2] = 1.0f;
    matrix->n[3][0] = 0.0f; matrix->n[3][1] = 0.0f; matrix->n[3][2] = 0.0f;
}

void matrix4x3_transpose(real_matrix4x3 *matrix) {
    math_assert(valid_real_matrix4x3(matrix));

    real swap;
    swap = matrix->n[1][0]; matrix->n[1][0] = matrix->n[0][1]; matrix->n[0][1] = swap;
    swap = matrix->n[2][0]; matrix->n[2][0] = matrix->n[0][2]; matrix->n[0][2] = swap;
    swap = matrix->n[2][1]; matrix->n[2][1] = matrix->n[1][2]; matrix->n[1][2] = swap;
}

void matrix4x3_inverse(const real_matrix4x3 *matrix, real_matrix4x3 *result) {
    math_assert(valid_real_matrix4x3(matrix));

    if(matrix->scale == 0.0f) {
        memset(result, 0, sizeof(real_matrix4x3));
        return;
    }

    real x = -matrix->n[3][0];
    real y = -matrix->n[3][1];
    real z = -matrix->n[3][2];
    if(matrix->scale != 1.0f) {
        result->scale = 1.0f / matrix->scale;
        x *= result->scale;
        y *= result->scale;
        z *= result->scale;
    }
    else {
        result->scale= 1.0f;
    }

    result->n[0][0] = matrix->n[0][0];
    result->n[1][1] = matrix->n[1][1];
    result->n[2][2] = matrix->n[2][2];

    real swap;
    swap = matrix->n[1][0]; result->n[1][0] = matrix->n[0][1]; result->n[0][1] = swap;
    swap = matrix->n[2][0]; result->n[2][0] = matrix->n[0][2]; result->n[0][2] = swap;
    swap = matrix->n[2][1]; result->n[2][1] = matrix->n[1][2]; result->n[1][2] = swap;

    result->n[3][0] = x * result->n[0][0] + y * result->n[1][0] + z * result->n[2][0];
    result->n[3][1] = x * result->n[0][1] + y * result->n[1][1] + z * result->n[2][1];
    result->n[3][2] = x * result->n[0][2] + y * result->n[1][2] + z * result->n[2][2];
}

void matrix4x3_scale(real_matrix4x3 *matrix, real scale) {
    matrix->scale = scale;
    matrix->n[0][0] = 1.0f; matrix->n[0][1] = 0.0f; matrix->n[0][2] = 0.0f;
    matrix->n[1][0] = 0.0f; matrix->n[1][1] = 1.0f; matrix->n[1][2] = 0.0f;
    matrix->n[2][0] = 0.0f; matrix->n[2][1] = 0.0f; matrix->n[2][2] = 1.0f;
    matrix->n[3][0] = 0.0f; matrix->n[3][1] = 0.0f; matrix->n[3][2] = 0.0f;
}

void matrix4x3_translation(real_matrix4x3 *matrix, const real_point3d *point) {
    matrix->scale = 1.0f;
    matrix->n[0][0] = 1.0f; matrix->n[0][1] = 0.0f; matrix->n[0][2] = 0.0f;
    matrix->n[1][0] = 0.0f; matrix->n[1][1] = 1.0f; matrix->n[1][2] = 0.0f;
    matrix->n[2][0] = 0.0f; matrix->n[2][1] = 0.0f; matrix->n[2][2] = 1.0f;
    matrix->position = *point;
}

void matrix4x3_rotation_from_axis_and_angle(real_matrix4x3 *matrix, const real_vector3d *axis, real sine, real cosine) {
    math_assert(valid_real_normal3d(axis));
    math_assert(valid_real_sine_cosine(sine, cosine));

    real_vector3d axis_squared = {.i = axis->i * axis->i, .j = axis->j * axis->j, .k = axis->k * axis->k};
    real_vector3d axis_sine = {.i = sine * axis->i, .j = sine * axis->j, .k = sine * axis->k};

    matrix->scale = 1.0f;
    matrix->n[0][0] = axis_squared.i + cosine * (1.0f - axis_squared.i);
    matrix->n[1][0] = matrix->n[0][1] = axis->i * axis->j * (1.0f - cosine); matrix->n[1][0] -= axis_sine.k, matrix->n[0][1] += axis_sine.k;
    matrix->n[1][1] = axis_squared.j + cosine * (1.0f - axis_squared.j);
    matrix->n[2][0] = matrix->n[0][2] = axis->k * axis->i * (1.0f - cosine); matrix->n[2][0] += axis_sine.j, matrix->n[0][2] -= axis_sine.j;
    matrix->n[2][2] = axis_squared.k + cosine * (1.0f - axis_squared.k);
    matrix->n[2][1] = matrix->n[1][2] = axis->j * axis->k * (1.0f - cosine); matrix->n[2][1] -= axis_sine.i, matrix->n[1][2] += axis_sine.i;
    matrix->n[3][0] = matrix->n[3][1] = matrix->n[3][2] = 0.0f;
}

void matrix4x3_rotation_from_vectors(real_matrix4x3 *matrix, const real_vector3d *forward, const real_vector3d *up) {
    math_assert(valid_real_vector3d_axes2(forward, up));

    matrix->scale = 1.0f;
    matrix->forward = *forward;
    cross_product3d(up, forward, &matrix->left);
    matrix->up = *up;
    set_real_point3d(&matrix->position, 0.0f, 0.0f, 0.0f);
}

void matrix4x3_rotation_from_angles(real_matrix4x3 *matrix, real yaw, real pitch, real roll) {
    real a = cosine(roll);
    real b = sine(roll);
    real c = cosine(pitch);
    real d = sine(pitch);
    real e = cosine(yaw);
    real f = sine(yaw);

#ifdef NEW_AND_IMPROVED_EULER_ANGLES
    real ae = a * e;
    real af = a * f;
    real be = b * e;
    real bf = b * f;

    matrix->scale = 1.0f;
    matrix->n[0][0] = c * e;        matrix->n[0][1] = c * f;        matrix->n[0][2] = d;
    matrix->n[1][0] = -be * d - af; matrix->n[1][1] = -bf * d + ae; matrix->n[1][2] = b * c;
    matrix->n[2][0] = -ae * d + bf; matrix->n[2][1] = -af * d - be; matrix->n[2][2] = a * c;
    matrix->n[3][0] = 0.0f;         matrix->n[3][1] = 0.0f;         matrix->n[3][2] = 0.0f;
#else
    real ad = a * d;
    real bd = b * d;

    matrix->scale = 1.0f;
    matrix->n[0][0] = c * e;  matrix->n[0][1] = -bd * e + a * f; matrix->n[0][2] = ad * e + b * f;
    matrix->n[1][0] = -c * f; matrix->n[1][1] = bd * f + a * e;  matrix->n[1][2] = -ad * f + b * e;
    matrix->n[2][0] = -d;     matrix->n[2][1] = -b * c;          matrix->n[2][2] = a * c;
    matrix->n[3][0] = 0.0f;   matrix->n[3][1] = 0.0f;            matrix->n[3][2] = 0.0f;
#endif
}

void matrix4x3_rotation_to_angles(real_matrix4x3 *matrix, real_euler_angles3d *angles) {
    math_assert(valid_real_matrix4x3(matrix));

#ifdef NEW_AND_IMPROVED_EULER_ANGLES
    angles->pitch= arcsine(matrix->n[0][2]);

    if(!realcmp(fabs(matrix->n[0][2]), 1.0f)) {
        angles->yaw = arctangent(matrix->n[0][1], matrix->n[0][0]);
        angles->roll = arctangent(matrix->n[1][2], matrix->n[2][2]);
    }
    else {
        angles->yaw = arctangent(-matrix->n[1][0], matrix->n[1][1]);
        angles->roll = 0.0f;
    }
#else
    angles->pitch = -arcsine( matrix->n[2][0]);

    real trx, try;
    real C = cosine(angles->pitch);

    if(C > 0.0001) {
        trx =  matrix->n[2][2] / C;
        try = -matrix->n[2][1] / C;

        angles->roll = arctangent(try, trx);

        trx =  matrix->n[0][0] / C;
        try = -matrix->n[1][0] / C;

        angles->yaw  = arctangent(try, trx);
    }
    else {
        angles->roll  = 0;

        trx = matrix->n[1][1];
        try = matrix->n[0][1];

        angles->yaw  = arctangent(try, trx);

    }
#endif
}

void matrix4x3_rotation_from_quaternion(real_matrix4x3 *matrix, const real_quaternion *quaternion) {
    real n = quaternion->v.i * quaternion->v.i + quaternion->v.j * quaternion->v.j + quaternion->v.k * quaternion->v.k + quaternion->w * quaternion->w;
    real scale = n != 0.0f ? 2.0f / n : 0.0f;
    real xs = quaternion->v.i * scale;
    real ys = quaternion->v.j * scale;
    real zs = quaternion->v.k * scale;
    real wx = quaternion->w * xs;
    real wy = quaternion->w * ys;
    real wz = quaternion->w * zs;
    real xx = quaternion->v.i * xs;
    real xy = quaternion->v.i * ys;
    real xz = quaternion->v.i * zs;
    real yy = quaternion->v.j * ys;
    real yz = quaternion->v.j * zs;
    real zz = quaternion->v.k * zs;

    math_assert(valid_real_quaternion(quaternion));

    matrix->scale = 1.0f;
    matrix->n[0][0] = 1.0f - (yy + zz); matrix->n[0][1] = xy - wz;          matrix->n[0][2] = xz + wy;
    matrix->n[1][0] = xy + wz;          matrix->n[1][1] = 1.0f - (xx + zz); matrix->n[1][2] = yz - wx;
    matrix->n[2][0] = xz - wy;          matrix->n[2][1] = yz + wx;          matrix->n[2][2] = 1.0f - (xx + yy);
    matrix->n[3][0] = 0.0f;             matrix->n[3][1] = 0.0f;             matrix->n[3][2] = 0.0f;
}

void matrix4x3_rotation_to_quaternion(const real_matrix4x3 *matrix, real_quaternion *quaternion) {
    math_assert(valid_real_matrix4x3(matrix));

    real tr = matrix->n[0][0] + matrix->n[1][1] + matrix->n[2][2];
    real s;
    if(tr > 0.0f) {
        s = square_root(tr + 1.0f);
        quaternion->w = s / 2.0f;
        s = 0.5f / s;
        quaternion->v.i = (matrix->n[2][1] - matrix->n[1][2]) * s;
        quaternion->v.j = (matrix->n[0][2] - matrix->n[2][0]) * s;
        quaternion->v.k = (matrix->n[1][0] - matrix->n[0][1]) * s;
    }
    else {
        int16_t i = 0;
        if(matrix->n[1][1] > matrix->n[0][0]) {
            i = 1;
        }
        if(matrix->n[2][2] > matrix->n[i][i]) {
            i = 2;
        }

        static int16_t next_field[3] = {1, 2, 0};
        int16_t j = next_field[i];
        int16_t k = next_field[j];
        s = square_root((matrix->n[i][i] - (matrix->n[j][j] + matrix->n[k][k])) + 1.0f);

        real q[3];
        q[i] = s * 0.5f;

        if(s != 0.0f) {
            s = 0.5f / s;
        }

        q[j] = (matrix->n[j][i] + matrix->n[i][j]) * s;
        q[k] = (matrix->n[k][i] + matrix->n[i][k]) * s;

        quaternion->w = (matrix->n[k][j] - matrix->n[j][k]) * s;
        quaternion->v.i = q[0];
        quaternion->v.j = q[1];
        quaternion->v.k = q[2];
    }
}

void matrix4x3_from_point_and_vectors(real_matrix4x3 *matrix, const real_point3d *point, const real_vector3d *forward, const real_vector3d *up) {
    matrix4x3_rotation_from_vectors(matrix, forward, up);
    matrix->position = *point;
}

void matrix4x3_from_point_and_quaternion(real_matrix4x3 *matrix, const real_point3d *point, const real_quaternion *quaternion) {
    matrix4x3_rotation_from_quaternion(matrix, quaternion);
    matrix->position = *point;
}

void matrix4x3_from_orientation(real_matrix4x3 *matrix, const real_orientation *orientation) {
    matrix4x3_rotation_from_quaternion(matrix, &orientation->rotation);
    matrix->scale = orientation->scale;
    matrix->position = orientation->translation;
}

void matrix4x3_from_plane(real_matrix4x3 *matrix, const real_plane3d *plane) {
    assert(valid_real_plane3d(plane));

    real_vector3d forward;
    perpendicular3d(&plane->n, &forward);
    normalize3d(&forward);

    real_point3d origin;
    origin.x = plane->n.i * plane->d;
    origin.y = plane->n.j * plane->d;
    origin.z = plane->n.k * plane->d;

    matrix4x3_from_point_and_vectors(matrix, &origin, &forward, &plane->n);
}

void matrix4x3_to_point_and_vectors(const real_matrix4x3 *matrix, real_point3d *point, real_vector3d *forward, real_vector3d *up) {
    math_assert(valid_real_matrix4x3(matrix));

    *forward = matrix->forward;
    *up = matrix->up;
    *point = matrix->position;
}

real_vector3d *vector_from_matrices4x3(const real_matrix4x3 *a, const real_matrix4x3 *b, real_vector3d *rotation) {
    real_matrix4x3 b_inverse, displacement_rotation_matrix;
    real_quaternion displacement_rotation_quaternion;
    real angle;
    matrix4x3_inverse(b, &b_inverse);
    matrix4x3_multiply(a, &b_inverse, &displacement_rotation_matrix);
    matrix4x3_rotation_to_quaternion(&displacement_rotation_matrix, &displacement_rotation_quaternion);
    quaternion_to_angle_and_vector(&displacement_rotation_quaternion, &angle, rotation);
    scale_vector3d(rotation, angle, rotation);

    return rotation;
}

real_point3d *matrix4x3_transform_point(const real_matrix4x3 *matrix, const real_point3d *point, real_point3d *result) {
    math_assert(valid_real_matrix4x3(matrix));

    real x = point->x;
    real y = point->y;
    real z = point->z;
    if(matrix->scale != 1.0f) {
        x *= matrix->scale;
        y *= matrix->scale;
        z *= matrix->scale;
    }

    result->x = x * matrix->n[0][0] + y * matrix->n[1][0] + z * matrix->n[2][0] + matrix->n[3][0];
    result->y = x * matrix->n[0][1] + y * matrix->n[1][1] + z * matrix->n[2][1] + matrix->n[3][1];
    result->z = x * matrix->n[0][2] + y * matrix->n[1][2] + z * matrix->n[2][2] + matrix->n[3][2];

    return result;
}

real_vector3d *matrix4x3_transform_vector(const real_matrix4x3 *matrix, const real_vector3d *vector, real_vector3d *result) {
    math_assert(valid_real_matrix4x3(matrix));

    real i = vector->i;
    real j = vector->j;
    real k = vector->k;
    if(matrix->scale != 1.0f) {
        i *= matrix->scale;
        j *= matrix->scale;
        k *= matrix->scale;
    }

    result->i = i * matrix->n[0][0] + j * matrix->n[1][0] + k * matrix->n[2][0];
    result->j = i * matrix->n[0][1] + j * matrix->n[1][1] + k * matrix->n[2][1];
    result->k = i * matrix->n[0][2] + j * matrix->n[1][2] + k * matrix->n[2][2];

    return result;
}

real_vector3d *matrix4x3_transform_normal(const real_matrix4x3 *matrix, const real_vector3d *normal, real_vector3d *result) {
    math_assert(valid_real_matrix4x3(matrix));
    math_assert(valid_real_normal3d(normal));

    real i = normal->i;
    real j = normal->j;
    real k = normal->k;
    result->i = i * matrix->n[0][0] + j * matrix->n[1][0] + k * matrix->n[2][0];
    result->j = i * matrix->n[0][1] + j * matrix->n[1][1] + k * matrix->n[2][1];
    result->k = i * matrix->n[0][2] + j * matrix->n[1][2] + k * matrix->n[2][2];

    return result;
}

real_plane3d *matrix4x3_transform_plane(const real_matrix4x3 *matrix, const real_plane3d *plane, real_plane3d *result) {
    math_assert(valid_real_plane3d(plane));

    matrix4x3_transform_normal(matrix, &plane->n, &result->n);
    result->d = matrix->scale*plane->d + dot_product3d((const real_vector3d *)&matrix->position, &result->n);

    return result;
}

real_point3d *matrix4x3_inverse_transform_point(const real_matrix4x3 *matrix, const real_point3d *point, real_point3d *result) {
    math_assert(valid_real_matrix4x3(matrix));

    if(matrix->scale != 0.0f) {
        real x = point->x - matrix->n[3][0];
        real y = point->y - matrix->n[3][1];
        real z = point->z - matrix->n[3][2];
        if(matrix->scale != 1.0f) {
            real one_over_scale = 1.0f / matrix->scale;
            x *= one_over_scale;
            y *= one_over_scale;
            z *= one_over_scale;
        }

        result->x = x * matrix->n[0][0] + y * matrix->n[0][1] + z * matrix->n[0][2];
        result->y = x * matrix->n[1][0] + y * matrix->n[1][1] + z * matrix->n[1][2];
        result->z = x * matrix->n[2][0] + y * matrix->n[2][1] + z * matrix->n[2][2];
    }
    else {
        result->x = 0.0f;
        result->y = 0.0f;
        result->z = 0.0f;
    }

    return result;
}

real_vector3d *matrix4x3_inverse_transform_vector(const real_matrix4x3 *matrix, const real_vector3d *vector, real_vector3d *result) {
    math_assert(valid_real_matrix4x3(matrix));

    real i = vector->i;
    real j = vector->j;
    real k = vector->k;
    if(matrix->scale != 1.0f) {
        real one_over_scale = 1.0f / matrix->scale;
        i *= one_over_scale;
        j *= one_over_scale;
        k *= one_over_scale;
    }

    result->i= i * matrix->n[0][0] + j * matrix->n[0][1] + k * matrix->n[0][2];
    result->j= i * matrix->n[1][0] + j * matrix->n[1][1] + k * matrix->n[1][2];
    result->k= i * matrix->n[2][0] + j * matrix->n[2][1] + k * matrix->n[2][2];

    return result;
}

real_vector3d *matrix4x3_inverse_transform_normal(const real_matrix4x3 *matrix, const real_vector3d *normal, real_vector3d *result) {
    math_assert(valid_real_matrix4x3(matrix));

    real i = normal->i;
    real j = normal->j;
    real k = normal->k;
    result->i = i * matrix->n[0][0] + j * matrix->n[0][1] + k * matrix->n[0][2];
    result->j = i * matrix->n[1][0] + j * matrix->n[1][1] + k * matrix->n[1][2];
    result->k = i * matrix->n[2][0] + j * matrix->n[2][1] + k * matrix->n[2][2];

    return result;
}

real_plane3d *matrix4x3_inverse_transform_plane(const real_matrix4x3 *matrix, const real_plane3d *plane, real_plane3d *result) {
    math_assert(valid_real_plane3d(plane));

    if(matrix->scale != 0.0f) {
        result->d = plane->d - dot_product3d((const real_vector3d *)&matrix->position, &plane->n);
        if(matrix->scale != 1.0f) {
            result->d /= matrix->scale;
        }
    }
    else {
        result->d = 0.0f;
    }

    matrix4x3_inverse_transform_vector(matrix, &plane->n, &result->n);

    return result;
}

void matrix4x3_multiply(const real_matrix4x3 *a, const real_matrix4x3 *b, real_matrix4x3 *result) {
    real_matrix4x3 ac = *a;
    real_matrix4x3 bc = *b;

    result->n[0][0] = ac.n[0][0] * bc.n[0][0] + ac.n[1][0] * bc.n[0][1] + ac.n[2][0] * bc.n[0][2];
    result->n[0][1] = ac.n[0][1] * bc.n[0][0] + ac.n[1][1] * bc.n[0][1] + ac.n[2][1] * bc.n[0][2];
    result->n[0][2] = ac.n[0][2] * bc.n[0][0] + ac.n[1][2] * bc.n[0][1] + ac.n[2][2] * bc.n[0][2];

    result->n[1][0] = ac.n[0][0] * bc.n[1][0] + ac.n[1][0] * bc.n[1][1] + ac.n[2][0] * bc.n[1][2];
    result->n[1][1] = ac.n[0][1] * bc.n[1][0] + ac.n[1][1] * bc.n[1][1] + ac.n[2][1] * bc.n[1][2];
    result->n[1][2] = ac.n[0][2] * bc.n[1][0] + ac.n[1][2] * bc.n[1][1] + ac.n[2][2] * bc.n[1][2];

    result->n[2][0] = ac.n[0][0] * bc.n[2][0] + ac.n[1][0] * bc.n[2][1] + ac.n[2][0] * bc.n[2][2];
    result->n[2][1] = ac.n[0][1] * bc.n[2][0] + ac.n[1][1] * bc.n[2][1] + ac.n[2][1] * bc.n[2][2];
    result->n[2][2] = ac.n[0][2] * bc.n[2][0] + ac.n[1][2] * bc.n[2][1] + ac.n[2][2] * bc.n[2][2];

    result->n[3][0] = ac.n[3][0] + ac.scale * (ac.n[0][0] * bc.n[3][0] + ac.n[1][0] * bc.n[3][1] + ac.n[2][0] * bc.n[3][2]);
    result->n[3][1] = ac.n[3][1] + ac.scale * (ac.n[0][1] * bc.n[3][0] + ac.n[1][1] * bc.n[3][1] + ac.n[2][1] * bc.n[3][2]);
    result->n[3][2] = ac.n[3][2] + ac.scale * (ac.n[0][2] * bc.n[3][0] + ac.n[1][2] * bc.n[3][1] + ac.n[2][2] * bc.n[3][2]);

    result->scale = ac.scale * bc.scale;
}

real matrix3x3_determinant(const real_matrix3x3 *matrix) {
    return
        matrix->n[0][0] * matrix->n[1][1] * matrix->n[2][2] +
        matrix->n[0][1] * matrix->n[1][2] * matrix->n[2][0] +
        matrix->n[0][2] * matrix->n[1][0] * matrix->n[2][1] -
        matrix->n[0][0] * matrix->n[1][2] * matrix->n[2][1] -
        matrix->n[0][1] * matrix->n[1][0] * matrix->n[2][2] -
        matrix->n[0][2] * matrix->n[1][1] * matrix->n[2][0];
}

real_matrix3x3 *matrix3x3_transpose(const real_matrix3x3 *matrix, real_matrix3x3 *result) {
    if(matrix == result) {
        real swap;
        swap = matrix->n[0][1]; result->n[0][1] = matrix->n[1][0]; result->n[1][0] = swap;
        swap = matrix->n[0][2]; result->n[0][2] = matrix->n[2][0]; result->n[2][0] = swap;
        swap = matrix->n[1][2]; result->n[1][2] = matrix->n[2][1]; result->n[2][1] = swap;
    }
    else {
        result->n[0][0] = matrix->n[0][0]; result->n[0][1] = matrix->n[1][0]; result->n[0][2] = matrix->n[2][0];
        result->n[1][0] = matrix->n[0][1]; result->n[1][1] = matrix->n[1][1]; result->n[1][2] = matrix->n[2][1];
        result->n[2][0] = matrix->n[0][2]; result->n[2][1] = matrix->n[1][2]; result->n[2][2] = matrix->n[2][2];
    }

    return result;
}

real_matrix3x3 *matrix3x3_inverse(const real_matrix3x3 *matrix, real determinant, real_matrix3x3 *result) {
    assert(!realcmp(determinant, 0.0f));

    real_matrix3x3 mc = *matrix;
    real oodeterminant = 1.0f / determinant;
    for(int16_t i = 0; i < 3; i++) {
        for(int16_t j = 0; j < 3; j++) {
            int16_t i1 = i < 2 ? i + 1 : 0;
            int16_t i2 = i > 0 ? i - 1 : 2;
            int16_t j1 = j < 2 ? j + 1 : 0;
            int16_t j2 = j > 0 ? j - 1 : 2;

            result->n[j][i] = oodeterminant * (mc.n[i1][j1] * mc.n[i2][j2] - mc.n[i1][j2] * mc.n[i2][j1]);
        }
    }

#ifdef DEBUG_MATRIX3X3_INVERSE
    real_matrix3x3 I;
    matrix3x3_multiply(&mc, result, &I);
    assert(
        realcmp(I.n[0][0], 1.0f) && realcmp(I.n[0][1], 0.0f) && realcmp(I.n[0][2], 0.0f) &&
        realcmp(I.n[1][0], 0.0f) && realcmp(I.n[1][1], 1.0f) && realcmp(I.n[1][2], 0.0f) &&
        realcmp(I.n[2][0], 0.0f) && realcmp(I.n[2][1], 0.0f) && realcmp(I.n[2][2], 1.0f)
    );
#endif

    return result;
}

real_matrix3x3 *matrix3x3_from_forward_and_up(real_matrix3x3 *matrix, const real_vector3d *forward, const real_vector3d *up) {
    matrix->forward = *forward;
    cross_product3d(up, forward, &matrix->left);
    matrix->up = *up;

    return matrix;
}

real_matrix3x3 *matrix3x3_from_axis_and_angle(real_matrix3x3 *matrix, const real_vector3d *axis, real sine, real cosine) {
    math_assert(valid_real_normal3d(axis));
    math_assert(valid_real_sine_cosine(sine, cosine));

    real_vector3d axis_squared = {.i = axis->i * axis->i, .j = axis->j * axis->j, .k = axis->k * axis->k};
    real_vector3d axis_sine = {.i = sine * axis->i, .j = sine * axis->j, .k = sine * axis->k};

    matrix->n[0][0] = axis_squared.i + cosine * (1.0f - axis_squared.i);
    matrix->n[1][0] = matrix->n[0][1] = axis->i * axis->j * (1.0f - cosine); matrix->n[1][0] -= axis_sine.k, matrix->n[0][1] += axis_sine.k;
    matrix->n[1][1] = axis_squared.j + cosine * (1.0f - axis_squared.j);
    matrix->n[2][0] = matrix->n[0][2] = axis->k * axis->i * (1.0f - cosine); matrix->n[2][0] += axis_sine.j, matrix->n[0][2] -= axis_sine.j;
    matrix->n[2][2] = axis_squared.k + cosine * (1.0f - axis_squared.k);
    matrix->n[2][1] = matrix->n[1][2] = axis->j * axis->k * (1.0f - cosine); matrix->n[2][1] -= axis_sine.i, matrix->n[1][2] += axis_sine.i;

    return matrix;
}

real_matrix3x3 *matrix3x3_multiply(const real_matrix3x3 *a, const real_matrix3x3 *b, real_matrix3x3 *result) {
    real_matrix3x3 ac = *a;
    real_matrix3x3 bc = *b;

    result->n[0][0] = ac.n[0][0] * bc.n[0][0] + ac.n[1][0] * bc.n[0][1] + ac.n[2][0] * bc.n[0][2];
    result->n[0][1] = ac.n[0][1] * bc.n[0][0] + ac.n[1][1] * bc.n[0][1] + ac.n[2][1] * bc.n[0][2];
    result->n[0][2] = ac.n[0][2] * bc.n[0][0] + ac.n[1][2] * bc.n[0][1] + ac.n[2][2] * bc.n[0][2];

    result->n[1][0] = ac.n[0][0] * bc.n[1][0] + ac.n[1][0] * bc.n[1][1] + ac.n[2][0] * bc.n[1][2];
    result->n[1][1] = ac.n[0][1] * bc.n[1][0] + ac.n[1][1] * bc.n[1][1] + ac.n[2][1] * bc.n[1][2];
    result->n[1][2] = ac.n[0][2] * bc.n[1][0] + ac.n[1][2] * bc.n[1][1] + ac.n[2][2] * bc.n[1][2];

    result->n[2][0] = ac.n[0][0] * bc.n[2][0] + ac.n[1][0] * bc.n[2][1] + ac.n[2][0] * bc.n[2][2];
    result->n[2][1] = ac.n[0][1] * bc.n[2][0] + ac.n[1][1] * bc.n[2][1] + ac.n[2][1] * bc.n[2][2];
    result->n[2][2] = ac.n[0][2] * bc.n[2][0] + ac.n[1][2] * bc.n[2][1] + ac.n[2][2] * bc.n[2][2];

    return result;
}

real_vector3d *matrix3x3_transform_vector(const real_matrix3x3 *matrix, const real_vector3d *vector, real_vector3d *result) {
    real_vector3d vc = *vector;

    result->i = vc.i * matrix->n[0][0] + vc.j * matrix->n[1][0] + vc.k * matrix->n[2][0];
    result->j = vc.i * matrix->n[0][1] + vc.j * matrix->n[1][1] + vc.k * matrix->n[2][1];
    result->k = vc.i * matrix->n[0][2] + vc.j * matrix->n[1][2] + vc.k * matrix->n[2][2];

    return result;
}

real_quaternion *matrix3x3_rotation_to_quaternion(const real_matrix3x3 *matrix, real_quaternion *quaternion) {
    math_assert(valid_real_vector3d_axes3(&matrix->forward, &matrix->left, &matrix->up));

    real tr = matrix->n[0][0] + matrix->n[1][1] + matrix->n[2][2];
    real s;
    if(tr > 0.0f) {
        s = square_root(tr + 1.0f);

        quaternion->w = s / 2.0f;
        s = 0.5f / s;
        quaternion->v.i = (matrix->n[2][1] - matrix->n[1][2]) * s;
        quaternion->v.j = (matrix->n[0][2] - matrix->n[2][0]) * s;
        quaternion->v.k = (matrix->n[1][0] - matrix->n[0][1]) * s;
    }
    else {
        int16_t i = 0;
        if(matrix->n[1][1]>matrix->n[0][0]) {
            i = 1;
        }
        if(matrix->n[2][2]>matrix->n[i][i]) {
            i = 2;
        }

        static int16_t next_field[3] = {1, 2, 0};
        int16_t j = next_field[i];
        int16_t k = next_field[j];
        s = square_root((matrix->n[i][i] - (matrix->n[j][j] + matrix->n[k][k])) + 1.0f);

        real q[3];
        q[i] = s * 0.5f;

        if(s != 0.0f) {
            s = 0.5f / s;
        }
        q[j] = (matrix->n[j][i] + matrix->n[i][j]) * s;
        q[k] = (matrix->n[k][i] + matrix->n[i][k]) * s;

        quaternion->w = (matrix->n[k][j] - matrix->n[j][k]) * s;
        quaternion->v.i = q[0];
        quaternion->v.j = q[1];
        quaternion->v.k = q[2];
    }

    return quaternion;
}
