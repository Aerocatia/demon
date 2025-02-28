use core::ops::{Add, Mul, Neg, Sub};
use crate::float::FloatFunctions;

pub const MIN_MAGNITUDE: f32 = 0.0001;

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct Matrix3x3 {
    pub a: Vector3D,
    pub b: Vector3D,
    pub c: Vector3D
}

impl Matrix3x3 {
    pub const IDENTITY: Matrix3x3 = Matrix3x3 {
        a: Vector3D { x: 1.0, y: 0.0, z: 0.0 },
        b: Vector3D { x: 0.0, y: 1.0, z: 0.0 },
        c: Vector3D { x: 0.0, y: 0.0, z: 1.0 },
    };

    pub const fn multiply(&self, by: &Self) -> Self {
        Matrix3x3 {
            a: Vector3D {
                x: by.a.x * self.a.x + by.a.y * self.b.x + by.a.z * self.c.x,
                y: by.a.x * self.a.y + by.a.y * self.b.y + by.a.z * self.c.y,
                z: by.a.x * self.a.z + by.a.y * self.b.z + by.a.z * self.c.z
            },
            b: Vector3D {
                x: by.b.x * self.a.x + by.b.y * self.b.x + by.b.z * self.c.x,
                y: by.b.x * self.a.y + by.b.y * self.b.y + by.b.z * self.c.y,
                z: by.b.x * self.a.z + by.b.y * self.b.z + by.b.z * self.c.z
            },
            c: Vector3D {
                x: by.c.x * self.a.x + by.c.y * self.b.x + by.c.z * self.c.x,
                y: by.c.x * self.a.y + by.c.y * self.b.y + by.c.z * self.c.y,
                z: by.c.x * self.a.z + by.c.y * self.b.z + by.c.z * self.c.z
            }
        }
    }
}

impl Mul<Matrix3x3> for Matrix3x3 {
    type Output = Self;

