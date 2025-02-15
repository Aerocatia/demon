use crate::float::FloatFunctions;

pub const MIN_MAGNITUDE: f32 = 0.0001;

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Matrix3x3 {
    pub vectors: [Vector3D; 3]
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
