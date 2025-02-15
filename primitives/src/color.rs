#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct ColorARGB {
    /// Alpha channel in the range of `[0..1]`
    pub a: f32,
    pub color: ColorRGB
}
impl AsRef<ColorARGB> for ColorARGB {
    fn as_ref(&self) -> &ColorARGB {
        self
    }
}
impl ColorARGB {
    pub const fn zeroed() -> Self {
        ColorARGB {
            a: 0.0,
            color: ColorRGB::BLACK
        }
    }
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
    pub const WHITE: ColorRGB = ColorRGB { r: 1.0, g: 1.0, b: 1.0 };
    pub const BLACK: ColorRGB = ColorRGB { r: 0.0, g: 0.0, b: 0.0 };
    pub fn is_valid(&self) -> bool {
        (0.0..=1.0).contains(&self.r) && (0.0..=1.0).contains(&self.g) && (0.0..=1.0).contains(&self.b)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct Pixel32(u32);
