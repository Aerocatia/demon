use core::ffi::{c_char, CStr};
use c_mine::c_mine;
use crate::id::ID;
use crate::init::{get_exe_type, ExeType};
use crate::table::DataTable;
use crate::util::VariableProvider;

pub const TAG_ID_SALT: u16 = 0x6174;
pub type TagID = ID<TAG_ID_SALT>;

pub const TAGS_TAG_INSTANCES: VariableProvider<*mut DataTable<TagTagInstance, TAG_ID_SALT>> = VariableProvider {
    name: "TAGS_TAG_INSTANCES",
    cache_address: 0x00000000 as *mut _,
    tags_address: 0x00FFDAF8 as *mut _
};

pub const CACHE_TAG_INSTANCES: VariableProvider<*mut CacheTagInstance> = VariableProvider {
    name: "CACHE_TAG_INSTANCES",
    cache_address: 0x00AF8364 as *mut _,
    tags_address: 0x00000000 as *mut _
};

pub const CACHE_TAGS_ARE_LOADED: VariableProvider<u8> = VariableProvider {
    name: "CACHE_TAGS_ARE_LOADED",
    cache_address: 0x00AF8368 as *mut _,
    tags_address: 0x00000000 as *mut _
};

pub const CACHE_FILE_TAG_HEADER: VariableProvider<*mut CacheFileTagHeader> = VariableProvider {
    name: "CACHE_FILE_TAG_HEADER",
    cache_address: 0x00AF8B70 as *mut _,
    tags_address: 0x00000000 as *mut _
};

/// Get all cache file tags.
///
/// # Panics
///
/// Panics if not on a cache EXE.
pub fn get_cache_file_tags() -> &'static [CacheTagInstance] {
    // SAFETY: Should be set already.
    unsafe {
        if *CACHE_TAGS_ARE_LOADED.get() == 0 {
            return &[]
        }
        let cache_header = *CACHE_FILE_TAG_HEADER.get();
        assert!(!cache_header.is_null(), "CACHE_FILE_TAG_HEADER is null!");
        let tags = *CACHE_TAG_INSTANCES.get();
        assert!(!tags.is_null(), "CACHE_TAGS_ADDRESS is null!");
        core::slice::from_raw_parts(tags, (&*cache_header).tag_count as usize)
    }
}

pub struct CacheFileTagHeader {
    pub tags: *const CacheTagInstance,
    pub scenario_tag: TagID,
    pub checksum: u32,
    pub tag_count: u32
}

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

/// Used only in tag builds.
#[repr(C)]
pub struct CacheTagInstance {
    pub primary_tag_group: TagGroupUnsafe,
    pub secondary_tag_group: TagGroupUnsafe,
    pub tertiary_tag_group: TagGroupUnsafe,
    pub tag_id: TagID,
    pub tag_path: *const c_char,
    pub tag_data: *mut u8,
    pub external: u32,
    pub padding: u32
}
impl TagIndex for CacheTagInstance {
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

/// Used only in cache builds.
#[repr(C)]
pub struct TagTagInstance {
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
impl TagIndex for TagTagInstance {
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
        CStr::from_bytes_until_nul(self.tag_path.as_ref())
            .expect("Tag path is not a null-terminated C string!")
            .to_str()
            .expect("Tag path is not UTF-8!")
    }

    fn get_tag_data(&self) -> *mut u8 {
        self.tag_data
    }
}

#[c_mine]
pub unsafe extern "C" fn resolve_tag_loaded(group: TagGroupUnsafe, path: *const c_char) -> TagID {
    let path = CStr::from_ptr(path).to_str().expect("input tag is not UTF-8");
    match get_exe_type() {
        ExeType::Tag => {
            let pointer = *TAGS_TAG_INSTANCES.get_mut();
            assert!(!pointer.is_null(), "TAGS_TAG_INSTANCES is null!");

            let table: &'static mut DataTable<TagTagInstance, TAG_ID_SALT> = &mut *pointer;
            let mut iterator = table.iter();
            let Some(_) = (&mut iterator)
                .filter(|tag| tag.item.get_primary_tag_group() == group && tag.item.get_tag_path() == path)
                .next() else {
                return TagID::NULL
            };

            iterator.id()
        },
        ExeType::Cache => get_cache_file_tags()
            .iter()
            .find(|f| f.get_primary_tag_group() == group && f.get_tag_path() == path)
            .map(|t| t.tag_id)
            .unwrap_or(TagID::NULL)
    }
}

#[repr(u32)]
pub enum TagGroup {
    Something
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct TagGroupUnsafe(u32);
