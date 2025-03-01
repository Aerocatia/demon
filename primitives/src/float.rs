use core::cmp::Ordering;

pub trait FloatFunctions: Copy + Copy {
    fn sqrt(self) -> Self;
    fn inverse_sqrt(self) -> Self;
    fn powi(self, exponent: i32) -> Self;
    fn powf(self, exponent: Self) -> Self;
    fn fabs(self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn acos(self) -> Self;
    fn round_ties_even_to_int(self) -> i32;
    fn round_towards_zero_to_int(self) -> i32;
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
    fn inverse_sqrt(self) -> Self {
        1.0 / self.sqrt()
    }
    fn fabs(self) -> Self {
        libm::fabsf(self)
    }
    fn sin(self) -> Self { libm::sinf(self) }
    fn cos(self) -> Self { libm::cosf(self) }
    fn acos(self) -> Self { libm::acosf(self) }

    fn round_ties_even_to_int(self) -> i32 {
        let a = self.floor_to_int();
        let b = a.saturating_add(1);
        let low = self - (a as f32);
        let high = (b as f32) - self;

        match low.total_cmp(&high) {
            Ordering::Less => a,
            Ordering::Greater => b,

            // Round to the nearest even number
            Ordering::Equal => if (a & 1) != 0 { b } else { a }
        }
    }
    fn round_towards_zero_to_int(self) -> i32 {
        self as i32
    }
    fn floor_to_int(self) -> i32 {
        match self.total_cmp(&0.0) {
            Ordering::Equal => 0,
            Ordering::Greater => self.round_towards_zero_to_int(),
            Ordering::Less => self.round_towards_zero_to_int() - 1
        }
    }
}
