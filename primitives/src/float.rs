pub trait FloatFunctions: Copy + Copy {
    fn sqrt(self) -> Self;
    fn powi(self, exponent: i32) -> Self;
    fn powf(self, exponent: Self) -> Self;
    fn fabs(self) -> Self;
}

impl FloatFunctions for f32 {
    fn powf(self, exponent: Self) -> Self {
        libm::powf(self, exponent)
    }
    fn powi(self, exponent: i32) -> Self {
        self.powf(exponent as f32)
    }
    fn sqrt(self) -> Self {
        libm::sqrtf(self)
    }
    fn fabs(self) -> Self {
        libm::fabsf(self)
    }
}
