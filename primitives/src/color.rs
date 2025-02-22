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
    pub const fn is_valid(&self) -> bool {
        self.r >= 0.0 && self.r <= 1.0
            && self.g >= 0.0 && self.g <= 1.0
            && self.b >= 0.0 && self.b <= 1.0
    }
    pub const fn as_colorargb(self) -> ColorARGB {
        ColorARGB { a: 1.0, color: self }
    }
    pub fn clamped(self) -> ColorRGB {
        ColorRGB { r: self.r.clamp(0.0, 1.0), g: self.g.clamp(0.0, 1.0), b: self.b.clamp(0.0, 1.0) }
    }
}

impl From<ColorRGB> for ColorARGB {
    fn from(value: ColorRGB) -> Self {
        value.as_colorargb()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct Pixel32(u32);
