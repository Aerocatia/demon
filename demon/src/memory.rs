use core::sync::atomic::{AtomicUsize, Ordering};
use c_mine::c_mine;
use crate::crc32::CRC32;
use crate::util::{CStrPtr, VariableProvider};

pub mod table;

pub const GAME_STATE_CPU_SIZE: usize = 0x440000;
pub static CPU_ALLOCATION_SIZE: AtomicUsize = AtomicUsize::new(0);

pub const GAME_STATE_GLOBALS_LOCKED: VariableProvider<u8> = variable! {
    name: "game_state_globals.locked",
    cache_address: 0x00F14610,
    tag_address: 0x00FCBBD8
};

pub const ALLOCATION_ADDRESS: VariableProvider<*mut u8> = variable! {
    name: "ALLOCATION_ADDRESS",
    cache_address: 0x00F14600,
    tag_address: 0x00FCBBC8
};

pub const GAME_STATE_CRC: VariableProvider<CRC32> = variable! {
    name: "GAME_STATE_CRC",
    cache_address: 0x00F1460C,
    tag_address: 0x00FCBBD4
};

pub fn update_game_state_crc(data: &[u8]) {
    unsafe { GAME_STATE_CRC.get_mut() }.update(data);
}

unsafe fn allocate_into_game_state(name: CStrPtr, size: usize) -> *mut u8 {
    let name = name.display_lossy();
    if *GAME_STATE_GLOBALS_LOCKED.get() == 1 {
        panic!("Unable to allocate {name}: Game state globals locked; cannot allocate into the game state anymore!")
    }
    if size > GAME_STATE_CPU_SIZE {
        panic!("Unable to allocate {name}: Cannot allocate {size} bytes (too big)")
    }

    let offset = CPU_ALLOCATION_SIZE.fetch_add(size, Ordering::Relaxed);
    if size + offset > GAME_STATE_CPU_SIZE {
        panic!("Unable to allocate {name}: Cannot allocate {size} bytes; only {} bytes remaining", GAME_STATE_CPU_SIZE.saturating_sub(offset));
    }
    ALLOCATION_ADDRESS.get().wrapping_byte_add(offset)
}

#[c_mine]
pub unsafe extern "C" fn game_state_malloc(
    name: CStrPtr,
    _data_type: *const u8,
    size: usize
) -> *mut u8 {
    update_game_state_crc(&size.to_ne_bytes());
    allocate_into_game_state(name, size)
}

#[c_mine]
pub extern "C" fn game_in_editor() -> bool {
    false
}

#[c_mine]
pub unsafe extern "C" fn crc_checksum_buffer(crc: &mut CRC32, data: *const u8, length: usize) {
    let data = core::slice::from_raw_parts(data, length);
    crc.update(data);
}
