use core::ffi::{c_char, CStr};
use core::fmt::{Debug, Formatter};
use c_mine::c_mine;
use crate::id::ID;
use crate::init::{get_exe_type, ExeType};
use crate::table::DataTable;
use crate::util::VariableProvider;

pub const TAG_ID_SALT: u16 = 0x6174;
pub type TagID = ID<TAG_ID_SALT>;

pub const TAGS_TAG_INSTANCES: VariableProvider<Option<&mut DataTable<TagTagInstance, TAG_ID_SALT>>> = VariableProvider {
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

pub const CACHE_FILE_TAG_HEADER: VariableProvider<Option<&mut CacheFileTagHeader>> = VariableProvider {
    name: "CACHE_FILE_TAG_HEADER",
    cache_address: 0x00AF8B70 as *mut _,
    tags_address: 0x00000000 as *mut _
};

/// These methods are unsafe as we cannot guarantee yet that the tag data is not being accessed
/// concurrently.
#[repr(C)]
pub struct Reflexive<T: Sized + 'static> {
    count: usize,
    objects: *mut T,
    unknown: u32
}
impl<T: Sized + 'static> Reflexive<T> {
    pub const fn len(&self) -> usize {
        self.count
    }
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub unsafe fn as_slice(&self) -> &[T] {
        if self.is_empty() {
            return &[]
        }
        if self.objects.is_null() {
            panic!("as_slice() -> Bad reflexive: {self:?} @ 0x{:08X}", (self as *const _) as usize);
        }
        core::slice::from_raw_parts(self.objects, self.count)
    }
    pub unsafe fn as_mut_slice(&self) -> &mut [T] {
        if self.is_empty() {
            return &mut []
        }

        if self.objects.is_null() {
            panic!("as_mut_slice() -> Bad reflexive: {self:?} @ 0x{:08X}", (self as *const _) as usize);
        }
        core::slice::from_raw_parts_mut(self.objects, self.count)
    }
    pub unsafe fn get(&self, index: usize) -> Option<&T> {
        self.as_slice().get(index)
    }
    pub unsafe fn get_mut(&self, index: usize) -> Option<&mut T> {
        self.as_mut_slice().get_mut(index)
    }
}
impl<T: Sized> Debug for Reflexive<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(
            format_args!(
                "Reflexive<{type_name}> {{ count={count}, objects=0x{objects:08X} }}",
                type_name=core::any::type_name::<T>(),
                count=self.count,
                objects=self.objects as usize
            ))
    }
}

/// These methods are unsafe as we cannot guarantee yet that the tag data is not being accessed
/// concurrently.
#[derive(Debug)]
#[repr(C)]
pub struct TagData {
    size: usize,
    flags: u32,
    file_offset: u32,
    data: *mut u8,
    unknown: u32,
}
impl TagData {
    pub const fn len(&self) -> usize {
        self.size
    }
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub unsafe fn as_slice(&self) -> &[u8] {
        if self.is_empty() {
            return &[]
        }
        if self.data.is_null() {
            panic!("as_slice() -> Bad data: {self:?} @ 0x{:08X}", (self as *const _) as usize);
        }
        core::slice::from_raw_parts(self.data, self.size)
    }
    pub unsafe fn as_mut_slice(&self) -> &mut [u8] {
        if self.is_empty() {
            return &mut []
        }
        if self.data.is_null() {
            panic!("as_mut_slice() -> Bad data: {self:?} @ 0x{:08X}", (self as *const _) as usize);
        }
        core::slice::from_raw_parts_mut(self.data, self.size)
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
    fn get_tag_data(&self) -> *mut [u8; 0];
}

/// Used only in tag builds.
#[repr(C)]
pub struct CacheTagInstance {
    pub primary_tag_group: TagGroupUnsafe,
    pub secondary_tag_group: TagGroupUnsafe,
    pub tertiary_tag_group: TagGroupUnsafe,
    pub tag_id: TagID,
    pub tag_path: *const c_char,
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

    unsafe fn get_tag_path(&self) -> &str {
        assert!(!self.tag_path.is_null(), "Tag path is null!");
        CStr::from_ptr(self.tag_path).to_str().expect("Tag path is not UTF-8!")
    }

    fn get_tag_data(&self) -> *mut [u8; 0] {
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

    fn get_tag_data(&self) -> *mut [u8; 0] {
        self.tag_data
    }
}

#[c_mine]
pub unsafe extern "C" fn resolve_tag_loaded(group: TagGroupUnsafe, path: *const c_char) -> TagID {
    let path = CStr::from_ptr(path).to_str().expect("input tag is not UTF-8");
    match get_exe_type() {
        ExeType::Tag => {
            let Some(table) = TAGS_TAG_INSTANCES.get_mut() else {
                panic!("TAGS_TAG_INSTANCES is null!");
            };

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
pub struct TagGroupUnsafe(pub u32);

#[derive(Copy, Clone)]
pub enum GetTagDataError {
    NoMatch { id: TagID },
    BadTagGroup { id: TagID, fourcc: TagGroupUnsafe, expected: [TagGroupUnsafe; 4] }
}

impl Debug for GetTagDataError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            GetTagDataError::NoMatch { id } => f.write_fmt(format_args!("Cannot find tag with ID {id:?}")),
            GetTagDataError::BadTagGroup { id, fourcc: group, expected } => f.write_fmt(format_args!("Found a tag with {id:?}, but the tag group is incorrect {group:?} not in {expected:?}")),
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
            let Some(table) = TAGS_TAG_INSTANCES.get_mut() else {
                panic!("TAGS_TAG_INSTANCES is null!");
            };
            let tag = table.get_element(id).ok()?;
            Some(&tag.item)
        }
    }

}

/// Gets the tag data.
pub unsafe fn get_tag_data_checking_tag_group(group: TagGroupUnsafe, id: TagID) -> Result<*mut [u8; 0], GetTagDataError> {
    let tag = get_tag_info(id).ok_or(GetTagDataError::NoMatch { id })?;

    let expected = [
        tag.get_primary_tag_group(),
        tag.get_secondary_tag_group(),
        tag.get_tertiary_tag_group(),
        TagGroupUnsafe(u32::MAX),
    ];

    if !expected.contains(&group) {
        return Err(GetTagDataError::BadTagGroup { id, fourcc: group, expected });
    }

    Ok(tag.get_tag_data())
}

#[c_mine]
pub unsafe extern "C" fn tag_get(group: TagGroupUnsafe, id: TagID) -> *mut [u8; 0] {
    get_tag_data_checking_tag_group(group, id).expect("tag_get failed!")
}

#[c_mine]
pub unsafe extern "C" fn tag_block_get_address(reflexive: Option<&Reflexive<[u8; 0]>>) -> *mut [u8; 0] {
    reflexive.expect("tag_block_get_address with null reflexive").objects
}

#[c_mine]
pub unsafe extern "C" fn tag_block_get_element_with_size(
    reflexive: Option<&Reflexive<[u8; 0]>>,
    index: usize,
    element_size: usize
) -> *mut [u8; 0] {
    let reflexive = reflexive.expect("tag_block_get_element_with_size with null reflexive");
    assert!(
        index < reflexive.len(),
        "tag_block_get_element_with_size with out-of-bounds index {index} < {} @ 0x{:08X}",
        reflexive.len(),
        (reflexive as *const _) as usize
    );

    let offset = index.checked_mul(element_size)
        .and_then(|v| isize::try_from(v).ok())
        .expect("tag_block_get_element_with_size with invalid offset/element size");

    reflexive
        .objects
        .wrapping_byte_offset(offset)
}