    fn mul(self, rhs: Matrix3x3) -> Self::Output {
        self.multiply(&rhs)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quaternion {
    pub const IDENTITY: Self = Self { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };

    pub const fn square_length(self) -> f32 {
        self.dot(self)
    }

    pub const fn as_matrix(self) -> Matrix3x3 {
        let square_length = self.square_length();
        if square_length.is_nan() || square_length == 0.0 {
            return Matrix3x3::IDENTITY;
        }

        let doubled_inverse_square_length = 2.0 / square_length;

        let inv_x = self.x * doubled_inverse_square_length;
        let inv_y = self.y * doubled_inverse_square_length;
        let inv_z = self.z * doubled_inverse_square_length;

        let wx = self.w * inv_x;
        let wy = self.w * inv_y;
        let wz = self.w * inv_z;
        let xx = self.x * inv_x;
        let xy = self.x * inv_y;
        let xz = self.x * inv_z;
        let yy = self.y * inv_y;
        let yz = self.y * inv_z;
        let zz = self.z * inv_z;

        Matrix3x3 {
            a: Vector3D {
                x: 1.0 - (yy + zz),
                y: xy - wz,
                z: xz + wy
            },
            b: Vector3D {
                x: xy + wz,
                y: 1.0 - (xx + zz),
                z: yz - wx
            },
            c: Vector3D {
                x: xz - wy,
                y: yz + wx,
                z: 1.0 - (xx + yy)
            }
        }
    }
    pub fn normalized(self) -> Quaternion {
        let square_length = self.square_length();
        if square_length <= 0.0 {
            return Self::IDENTITY
        }

        let inv = square_length.inverse_sqrt();
        Self {
            x: self.x * inv,
            y: self.y * inv,
            z: self.z * inv,
            w: self.w * inv
        }
    }

    /// Interpolate this quaternion with another one by `by` amount, returning a normalized vector.
    ///
    /// This function is more accurate than [linear_interpolated](Self::linear_interpolated), but it
    /// is less performant.
    ///
    /// This function returns a normalized vector. If one isn't necessary, use
    /// [interpolated_unnormalized](Self::interpolated_unnormalized).
    pub fn interpolated(self, with: Quaternion, by: f32) -> Quaternion {
        self.interpolated_unnormalized(with, by).normalized()
    }

    /// Linear interpolate this quaternion with another one by `by` amount, returning a normalized
    /// vector.
    ///
    /// This function is faster than [interpolate](Self::interpolated) but less accurate.
    ///
    /// This function returns a normalized vector. If one isn't necessary, use
    /// [linear_interpolated_unnormalized](Self::linear_interpolated_unnormalized).
    pub fn linear_interpolated(self, with: Quaternion, by: f32) -> Quaternion {
        self.linear_interpolated_unnormalized(with, by).normalized()
    }

    /// Interpolate this quaternion with another one by `by` amount, returning an unnormalized
    /// vector.
    ///
    /// This function is more accurate than [linear_interpolated_unnormalized](Self::linear_interpolated_unnormalized),
    /// but it is less performant.
    ///
    /// This function returns a (most likely) unnormalized vector. If one is necessary, use
    /// [interpolated](Self::interpolated).
    pub fn interpolated_unnormalized(self, with: Quaternion, by: f32) -> Quaternion {
        // special thanks to MosesOfEgypt for the rotation interpolation stuff here
        let mut cos_half_theta = self.dot(with);

        let mut with_n = with;
        if cos_half_theta < 0.0 {
            with_n = -with_n;
            cos_half_theta = -cos_half_theta;
        }

        if cos_half_theta.fabs() < 0.01 {
            return self.linear_interpolated_unnormalized(with, by)
        }

        let half_theta = cos_half_theta.min(1.0).acos();
        let m = 1.0 - cos_half_theta*cos_half_theta;
        let sin_half_theta = m.max(0.0);

        let mut r0 = 1.0 - by;
        let mut r1 = by;

        if sin_half_theta > 0.00001 {
            r0 = (r0 * half_theta).sin() / sin_half_theta;
            r1 = (r1 * half_theta).sin() / sin_half_theta;
        }

        with_n * r1 + self * r0
    }

    /// Linear interpolate this quaternion with another one by `by` amount, returning an
    /// unnormalized vector.
    ///
    /// This function is faster than [interpolated_unnormalized](Self::interpolated_unnormalized)
    /// but less accurate.
    ///
    /// This function returns a (most likely) unnormalized vector. If one is necessary, use
    /// [linear_interpolated](Self::linear_interpolated).
    pub fn linear_interpolated_unnormalized(self, with: Quaternion, by: f32) -> Quaternion {
        // linear interpolate; this is not very good, but this is how Halo originally does it
        let dot = self.dot(with);

        let this_amt = 1.0 - by;
        let with_amt = if dot < 0.0 {
            -by
        }
        else {
            by
        };

        self * this_amt + with * with_amt
    }

    const fn dot(self, with: Quaternion) -> f32 {
        let xx = self.x * with.x;
        let yy = self.y * with.y;
        let zz = self.z * with.z;
        let ww = self.w * with.w;
        xx + yy + zz + ww
    }

    const fn multiplied_by(self, by: f32) -> Quaternion {
        Quaternion {
            x: self.x * by,
            y: self.y * by,
            z: self.z * by,
            w: self.w * by,
        }
    }
}

impl Neg for Quaternion {
    type Output = Quaternion;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<f32> for Quaternion {
    type Output = Quaternion;
    fn mul(self, rhs: f32) -> Self::Output {
        self.multiplied_by(rhs)
    }
}

impl Add<Quaternion> for Quaternion {
    type Output = Quaternion;
    fn add(self, rhs: Quaternion) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Sub<Quaternion> for Quaternion {
    type Output = Quaternion;
    fn sub(self, rhs: Quaternion) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32
}

impl Vector2D {
    pub fn is_valid(self) -> bool {
        !self.x.is_nan() && !self.y.is_nan()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vector3D {
    pub const fn is_valid(self) -> bool {
        !self.x.is_nan() && !self.y.is_nan() && !self.z.is_nan()
    }
    pub const fn dot(self, other: Vector3D) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub const fn scale(self, by: f32) -> Self {
        Self {
            x: self.x * by,
            y: self.y * by,
            z: self.z * by
        }
    }
    pub const fn magnitude_squared(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn magnitude(self) -> f32 {
        self.magnitude_squared().sqrt()
    }
    pub fn normalized(self) -> Option<Self> {
        let magnitude = self.magnitude();
        if magnitude < MIN_MAGNITUDE {
            None
        }
        else {
            // Bad for floating point precision, but needed to be accurate to the original...
            Some(self.scale(1.0 / magnitude))
        }
    }
    pub const fn negated(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl Neg for Vector3D {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.negated()
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Vector2DInt {
    pub x: i16,
    pub y: i16
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Euler2D {
    pub yaw: f32,
    pub pitch: f32
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Euler3D {
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Rectangle {
    pub top: i16,
    pub left: i16,
    pub bottom: i16,
    pub right: i16
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Plane2D {
    pub offset: f32,
    pub vector: Vector2D
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Plane3D {
    pub vector: Vector3D,
    pub offset: f32,
}
impl Plane3D {
    pub const fn distance_to_point(self, point: Vector3D) -> f32 {
        let dot = point.dot(self.vector);
        dot - self.offset
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct Angle(pub f32);

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct CompressedFloat(pub u16);

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct CompressedVector2D(pub u32);

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct CompressedVector3D(pub u32);

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct Matrix4x3 {
    pub scale: f32,
    pub rotation_matrix: Matrix3x3,
    pub position: Vector3D
}

impl Matrix4x3 {
    pub const fn from_matrix3x3(matrix3x3: Matrix3x3) -> Self {
        Self {
            scale: 1.0,
            rotation_matrix: matrix3x3,
            position: Vector3D { x: 0.0, y: 0.0, z: 0.0 }
        }
    }
    pub const fn multiply(&self, by: &Self) -> Self {
        Self {
            scale: self.scale * by.scale,
            position: Vector3D {
                x: (by.position.x * self.rotation_matrix.a.x + by.position.y * self.rotation_matrix.b.x + by.position.z * self.rotation_matrix.c.x) * self.scale + self.position.x,
                y: (by.position.x * self.rotation_matrix.a.y + by.position.y * self.rotation_matrix.b.y + by.position.z * self.rotation_matrix.c.y) * self.scale + self.position.y,
                z: (by.position.x * self.rotation_matrix.a.z + by.position.y * self.rotation_matrix.b.z + by.position.z * self.rotation_matrix.c.z) * self.scale + self.position.z
            },
            rotation_matrix: self.rotation_matrix.multiply(&by.rotation_matrix)
        }
    }
    pub const fn from_point_and_quaternion(point: Vector3D, quaternion: Quaternion) -> Self {
        Self {
            position: point,
            ..Self::from_matrix3x3(quaternion.as_matrix())
        }
    }
}

impl Mul<Matrix4x3> for Matrix4x3 {
    type Output = Self;

    fn mul(self, rhs: Matrix4x3) -> Self::Output {
        self.multiply(&rhs)
    }
}

impl From<Matrix3x3> for Matrix4x3 {
    fn from(value: Matrix3x3) -> Self {
        Self::from_matrix3x3(value)
    }
}

const _: () = assert!(size_of::<Matrix4x3>() == 0x34);
