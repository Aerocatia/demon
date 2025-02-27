use core::fmt::{Debug, Formatter};
use core::marker::PhantomData;
use num_enum::TryFromPrimitive;
use crate::named_tag_struct::NamedTagStruct;

/// Wraps a 16-bit enum around a 16-bit value that may or may not actually correspond to a variant
/// of the enum.
#[derive(Copy, Clone, PartialEq)]
#[repr(transparent)]
pub struct EnumWrapper<T> {
    value: u16,
    _phantom_data: PhantomData<T>
}

impl<T> EnumWrapper<T> {
    /// Construct a wrapper from a raw u16.
    ///
    /// This value does not have to actually conform to a valid variant of `T`.
    pub const fn from_raw(value: u16) -> Self {
        Self {
            value,
            _phantom_data: PhantomData
        }
    }

    /// Construct a wrapper from a variant of `T`.
    pub fn from_value(value: T) -> Self where T: Into<u16> {
        Self {
            value: value.into(),
            _phantom_data: PhantomData
        }
    }
}

impl<T: TryFromPrimitive<Primitive = u16>> EnumWrapper<T> {
    /// Get the value as `T`.
    ///
    /// Panic if the value is out-of-range.
    pub fn get(self) -> T where T: NamedTagStruct {
        match self.try_get() {
            Ok(t) => t,
            Err(value) => {
                let name = T::name();
                panic!("{value} is out-of-range for flag {name}");
            }
        }
    }

    /// Get the value as `T`.
    ///
    /// Return Err if the value is out-of-range, containing the original value.
    pub fn try_get(self) -> Result<T, u16> {
        T::try_from_primitive(self.value).map_err(|_| self.value)
    }
}

impl<T: TryFromPrimitive<Primitive = u16> + Copy + Clone + NamedTagStruct + Debug> Debug for EnumWrapper<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self.try_get() {
            Ok(t) => Debug::fmt(&t, f),
            Err(value) => f.write_fmt(format_args!("<invalid {} = 0x{value:04X}>", T::name()))
        }
    }
}

impl<T: Into<u16> + Copy + Clone> PartialEq<T> for EnumWrapper<T> {
    fn eq(&self, other: &T) -> bool {
        Into::<u16>::into(*other) == self.value
    }
}

impl<T: NamedTagStruct> From<u16> for EnumWrapper<T> {
    fn from(value: u16) -> Self {
        Self::from_raw(value)
    }
}

impl<T: NamedTagStruct + Into<u16>> From<T> for EnumWrapper<T> {
    fn from(value: T) -> Self {
        Self::from_value(value)
    }
}

impl<T: Into<u16> + Default> Default for EnumWrapper<T> {
    fn default() -> Self {
        Self::from_value(T::default())
    }
}
