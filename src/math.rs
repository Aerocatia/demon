use c_mine::c_mine;

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

#[repr(C)]
pub struct ColorARGB {
    pub alpha: f32,
    pub color: ColorRGB
}

impl ColorARGB {
    pub fn is_valid(&self) -> bool {
        (0.0..=1.0).contains(&self.alpha) && self.color.is_valid()
    }
}

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
