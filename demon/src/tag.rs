use core::ffi::CStr;
use core::fmt::{Debug, Formatter};
use tag_structs::primitives::NamedTagStruct;
use tag_structs::primitives::tag_group::{TagGroup, TagGroupStruct, TagGroupUnsafe};
use tag_structs::Scenario;
use crate::id::ID;
use crate::init::{get_exe_type, ExeType};
use crate::memory::table::DataTable;
use crate::util::{CStrPtr, VariableProvider};

pub mod c;

pub const TAG_ID_SALT: u16 = 0x6174;
pub type TagID = ID<TAG_ID_SALT>;
impl From<tag_structs::primitives::data::TagID> for TagID {
    fn from(value: tag_structs::primitives::data::TagID) -> Self {
        Self::from_full_id(value.0)
    }
}


pub const GLOBAL_SCENARIO: VariableProvider<Option<&mut Scenario>> = variable! {
    name: "global_scenario",
    cache_address: 0x00F1A67C,
    tag_address: 0x00FD1C44
};

pub const GLOBAL_SCENARIO_INDEX: VariableProvider<TagID> = variable! {
    name: "global_scenario_index",
    cache_address: 0x00A39C64,
    tag_address: 0x00AE1174
};

pub const TAGS_TAG_INSTANCES: VariableProvider<Option<&mut DataTable<TagTagInstance, TAG_ID_SALT>>> = variable! {
    name: "TAGS_TAG_INSTANCES",
    tag_address: 0x00FFDAF8
};

pub const CACHE_TAG_INSTANCES: VariableProvider<*mut CacheTagInstance> = variable! {
    name: "CACHE_TAG_INSTANCES",
    cache_address: 0x00AF8364
};

pub const CACHE_TAGS_ARE_LOADED: VariableProvider<u8> = variable! {
    name: "CACHE_TAGS_ARE_LOADED",
    cache_address: 0x00AF8368
};

pub const CACHE_FILE_TAG_HEADER: VariableProvider<Option<&mut CacheFileTagHeader>> = variable! {
    name: "CACHE_FILE_TAG_HEADER",
    cache_address: 0x00AF8B70
};

#[derive(Copy, Clone, Debug)]
pub struct UnknownType;
impl NamedTagStruct for UnknownType {
    fn name() -> &'static str {
        "(unknown)"
    }
}

/// These methods are unsafe as we cannot guarantee yet that the tag data is not being accessed
/// concurrently.
pub unsafe trait ReflexiveImpl<T: Sized + 'static>: Copy + Clone {
    fn len(self) -> usize;
    fn is_empty(self) -> bool;
    unsafe fn as_slice(self) -> &'static [T];
    unsafe fn as_mut_slice(self) -> &'static mut [T];
    unsafe fn get(self, index: usize) -> Option<&'static T>;
    unsafe fn get_mut(self, index: usize) -> Option<&'static mut T>;
}

unsafe impl<T: Sized + Copy + Clone + Debug + 'static + NamedTagStruct> ReflexiveImpl<T> for tag_structs::primitives::data::Reflexive<T> {
    fn len(self) -> usize {
        self.count as usize
    }

    fn is_empty(self) -> bool {
        self.count == 0
    }

    unsafe fn as_slice(self) -> &'static [T] {
        if self.is_empty() {
            return &[]
        }
        let address = self.address.0 as *const T;
        if address.is_null() {
            panic!("as_slice() -> Bad reflexive {}: {self:?}", T::name());
        }
        core::slice::from_raw_parts(address, self.len())
    }

    unsafe fn as_mut_slice(self) -> &'static mut [T] {
        if self.is_empty() {
            return &mut []
        }
        let address = self.address.0 as *mut T;
        if address.is_null() {
            panic!("as_slice() -> Bad reflexive {}: {self:?}", T::name());
        }
        core::slice::from_raw_parts_mut(address, self.len())
    }

    unsafe fn get(self, index: usize) -> Option<&'static T> {
        self.as_slice().get(index)
    }

    unsafe fn get_mut(self, index: usize) -> Option<&'static mut T> {
        self.as_mut_slice().get_mut(index)
    }
}

/// These methods are unsafe as we cannot guarantee yet that the tag data is not being accessed
/// concurrently.
pub unsafe trait TagData: Copy + Clone {
    fn len(self) -> usize;
    fn is_empty(self) -> bool;
    unsafe fn as_slice(self) -> &'static [u8];
    unsafe fn as_mut_slice(self) -> &'static mut [u8];
}

