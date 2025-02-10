//! Linear Congruential Generator PRNG
//!
//! These types generate pseudorandom numbers using a LCG (linear congruent generator).
//!
//! **NOTE:** These values are 110% NOT cryptographically secure. This is intended for quickly
//! generating seemingly unpredictable, random game events. Use a CSPRNG if you want secure
//! pseudorandomness.

use core::sync::atomic::{AtomicU32, Ordering};
use crate::game_engine::GAME_ENGINE_RUNNING;

static mut GLOBAL_RANDOM_SEED: u32 = 0;
static RANDOM_SEED_LOCK_COUNT: AtomicU32 = AtomicU32::new(0);

pub fn seed_next(seed: &mut u32) -> u32 {
    *seed = *seed * 0x19660D + 0x3C6EF35F;
    *seed
}

/// Gets the global random seed.
///
/// # Safety
///
/// This is not thread-safe!
#[allow(static_mut_refs)]
pub unsafe fn get_global_random_seed() -> &'static mut u32 {
    // this check does not actually protect the seed, as this reference can be stored
    let locks = RANDOM_SEED_LOCK_COUNT.load(Ordering::Relaxed);
    if GAME_ENGINE_RUNNING.get()() && locks > 0 {
        panic!("Using get_global_random_seed() when locked is not allowed ({locks} locks)");
    }
    &mut GLOBAL_RANDOM_SEED
}

pub fn lock_global_random_seed() {
    let q = RANDOM_SEED_LOCK_COUNT.fetch_add(1, Ordering::Relaxed);
    assert_ne!(q, u32::MAX, "RANDOM_SEED_LOCK_COUNT overflowed!");
}

pub fn unlock_global_random_seed() {
    let q = RANDOM_SEED_LOCK_COUNT.fetch_sub(1, Ordering::Relaxed);
    assert_ne!(q, 0, "RANDOM_SEED_LOCK_COUNT underflowed!");
}


/// Linear Congruential Generator PRNG that generates two values between a range.
pub trait LCGRandomRange: Sized {
    /// Generate a pseudorandom number in a range between min and max.
    ///
    /// For integer values, max is exclusive. For floating point values, max is inclusive.
    fn lcg_random_range(seed: &mut u32, min: Self, max: Self) -> Self;

    /// Generate a pseudorandom number in a range between min and max.
    ///
    /// For integer values, max is exclusive. For floating point values, max is inclusive.
    ///
    /// # Safety
    ///
    /// The global seed is not (yet) thread-safe.
    unsafe fn lcg_global_random_range(min: Self, max: Self) -> Self {
        Self::lcg_random_range(get_global_random_seed(), min, max)
    }
}

impl LCGRandomRange for i16 {
    fn lcg_random_range(seed: &mut u32, min: Self, max: Self) -> Self {
        let seed = (seed_next(seed) >> 16) as i32;
        let range = (max - min) as i32;
        let range_random = (range.wrapping_mul(seed) >> 16) as i16;
        min.wrapping_add(range_random)
    }
}

impl LCGRandomRange for u16 {
    fn lcg_random_range(seed: &mut u32, min: Self, max: Self) -> Self {
        let (min, max) = if min > max { (max, min) } else { (min, max) };
        let seed = seed_next(seed) >> 16;
        let range = (max - min) as u32;
        let range_random = (range.wrapping_mul(seed) >> 16) as u16;
        min.wrapping_add(range_random)
    }
}

impl LCGRandomRange for f32 {
    fn lcg_random_range(seed: &mut u32, min: Self, max: Self) -> Self {
        let seed = (seed_next(seed) >> 16) as f32;
        let range = max - min;
        let range_random = (range * seed) * (1.0 / 65535.0);
        min + range_random
    }
}

/// Linear Congruential Generator PRNG that generates a value between 0 and 1 (inclusive).
pub trait LCGRandomZeroToOne: Sized {
    fn lcg_random_zero_to_one(seed: &mut u32) -> Self;
    unsafe fn lcg_global_random_zero_to_one() -> Self {
        Self::lcg_random_zero_to_one(get_global_random_seed())
    }
}

impl LCGRandomZeroToOne for f32 {
    fn lcg_random_zero_to_one(seed: &mut u32) -> Self {
        Self::lcg_random_range(seed, 0.0, 1.0)
    }
}

impl LCGRandomZeroToOne for bool {
    fn lcg_random_zero_to_one(seed: &mut u32) -> Self {
        ((seed_next(seed) >> 16) & 1) == 1
    }
}

/// Linear Congruential Generator PRNG that generates any value.
pub trait LCGRandom: Sized {
    fn lcg_random(seed: &mut u32) -> Self;
    unsafe fn lcg_global_random() -> Self {
        Self::lcg_random(get_global_random_seed())
    }
}

impl LCGRandom for u16 {
    fn lcg_random(seed: &mut u32) -> Self {
        (seed_next(seed) >> 16) as u16
    }
}

impl LCGRandom for u32 {
    fn lcg_random(seed: &mut u32) -> Self {
        let low = u16::lcg_random(seed) as u32;
        let high = u16::lcg_random(seed) as u32;
        low | (high << 16)
    }
}

impl LCGRandom for i16 {
    fn lcg_random(seed: &mut u32) -> Self {
        (seed_next(seed) >> 16) as i16
    }
}
