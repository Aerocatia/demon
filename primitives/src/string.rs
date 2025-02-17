use core::ffi::CStr;
use core::fmt::{Display, Formatter};

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct String32([u8; 32]);
impl String32 {
    pub const fn to_str(&self) -> &str {
        let Ok(s) = self.to_cstr().to_str() else {
            panic!("not UTF-8")
        };
        s
    }
    pub const fn to_cstr(&self) -> &CStr {
        let Ok(s) = CStr::from_bytes_until_nul(&self.0) else {
            panic!("not null terminated")
        };
        s
    }
    pub fn from_str(str: &str) -> Option<Self> {
        let mut into = [0u8; 32];
        let bytes = str.as_bytes();
        if bytes.len() >= into.len() {
            return None
        }
        into[..bytes.len()].copy_from_slice(bytes);
        Some(Self(into))
    }
}
impl Display for String32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.to_str())
    }
}
