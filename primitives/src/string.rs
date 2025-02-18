use core::ffi::CStr;
use core::fmt::{Display, Formatter};

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct String32([u8; 32]);
impl String32 {
    pub const fn to_str(&self) -> &str {
        let Ok(s) = self.to_cstr().to_str() else {
            panic!("String32 was not UTF-8")
        };
        s
    }
    pub const fn to_cstr(&self) -> &CStr {
        let Ok(s) = CStr::from_bytes_until_nul(&self.0) else {
            panic!("String32 was not null terminated")
        };
        s
    }
    pub fn string_bytes(&self) -> &[u8] {
        let length = self.0.as_slice().iter().position(|b| *b == 0).unwrap_or(self.0.len());
        &self.0[..length]
    }
    pub const fn to_bytes(&self) -> &[u8] {
        self.0.as_slice()
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
