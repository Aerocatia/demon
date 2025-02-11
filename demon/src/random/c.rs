use windows_sys::Win32::System::SystemInformation::GetTickCount;
use super::lcg::*;
use c_mine::c_mine;
use tag_structs::primitives::vector::Vector3D;
use crate::random::random_directions::RANDOM_DIRECTIONS;
use crate::timing::PerformanceCounter;

#[c_mine]
pub unsafe extern "C" fn get_global_random_seed_address() -> &'static mut u32 {
    get_global_random_seed()
}

#[c_mine]
pub extern "C" fn lock_global_random_seed() {
    super::lcg::lock_global_random_seed();
}

#[c_mine]
pub extern "C" fn unlock_global_random_seed() {
    super::lcg::unlock_global_random_seed();
}

#[c_mine]
pub extern "C" fn seed_random_range(seed: &mut u32, low: i16, high: i16) -> i16 {
    i16::lcg_random_range(seed, low, high)
}

#[c_mine]
pub unsafe extern "C" fn random_range(low: i16, high: i16) -> i16 {
    i16::lcg_global_random_range(low, high)
}

#[c_mine]
pub extern "C" fn real_seed_random_range(seed: &mut u32, low: f32, high: f32) -> f32 {
    f32::lcg_random_range(seed, low, high)
}

#[c_mine]
pub unsafe extern "C" fn real_random_range(low: f32, high: f32) -> f32 {
    f32::lcg_global_random_range(low, high)
}

#[c_mine]
pub extern "C" fn real_seed_random(seed: &mut u32) -> f32 {
    f32::lcg_random_zero_to_one(seed)
}

#[c_mine]
pub unsafe extern "C" fn real_random() -> f32 {
    f32::lcg_global_random_zero_to_one()
}

#[c_mine]
pub extern "C" fn seed_random_direction3d(seed: &mut u32, to: &mut Vector3D) {
    const MAX: usize = RANDOM_DIRECTIONS.len();
    const _: () = assert!(MAX <= u16::MAX as usize);
    let direction = u16::lcg_random_range(seed, 0, MAX as u16) as usize;
    *to = RANDOM_DIRECTIONS[direction];
}

#[c_mine]
pub unsafe extern "C" fn get_global_local_random_seed_address() -> &'static mut u32 {
    get_local_random_seed()
}

#[c_mine]
pub unsafe extern "C" fn random_math_initialize() {
    // Should provide a fairly unpredictable starting value for use in-game.
    let counter = PerformanceCounter::now().counter;
    let entropy = (((counter >> 32) ^ (counter)) as u32) ^ GetTickCount();
    let rng = (entropy >> 16) | (entropy << 16);
    *get_local_random_seed() = rng;
}
