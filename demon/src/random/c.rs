use super::lcg::*;
use c_mine::c_mine;

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
