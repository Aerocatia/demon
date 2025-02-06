use crate::math::sqrt::Sqrt;

pub mod c;
pub mod sqrt;
pub mod powf;
pub mod powi;

#[derive(Copy, Clone, Default, Debug, PartialEq)]
#[repr(C)]
pub struct ColorRGB {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

impl ColorRGB {
    pub fn is_valid(&self) -> bool {
        (0.0..=1.0).contains(&self.r) && (0.0..=1.0).contains(&self.g) && (0.0..=1.0).contains(&self.b)
    }
}

#[derive(Copy, Clone, Default, Debug, PartialEq)]
#[repr(C)]
pub struct ColorARGB {
    pub alpha: f32,
    pub color: ColorRGB
}

impl ColorARGB {
    pub fn is_valid(&self) -> bool {
        (0.0..=1.0).contains(&self.alpha) && self.color.is_valid()
    }
    pub fn to_a8r8g8b8(&self) -> u32 {
        let alpha = (self.alpha * 255.0) as u32;
        let red = (self.color.r * 255.0) as u32;
        let green = (self.color.g * 255.0) as u32;
        let blue = (self.color.b * 255.0) as u32;

        (alpha << 24) | (red << 16) | (green << 8) | blue
    }
}

pub type Euler2D = Vector2D;

#[derive(Copy, Clone, Default, Debug, PartialEq)]
#[repr(C)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32
}

impl Vector2D {
    pub fn is_valid(&self) -> bool {
        !self.x.is_nan() && !self.y.is_nan()
    }
}

#[derive(Copy, Clone, Default, Debug, PartialEq)]
#[repr(C)]
pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vector3D {
    pub fn is_valid(&self) -> bool {
        !self.x.is_nan() && !self.y.is_nan() && !self.z.is_nan()
    }
    pub fn dot(&self, other: &Vector3D) -> f32 {
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
}
