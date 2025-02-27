#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(C)]
pub struct Bounds<T: Sized> {
    pub lower_bound: T,
    pub upper_bound: T
}
