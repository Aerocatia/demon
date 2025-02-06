use c_mine::c_mine;

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
}

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
