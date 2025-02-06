pub trait Sqrt {
    fn sqrt(&self) -> Self;
}

impl Sqrt for f32 {
    fn sqrt(&self) -> Self {
        libm::sqrtf(*self)
    }
}
