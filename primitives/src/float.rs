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
        libm::roundf(self).floor_to_int()
    }
    fn floor_to_int(self) -> i32 {
        self as i32
    }
}
