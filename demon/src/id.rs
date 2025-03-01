use core::fmt::{Debug, Formatter};

const NULL_ID: u32 = 0xFFFFFFFF;

#[derive(Copy, Clone, PartialEq, PartialOrd, Ord, Eq)]
#[repr(transparent)]
pub struct ID<const SALT: u16> {
    full_id: u32
}
impl<const SALT: u16> ID<SALT> {
    pub const NULL: Self = Self { full_id: NULL_ID };
    pub const fn from_index(index: usize) -> Result<Self, &'static str> {
        if index > u16::MAX as usize {
            return Err("index is too big")
        }

        let index = (index & 0xFFFF) as u16;
        Ok(Self {
            full_id: make_id(index, SALT)
        })
    }
    pub const fn from_full_id(full_id: u32) -> Self {
        Self { full_id }
    }
    pub const fn full_id(&self) -> u32 {
        self.full_id
    }

    pub const fn is_null(&self) -> bool {
        self.full_id == NULL_ID
    }

    pub const fn index(&self) -> Option<usize> {
        if !self.is_null() {
            Some((self.full_id & 0xFFFF) as usize)
        }
        else {
            None
        }
    }

    pub const fn identifier(&self) -> Option<u16> {
        if !self.is_null() {
            Some((self.full_id >> 16) as u16)
        }
        else {
            None
        }
    }

    pub const fn is_valid(&self) -> bool {
        let Some(index) = self.index() else {
            return true
        };

        let Ok(redone) = Self::from_index(index) else {
            unreachable!()
        };

        redone.full_id == self.full_id
    }
}
impl<const SALT: u16> Default for ID<SALT> {
    fn default() -> Self {
        Self::NULL
    }
}
impl<const SALT: u16> Debug for ID<SALT> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("ID<SALT=0x{SALT:04X}; ID=0x{:08X}>", self.full_id))
    }
}

pub const fn make_id(index: u16, salt: u16) -> u32 {
    let salt = salt as u32;
    let index = (index & 0xFFFF) as u32;
    let high = (0x8000 | index.wrapping_add(salt)) << 16;
    index | high
}

