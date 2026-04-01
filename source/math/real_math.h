#ifndef __REAL_MATH_H__
#define __REAL_MATH_H__

#include <math.h>

/* ---------- constants */

#define REAL_MAX FLT_MAX
#define REAL_MIN (-FLT_MAX)

#define _2pi 6.28318530718f
#define _pi 3.14159265359f
#define _half_pi 1.570796326795f
#define _quarter_pi 0.7853981633974f

#define _e 2.71828182845f
#define _log2 0.69314718056f

#define _root2 1.41421356237f
#define _root3 1.73205080757f
#define _one_over_root2 0.70710678119f

#define _cosine30 0.866025403784f
#define _sine30 0.5f
#define _cosine45 _one_over_root2
#define _sine45 _one_over_root2
#define _cosine60 _sine30
#define _sine60 _cosine30

#define _deg2rad 0.01745329251994f
#define _rad2deg 57.29577951308f

#define _full_circle _2pi
#define _half_circle _pi
#define _quarter_circle _half_pi
#define _eighth_circle _quarter_pi

#define _real_epsilon .0001f
#define _valid_real_epsilon 0.001f

/* ---------- macros */

/* ---------- globals */

/* ---------- prototypes */

real magnitude_squared2d(const real_vector2d *v);
real dot_product3d(const real_vector3d *a, const real_vector3d *b);
real magnitude_squared3d(const real_vector3d *v);

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

#endif
