use core::ffi::{c_char, CStr};
use core::sync::atomic::{AtomicUsize, Ordering};
use c_mine::c_mine;
use crate::util::{PointerProvider, VariableProvider};

pub mod table;

pub const GAME_STATE_CPU_SIZE: usize = 0x440000;
pub static CPU_ALLOCATION_SIZE: AtomicUsize = AtomicUsize::new(0);

pub const GAME_STATE_GLOBALS_LOCKED: VariableProvider<u8> = variable! {
    name: "game_state_globals.locked",
    cache_address: 0x00F14610,
    tags_address: 0x00FCBBD8
};

pub const ALLOCATION_ADDRESS: VariableProvider<*mut [u8; 0]> = variable! {
    name: "ALLOCATION_ADDRESS",
    cache_address: 0x00F14600,
    tags_address: 0x00FCBBC8
};

pub const GAME_STATE_CRC: VariableProvider<u32> = variable! {
    name: "GAME_STATE_CRC",
    cache_address: 0x00F1460C,
    tags_address: 0x00FCBBD4
};

pub const CRC_CHECKSUM_BUFFER: PointerProvider<extern "C" fn(&mut u32, &mut usize, usize)> = pointer! {
    name: "crc_checksum_buffer",
    cache_address: 0x00408567,
    tags_address: 0x0040779D
};

fn allocate_into_game_state<R: FnOnce() -> &'static str>(name_resolver: R, size: usize) -> *mut [u8; 0] {
    if unsafe { *GAME_STATE_GLOBALS_LOCKED.get() } == 1 {
        let name = name_resolver();
        panic!("Unable to allocate {name}: Game state globals locked; cannot allocate into the game state anymore!")
    }
    if size > GAME_STATE_CPU_SIZE {
        let name = name_resolver();
        panic!("Unable to allocate {name}: Cannot allocate {size} bytes (too big)")
    }

    let offset = CPU_ALLOCATION_SIZE.fetch_add(size, Ordering::Relaxed);
    if size + offset > GAME_STATE_CPU_SIZE {
        let name = name_resolver();
        panic!("Unable to allocate {name}: Cannot allocate {size} bytes; only {} bytes remaining", GAME_STATE_CPU_SIZE.saturating_sub(offset));
    }

    unsafe { *ALLOCATION_ADDRESS.get() }.wrapping_byte_add(offset)
}

#[c_mine]
pub unsafe extern "C" fn game_state_malloc(
    name: *const c_char,
    _data_type: *const c_char,
    size: usize
) -> *mut [u8; 0] {
    let resolve_name = || {
        if name.is_null() {
            return "(null)"
        };
        CStr::from_ptr(name).to_str().expect("name passed was not UTF-8???")
    };

    let data = allocate_into_game_state(resolve_name, size);
    let mut size = size;
    CRC_CHECKSUM_BUFFER.get()(GAME_STATE_CRC.get_mut(), &mut size, 4);
    data
}
