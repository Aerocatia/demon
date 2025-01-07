use core::ffi::{c_char, CStr};
use crate::id::TagID;
use crate::util::VariableProvider;

pub const CACHE_TAGS_ADDRESS: VariableProvider<*const CacheTag> = VariableProvider {
    name: "CACHE_TAGS_ADDRESS",
    cache_address: 0x00AF8364 as *mut _,
    tags_address: 0x00000000 as *mut _
};

pub const CACHE_TAGS_ARE_LOADED: VariableProvider<u8> = VariableProvider {
    name: "CACHE_TAGS_ARE_LOADED",
    cache_address: 0x00AF8368 as *mut _,
    tags_address: 0x00000000 as *mut _
};

pub trait TagIndex {
    fn get_primary_tag_group(&self) -> TagGroupUnsafe;
    fn get_secondary_tag_group(&self) -> TagGroupUnsafe;
    fn get_tertiary_tag_group(&self) -> TagGroupUnsafe;

    /// Attempt to get the tag path.
    ///
    /// # Panics
    ///
    /// Panics if tag_path is null or is not valid UTF-8.
    ///
    /// # Safety
    ///
    /// This is unsafe because tag_path is not verified to be accurate or even pointing to anything.
    unsafe fn get_tag_path(&self) -> &str;
    fn get_tag_data(&self) -> *mut u8;
}

#[repr(C)]
pub struct CacheTag {
    pub primary_tag_group: TagGroupUnsafe,
    pub secondary_tag_group: TagGroupUnsafe,
    pub tertiary_tag_group: TagGroupUnsafe,
    pub tag_id: TagID,
    pub tag_path: *const c_char,
    pub tag_data: *mut u8,
    pub external: u32,
    pub padding: u32
}
impl TagIndex for CacheTag {
    fn get_primary_tag_group(&self) -> TagGroupUnsafe {
        self.primary_tag_group
    }

    fn get_secondary_tag_group(&self) -> TagGroupUnsafe {
        self.secondary_tag_group
    }

    fn get_tertiary_tag_group(&self) -> TagGroupUnsafe {
        self.tertiary_tag_group
    }

    unsafe fn get_tag_path(&self) -> &str {
        assert!(!self.tag_path.is_null(), "Tag path is null!");
        CStr::from_ptr(self.tag_path).to_str().expect("Tag path is not UTF-8!")
    }

    fn get_tag_data(&self) -> *mut u8 {
        self.tag_data
    }
}

#[repr(C)]
pub struct TagInstance {
    pub tag_path: [u8; 256],
    pub primary_tag_group: TagGroupUnsafe,
    pub secondary_tag_group: TagGroupUnsafe,
    pub tertiary_tag_group: TagGroupUnsafe,
    /// 0x00000000?
    pub idk1: u32,
    /// 0xFFFFFFFF?
    pub idk2: u32,
    pub crc: u32,
    pub valid: u32,
    pub tag_data: *mut u8,
    pub tag_definitions: *const u8,
}
impl TagIndex for TagInstance {
    fn get_primary_tag_group(&self) -> TagGroupUnsafe {
        self.primary_tag_group
    }

    fn get_secondary_tag_group(&self) -> TagGroupUnsafe {
        self.secondary_tag_group
    }

    fn get_tertiary_tag_group(&self) -> TagGroupUnsafe {
        self.tertiary_tag_group
    }

    unsafe fn get_tag_path(&self) -> &str {
        CStr::from_bytes_with_nul(self.tag_path.as_ref())
            .expect("Tag path is not a null-terminated C string!")
            .to_str()
            .expect("Tag path is not UTF-8!")
    }

    fn get_tag_data(&self) -> *mut u8 {
        self.tag_data
    }
}

#[repr(u32)]
pub enum TagGroup {
    Something
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct TagGroupUnsafe(u32);