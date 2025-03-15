use c_mine::c_mine;
use tag_structs::primitives::color::{ColorARGB, ColorRGB};
use tag_structs::primitives::float::FloatFunctions;
use tag_structs::primitives::vector::{Matrix3x3, Matrix4x3, Plane3D, Quaternion, Vector2D, Vector3D};

/// 1 world unit = 10 feet = 3.048 meters
#[inline(always)]
pub const fn world_units_to_meters(world_units: f32) -> f32 {
    world_units * 3.048
}

#[c_mine]
pub extern "C" fn is_valid_real(real: f32) -> bool {
    !real.is_nan()
}

#[c_mine]
pub extern "C" fn is_valid_rgb_color(rgb: &ColorRGB) -> bool {
    rgb.is_valid()
}

#[c_mine]
pub extern "C" fn is_valid_argb_color(argb: &ColorARGB) -> bool {
    argb.is_valid()
}

#[c_mine]
pub extern "C" fn is_valid_vector2d(vector: &Vector2D) -> bool {
    vector.is_valid()
}

#[c_mine]
pub extern "C" fn is_valid_point2d(point: &Vector2D) -> bool {
    point.is_valid()
}

#[c_mine]
pub extern "C" fn is_valid_vector3d(vector: &Vector3D) -> bool {
    vector.is_valid()
}

#[c_mine]
pub extern "C" fn is_valid_point3d(point: &Vector3D) -> bool {
    point.is_valid()
}

#[c_mine]
pub extern "C" fn dot_product_3d(a: &Vector3D, b: &Vector3D) -> f32 {
    a.dot(b)
}

#[c_mine]
pub extern "C" fn scale_vector_3d(from: &Vector3D, by: f32, to: &mut Vector3D) -> *mut Vector3D {
    *to = from.scale(by);
    to
}

#[c_mine]
pub extern "C" fn magnitude3d(vector: &Vector3D) -> f32 {
    vector.magnitude()
}

#[c_mine]
pub extern "C" fn magnitude_squared3d(vector: &Vector3D) -> f32 {
    vector.magnitude_squared()
}

#[c_mine]
pub extern "C" fn sqrt(value: f32) -> f32 {
    value.sqrt()
}

#[c_mine]
pub extern "C" fn powf(base: f32, exponent: f32) -> f32 {
    base.powf(exponent)
}

#[c_mine]
pub extern "C" fn fmod(base: f32, modulo: f32) -> f32 {
    base % modulo
}

#[c_mine]
pub extern "C" fn powi(base: f32, exponent: i32) -> f32 {
    base.powi(exponent)
}

#[c_mine]
pub extern "C" fn fabs(float: f32) -> f32 {
    float.fabs()
}

#[c_mine]
pub extern "C" fn normalize_3d(vector: &mut Vector3D) -> f32 {
    let magnitude = vector.magnitude();
    if let Some(normalized) = vector.normalized() {
        *vector = normalized;
        magnitude
    }
    else {
        // ...don't actually normalize the vector, and then hope that the game doesn't explode!
        0.0
    }
}

#[c_mine]
pub extern "C" fn matrix3x3_multiply(a: &Matrix3x3, b: &Matrix3x3, to: &mut Matrix3x3) {
    *to = a.multiply(b)
}

#[c_mine]
pub extern "C" fn matrix4x3_multiply(a: &Matrix4x3, b: &Matrix4x3, to: &mut Matrix4x3) {
    *to = a.multiply(b)
}

#[c_mine]
pub extern "C" fn matrix4x3_rotation_from_quaternion(matrix4x3: &mut Matrix4x3, quaternion: &Quaternion) {
    *matrix4x3 = quaternion.as_matrix().into();
}

#[c_mine]
pub extern "C" fn matrix4x3_from_point_from_quaternion(matrix4x3: &mut Matrix4x3, point: &Vector3D, quaternion: &Quaternion) {
    *matrix4x3 = Matrix4x3::from_point_and_quaternion(*point, *quaternion);
}

#[c_mine]
pub extern "C" fn negate_vector3d(from: &Vector3D, to: &mut Vector3D) -> *mut Vector3D {
    *to = from.negated();
    to
}

#[c_mine]
pub extern "C" fn plane3d_distance_to_point(plane: &Plane3D, point: &Vector3D) -> f32 {
    plane.distance_to_point(*point)
}

#[c_mine]
pub extern "C" fn round(what: f32) -> i32 {
    // If rounding is not done exactly this way, it leads to death!
    what.round_ties_even_to_int()
}

#[c_mine]
pub extern "C" fn inverse_square_root(what: f32) -> f32 {
    what.inverse_sqrt()
}

#[c_mine]
pub extern "C" fn quaternion_normalize(quaternion: &mut Quaternion) {
    *quaternion = quaternion.normalized()
}

#[c_mine]
pub extern "C" fn quaternions_interpolate(a: &Quaternion, b: &Quaternion, by: f32, output: &mut Quaternion) {
    *output = a.linear_interpolated_unnormalized(*b, by)
}

#[c_mine]
pub extern "C" fn quaternions_interpolate_and_normalize(a: &Quaternion, b: &Quaternion, by: f32, output: &mut Quaternion) {
    *output = a.linear_interpolated(*b, by)
}

#[c_mine]
pub extern "C" fn matrix4x3_transform_normal(matrix: &Matrix4x3, normal: &Vector3D, output: &mut Vector3D) -> *mut Vector3D {
    *output = matrix.transform_normal(normal);
    output
}

#[c_mine]
pub extern "C" fn matrix4x3_transform_plane(matrix: &Matrix4x3, plane: &Plane3D, output: &mut Plane3D) -> *mut Plane3D {
    *output = matrix.transform_plane(plane);
    output
}

#[c_mine]
pub extern "C" fn matrix4x3_transform_point(matrix: &Matrix4x3, point: &Vector3D, output: &mut Vector3D) -> *mut Vector3D {
    *output = matrix.transform_point(point);
    output
}

#[c_mine]
pub extern "C" fn matrix4x3_transform_vector(matrix: &Matrix4x3, vector: &Vector3D, output: &mut Vector3D) -> *mut Vector3D {
    *output = matrix.transform_vector(vector);
    output
}

#[c_mine]
pub extern "C" fn matrix3x3_transform_vector(matrix: &Matrix3x3, vector: &Vector3D, output: &mut Vector3D) -> *mut Vector3D {
    *output = matrix.transform_vector(vector);
    output
}

#[c_mine]
pub extern "C" fn tan(value: f32) -> f32 {
    value.tan()
}

#[c_mine]
pub extern "C" fn flipped_subtract_vectors_3d(a: &Vector3D, b: &Vector3D, out: &mut Vector3D) -> *mut Vector3D {
    *out = *b - *a;
    out
}