unsafe impl TagData for tag_structs::primitives::data::Data {
    fn len(self) -> usize {
        self.size as usize
    }
    fn is_empty(self) -> bool {
        self.len() == 0
    }
    unsafe fn as_slice(self) -> &'static [u8] {
        if self.is_empty() {
            return &[]
        }
        let data = (self.data.0 as usize) as *const u8;
        if data.is_null() {
            panic!("as_slice() -> Bad data (null): {self:?}");
        }
        core::slice::from_raw_parts(data, self.len())
    }
    unsafe fn as_mut_slice(self) -> &'static mut [u8] {
        if self.is_empty() {
            return &mut []
        }
        let data = (self.data.0 as usize) as *mut u8;
        if data.is_null() {
            panic!("as_slice() -> Bad data (null): {self:?}");
        }
        core::slice::from_raw_parts_mut(data, self.len())
    }
}

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
        let Some(cache_header) = CACHE_FILE_TAG_HEADER.get() else {
            panic!("CACHE_FILE_TAG_HEADER is null!")
        };
        let tags = *CACHE_TAG_INSTANCES.get();
        assert!(!tags.is_null(), "CACHE_tag_address is null!");
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
    fn get_tag_id(&self) -> TagID;

    /// Returns Ok(()) if any of the tag's groups correspond to `tag_group`.
    fn verify_tag_group(&self, tag_group: TagGroupUnsafe) -> Result<(), GetTagDataError> {
        let expected = [
            self.get_primary_tag_group(),
            self.get_secondary_tag_group(),
            self.get_tertiary_tag_group(),
            TagGroup::Null.into()
        ];

        expected
            .contains(&tag_group)
            .then_some(())
            .ok_or_else(|| GetTagDataError::BadTagGroup { id: self.get_tag_id(), tag_group, expected })
    }

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
    fn get_tag_data_address(&self) -> *mut [u8; 0];
}

/// Used only in cache builds.
#[repr(C)]
pub struct CacheTagInstance {
    pub primary_tag_group: TagGroupUnsafe,
    pub secondary_tag_group: TagGroupUnsafe,
    pub tertiary_tag_group: TagGroupUnsafe,
    pub tag_id: TagID,
    pub tag_path: CStrPtr,
    pub tag_data: *mut [u8; 0],
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

    fn get_tag_id(&self) -> TagID {
        self.tag_id
    }

    unsafe fn get_tag_path(&self) -> &str {
        self.tag_path.expect_str()
    }

    fn get_tag_data_address(&self) -> *mut [u8; 0] {
        self.tag_data
    }
}

/// Used only in tag builds.
#[repr(C)]
pub struct TagTagInstance {
    pub identifier: u16,
    pub _unknown: u16,
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
    pub tag_data: *mut [u8; 0],
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

    fn get_tag_data_address(&self) -> *mut [u8; 0] {
        self.tag_data
    }

    fn get_tag_id(&self) -> TagID {
        // SAFETY: This is safe because the tag path is self-contained in the struct.
        let path = unsafe { self.get_tag_path() };
        let group = self.primary_tag_group;

        // SAFETY: This is safe provided there are no data races. Therefore, it's not safe. Hopefully we don't blow up!
        let Some(t) = (unsafe { lookup_tag(path, self.primary_tag_group) }) else {
            panic!("Calling TagTagInstance::get_tag_id, but can't get the tag ID ({path}.{group:?})", )
        };

        t.1
    }
}

/// Look up a tag, returning a reference to it and its ID.
///
/// # Safety
///
/// No guarantee is made that there are no data races.
pub unsafe fn lookup_tag(path: &str, group: TagGroupUnsafe) -> Option<(&dyn TagIndex, TagID)> {
    match get_exe_type() {
        ExeType::Tag => {
            let Some(table) = TAGS_TAG_INSTANCES.get_copied() else {
                panic!("TAGS_TAG_INSTANCES is null!");
            };

            let mut iterator = table.iter();
            let Some(tag) = (&mut iterator)
                .filter(|tag| tag.get().get_primary_tag_group() == group && tag.get().get_tag_path() == path)
                .next() else {
                return None
            };

            Some((tag.get(), iterator.id()))
        },
        ExeType::Cache => get_cache_file_tags()
            .iter()
            .find(|f| f.get_primary_tag_group() == group && f.get_tag_path() == path)
            .map(|f| {
                let tag_id = f.tag_id;
                (f as &dyn TagIndex, tag_id)
            })
    }
}

#[derive(Copy, Clone)]
pub enum GetTagDataError {
    NoMatch { id: TagID },
    BadTagGroup { id: TagID, tag_group: TagGroupUnsafe, expected: [TagGroupUnsafe; 4] }
}

impl Debug for GetTagDataError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            GetTagDataError::NoMatch { id } => f.write_fmt(format_args!("Cannot find tag with ID {id:?}")),
            GetTagDataError::BadTagGroup { id, tag_group, expected } => f.write_fmt(format_args!("Found a tag with {id:?}, but the tag group is incorrect: '{tag_group:?}' not in {expected:?}")),
        }
    }
}

pub unsafe fn get_tag_info(id: TagID) -> Option<&'static dyn TagIndex> {
    match get_exe_type() {
        ExeType::Cache => {
            let index = id.index()?;
            let tags = get_cache_file_tags();
            let result = tags.get(index)?;
            Some(result)
        },
        ExeType::Tag => {
            let Some(table) = TAGS_TAG_INSTANCES.get_copied() else {
                panic!("TAGS_TAG_INSTANCES is null!");
            };
            let tag = table.get_element(id).ok()?;
            Some(tag.get())
        }
    }
}

/// Gets the tag info as well as the data (typed).
pub unsafe fn get_tag_info_typed<T: TagGroupStruct>(id: TagID) -> Result<(&'static dyn TagIndex, &'static mut T), GetTagDataError> {
    let info = get_tag_info(id).ok_or(GetTagDataError::NoMatch { id })?;
    Ok((info, get_tag_data_from_info(info)?))
}

pub unsafe fn get_tag_data_from_info<T: TagGroupStruct>(info: &dyn TagIndex) -> Result<&'static mut T, GetTagDataError> {
    info.verify_tag_group(T::get_tag_group().into())?;
    Ok(&mut *(info.get_tag_data_address() as *mut T))
}
