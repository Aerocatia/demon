#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct ColorARGB {
    /// Alpha channel in the range of `[0..1]`
    pub a: f32,
    pub color: ColorRGB
}
impl ColorARGB {
    pub fn is_valid(&self) -> bool {
        (0.0..=1.0).contains(&self.a) && self.color.is_valid()
    }
    pub fn to_pixel32(&self) -> Pixel32 {
        let a = (self.a * 255.0) as u32;
        let r = (self.color.r * 255.0) as u32;
        let g = (self.color.g * 255.0) as u32;
        let b = (self.color.b * 255.0) as u32;
        Pixel32((a << 24) | (r << 16) | (g << 8) | b)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct ColorRGB {
    /// Red channel in the range of `[0..1]`
    pub r: f32,

    /// Green channel in the range of `[0..1]`
    pub g: f32,

    /// Blue channel in the range of `[0..1]`
    pub b: f32
}

impl ColorRGB {
    pub fn is_valid(&self) -> bool {
        (0.0..=1.0).contains(&self.r) && (0.0..=1.0).contains(&self.g) && (0.0..=1.0).contains(&self.b)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct Pixel32(u32);
