pub trait FloatFunctions: Copy + Copy {
    fn sqrt(self) -> Self;
    fn powi(self, exponent: i32) -> Self;
    fn powf(self, exponent: Self) -> Self;
    fn fabs(self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn round_to_int(self) -> i32;
    fn floor_to_int(self) -> i32;
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
    fn sin(self) -> Self { libm::sinf(self) }
    fn cos(self) -> Self { libm::cosf(self) }
    fn round_to_int(self) -> i32 {
        (self + 0.5) as i32
    }
    fn floor_to_int(self) -> i32 {
        if !self.is_finite() {
            return 0
        }
        unsafe {
            self.clamp(i32::MIN as f32, i32::MAX as f32).to_int_unchecked()
        }
    }
}
