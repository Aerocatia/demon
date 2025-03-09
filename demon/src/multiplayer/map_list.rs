#![allow(static_mut_refs)]

pub mod c;

use alloc::{format, vec};
use alloc::vec::Vec;
use core::ffi::{c_char, CStr};
use core::mem::transmute;
use spin::Mutex;
use tag_structs::{CacheFileHeader, ScenarioType};
use crate::file::{read_partial_data_from_file, Path};
use crate::init::{get_exe_type, ExeType};
use crate::map::verify_map_header;
use crate::util::{CStrPtr, VariableProvider};

pub static mut ALL_MP_MAPS: Vec<MultiplayerMapListEntry> = Vec::new();
pub static ALL_MP_MAP_NAMES: Mutex<Vec<Vec<u8>>> = Mutex::new(Vec::new());

const MP_MAP_ARRAY: VariableProvider<*mut MultiplayerMapListEntry> = variable! {
    name: "mp_map_array",
    cache_address: 0x00C838E4,
    tag_address: 0x00D3AEA4
};

const MP_MAP_COUNT: VariableProvider<usize> = variable! {
    name: "mp_map_count",
    cache_address: 0x00C838E8,
    tag_address: 0x00D3AE9C
};

#[repr(C)]
pub struct MultiplayerMapListEntry {
    name: CStrPtr,
    index: u32,
    valid: bool,
    crc_verified: bool,
    _padding: [u8; 2],
    crc32: u32
}

impl MultiplayerMapListEntry {
    pub fn new(name: &str, index: u32) -> Self {
        assert!(!name.contains('\x00'), "name cannot contain null bytes");

        let mut maps = ALL_MP_MAP_NAMES.lock();
        let mut storage = vec![0u8; name.len() + 1];
        storage[..name.len()].copy_from_slice(name.as_bytes());

        let ptr = storage.as_ptr();
        maps.push(storage);

        Self {
            // SAFETY: This pointer should not move after pushing the storage buffer.
            name: CStrPtr::from_cstr(unsafe { CStr::from_ptr(ptr as *const c_char) }),
            index,
            crc_verified: false,
            valid: false,
            _padding: [0u8; 2],
            crc32: u32::MAX
        }
    }
}

impl Drop for MultiplayerMapListEntry {
    fn drop(&mut self) {
        let mut maps = ALL_MP_MAP_NAMES.lock();
        let ptr = self.name.as_byte_ptr();
        maps.retain(|f| f.as_ptr() != ptr);
        self.name = CStrPtr::from_cstr(c"<removed>");
    }
}

pub unsafe fn all_mp_maps() -> &'static [MultiplayerMapListEntry] {
    ALL_MP_MAPS.as_slice()
}

/// SAFETY: Only safe if on one thread.
pub unsafe fn add_mp_map(name: &str, map_index: Option<u32>) -> bool {
    for i in ALL_MP_MAPS.iter() {
        if i.name.to_str_lossless() == Some(name) {
            return false
        }
    }

    match get_exe_type() {
        ExeType::Cache => {
            let header = match header_from_cache(name) {
                Ok(n) => n,
                Err(e) => {
                    error!("Failed to load map {name}: {e}");
                    debug_log!("Failed to load map {name}: {e}");
                    return false
                }
            };
            if header.map_type.try_get() != Ok(ScenarioType::Multiplayer) {
                return false
            }
        }
        ExeType::Tag => {
            let mut tag_prefix = [0u8; 0x100];
            let Some(n) = read_partial_data_from_file(&Path::from(format!("tags\\{name}.scenario")), &mut tag_prefix, 0) else {
                return false
            };
            if n.len() != 0x100 {
                return false
            }
            if u16::from_be_bytes([n[0x7C], n[0x7D]]) != ScenarioType::Multiplayer as u16 {
                return false
            }
        }
    }

    let mut entry = MultiplayerMapListEntry::new(name, map_index.unwrap_or(0x13));
    entry.valid = true;

    ALL_MP_MAPS.push(entry);
    *MP_MAP_ARRAY.get_mut() = ALL_MP_MAPS.as_mut_ptr();
    *MP_MAP_COUNT.get_mut() = ALL_MP_MAPS.len();

    true
}

fn header_from_cache(name: &str) -> Result<CacheFileHeader, &'static str> {
    match get_exe_type() {
        ExeType::Cache => {
            let mut cache_header = [0u8; 0x800];
            let Some(n) = read_partial_data_from_file(&Path::from(format!("maps\\{name}.map")), &mut cache_header, 0) else {
                return Err("cannot open map file");
            };
            if n.len() != cache_header.len() {
                return Err("cannot read entire header");
            }
            let header: CacheFileHeader = unsafe { transmute(cache_header) };
            verify_map_header(name, &header).map(|_| header)
        }
        _ => unreachable!("not a cache build")
    }
}

unsafe fn get_mp_map_data_by_name(name: &str) -> Option<&'static mut MultiplayerMapListEntry> {
    for i in &mut ALL_MP_MAPS {
        if i.name.expect_str() == name {
            return Some(i)
        }
    }
    None
}

unsafe fn get_mp_map_data_by_index(index: usize) -> Option<&'static mut MultiplayerMapListEntry> {
    ALL_MP_MAPS.get_mut(index)
}
