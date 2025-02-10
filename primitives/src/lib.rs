#![no_std]

use core::fmt::{Debug, Formatter};
use core::marker::PhantomData;
use num_enum::TryFromPrimitive;

pub mod vector;
pub mod color;
pub mod data;
pub mod string;
pub mod float;
pub mod tag_group;

#[derive(Copy, Clone, PartialEq)]
#[repr(transparent)]
pub struct EnumWrapper<T: TryFromPrimitive<Primitive = u16> + Copy + Clone + NamedTagStruct + Debug> {
    pub value: u16,
    pub _phantom_data: PhantomData<T>
}
impl<T: TryFromPrimitive<Primitive = u16> + Copy + Clone + NamedTagStruct + Debug> EnumWrapper<T> {
    pub fn get(self) -> T {
        match self.try_get() {
            Some(t) => t,
            None => {
                let name = T::name();
                let value = self.value;
                panic!("{value} is out-of-range for flag {name}");
            }
        }
    }
    pub fn try_get(self) -> Option<T> {
        T::try_from_primitive(self.value).ok()
    }
}
impl<T: TryFromPrimitive<Primitive = u16> + Copy + Clone + NamedTagStruct + Debug> Debug for EnumWrapper<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self.try_get() {
            Some(t) => Debug::fmt(&t, f),
            None => f.write_fmt(format_args!("<invalid {} = 0x{:04X}>", T::name(), self.value))
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(C)]
pub struct Bounds<T: Sized> {
    pub lower_bound: T,
    pub upper_bound: T
}

pub trait NamedTagStruct: Sized {
    fn name() -> &'static str;
}
