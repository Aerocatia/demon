// These should be static inline and in real_math.h, but are not in the debug exe

#include "../cseries/cseries.h"
#include "real_math.h"

real magnitude_squared2d(const real_vector2d *v) {
    return v->i * v->i + v->j * v->j;
}

real dot_product3d(const real_vector3d *a, const real_vector3d *b) {
    return a->i * b->i + a->j * b->j + a->k * b->k;
}

real magnitude_squared3d(const real_vector3d *v) {
    return v->i * v->i + v->j * v->j + v->k * v->k;
}

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
