use core::ops::Mul;
use crate::float::FloatFunctions;

pub const MIN_MAGNITUDE: f32 = 0.0001;

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Matrix3x3 {
    pub a: Vector3D,
    pub b: Vector3D,
    pub c: Vector3D
}

impl Matrix3x3 {
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
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32
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

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vector3D {
    pub fn is_valid(self) -> bool {
        !self.x.is_nan() && !self.y.is_nan() && !self.z.is_nan()
    }
    pub fn dot(self, other: Vector3D) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn scale(self, by: f32) -> Self {
        Self {
            x: self.x * by,
            y: self.y * by,
            z: self.z * by
        }
    }
    pub fn magnitude_squared(self) -> f32 {
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
    pub offset: f32,
    pub vector: Vector3D
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

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Matrix4x3 {
    pub scale: f32,
    pub rotation_matrix: Matrix3x3,
    pub position: Vector3D
}

impl Matrix4x3 {
    pub const fn multiply(&self, by: &Self) -> Self {
        Self {
            scale: self.scale * by.scale,
            position: Vector3D {
                x: by.position.x * self.rotation_matrix.a.x + by.position.y * self.rotation_matrix.b.x + by.position.z * self.rotation_matrix.c.x * self.scale + self.position.x,
                y: by.position.x * self.rotation_matrix.a.y + by.position.y * self.rotation_matrix.b.y + by.position.z * self.rotation_matrix.c.y * self.scale + self.position.y,
                z: by.position.x * self.rotation_matrix.a.z + by.position.y * self.rotation_matrix.b.z + by.position.z * self.rotation_matrix.c.z * self.scale + self.position.z
            },
            rotation_matrix: self.rotation_matrix.multiply(&by.rotation_matrix)
        }
    }
}

impl Mul<Matrix4x3> for Matrix4x3 {
    type Output = Self;

    fn mul(self, rhs: Matrix4x3) -> Self::Output {
        self.multiply(&rhs)
    }
}

const _: () = assert!(size_of::<Matrix4x3>() == 0x34);
