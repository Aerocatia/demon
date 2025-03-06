pub mod c;

use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use windows_sys::Win32::Foundation::TRUE;
use windows_sys::Win32::System::Performance::{QueryPerformanceCounter, QueryPerformanceFrequency};
use crate::util::VariableProvider;

/// The base tick rate of the game's engine (before `game_speed`).
///
/// # Regret
///
/// Changing this value leads to regret.
pub const TICK_RATE: f32 = 30.0;

#[derive(Default, Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct PerformanceCounter {
    pub counter: u64
}
impl PerformanceCounter {
    pub fn now() -> Self {
        let mut counter = 0i64;

        // SAFETY: Points to a correct i64
        let success = unsafe { QueryPerformanceCounter(&mut counter) };
        assert_eq!(success, TRUE, "QueryPerformanceCounter error!");

        Self { counter: u64::try_from(counter).expect("QueryPerformanceCounter returned a negative counter!") }
    }

    /// # Panics
    ///
    /// Panics if `self.counter` > `i64::MAX as u64` or `when.counter` > `i64::MAX as u64`
    pub fn time_since(self, when: PerformanceCounter) -> PerformanceCounterDelta {
        let Ok(self_counter) = i64::try_from(self.counter) else {
            panic!("self.counter overflows an i64");
        };
        let Ok(when_counter) = i64::try_from(when.counter) else {
            panic!("when.counter overflows an i64");
        };

        let counter = self_counter - when_counter;
        PerformanceCounterDelta { counter: counter.unsigned_abs(), backwards: counter.is_negative() }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub struct PerformanceCounterDelta {
    pub backwards: bool,
    pub counter: u64
}
impl PerformanceCounterDelta {
    pub fn seconds(self) -> u64 {
        let frequency = Self::get_frequency();
        (self.counter) / (frequency)
    }
    pub fn milliseconds(self) -> u64 {
        let frequency = Self::get_frequency();
        (self.counter * 1000) / (frequency)
    }
    pub fn microseconds(self) -> u64 {
        let frequency = Self::get_frequency();
        (self.counter * 1000000) / (frequency)
    }
    pub fn seconds_f64(self) -> f64 {
        let frequency = Self::get_frequency();
        (self.counter as f64) / (frequency as f64)
    }
    pub fn from_seconds_f64(sec: f64) -> Self {
        let frequency = Self::get_frequency();
        Self { counter: ((frequency as f64) * sec.abs()) as u64, backwards: sec < 0.0 }
    }
    fn get_frequency() -> u64 {
        let mut frequency = 0i64;

        // SAFETY: Points to a correct i64
        let success = unsafe { QueryPerformanceFrequency(&mut frequency) };
        assert_eq!(success, TRUE, "QueryPerformanceFrequency error!");
        assert!(frequency > 0, "QueryPerformanceFrequency returned a non-positive frequency! {frequency}");
        frequency as u64
    }
}

/// Primitive for un-tying things from frame rate.
///
/// This is a band-aid solution that will break if frame time is higher than half of `delay`. Things
/// that use this should eventually do ONE of the following:
/// - Move to the tick loop (i.e. the 30 Hz loop that's also configurable with game_speed)
/// - Be properly interpolated by a fixed rate (e.g. `InterpolatedTimer`)
pub struct FixedTimer {
    last_update: AtomicU64,
    guard: AtomicBool,
    max_ticks_behind: u16,
    delay: f64
}
impl FixedTimer {
    /// Instantiate a new timer.
    ///
    /// `delay` is the number of seconds between ticks. For example, 1.0/30.0 will cause this to
    /// return `true` up to 30 times a second.
    ///
    /// `max_ticks_behind` will allow a backlog of ticks.
    ///
    /// # Panics
    ///
    /// Panics if `delay` is non-positive.
    pub const fn new(delay: f64, max_ticks_behind: u16) -> Self {
        assert!(delay > 0.0, "FixedTimer::new with non-positive delay");
        Self {
            last_update: AtomicU64::new(0),
            guard: AtomicBool::new(false),
            max_ticks_behind,
            delay
        }
    }

    /// Returns true if the clock has been hit.
    ///
    /// This can return false or true immediately afterwards depending on if there is a backlog.
    pub fn test(&self) -> bool {
        if !self.lock() {
            return false
        }

        let tick_length = PerformanceCounterDelta::from_seconds_f64(self.delay);
        let tick_length_counter = tick_length.counter as u64;

        let now = PerformanceCounter::now();
        let last_update_get = self.last_update.load(Ordering::Relaxed);
        let last_update_counter = PerformanceCounter { counter: last_update_get };

        let iterations = (now.counter - last_update_counter.counter) / tick_length_counter;

        if iterations <= 0 {
            self.unlock();
            return false;
        }

        let new_value;
        let max_backlog = self.max_ticks_behind as u64 + 1;
        if iterations > max_backlog {
            new_value = now.counter - (max_backlog * tick_length_counter);
        }
        else {
            new_value = last_update_get + tick_length_counter;
        }

        self.last_update.store(new_value, Ordering::Relaxed);
        self.unlock();
        true
    }

    /// Runs the closure up to `max_ticks_behind` times.
    pub fn run<F: FnMut()>(&self, mut what: F) {
        for i in 0..self.max_ticks_behind {
            if !self.test() {
                return
            }
            what();
        }
    }

    fn lock(&self) -> bool {
        !self.guard.swap(true, Ordering::Relaxed)
    }

    fn unlock(&self) {
        assert!(self.guard.swap(false, Ordering::Relaxed), "FixedTimer::unlock when not locked...");
    }
}

/// Use for things that should NOT be tied to the game's tick rate.
///
/// If something ought to be tied to the tick rate, use `get_game_time_fractional()` instead.
pub struct InterpolatedTimer {
    delay: AtomicU64,
    start: AtomicU64
}
impl InterpolatedTimer {
    /// Instantiate a new timer with each tick being one second.
    ///
    /// This is the default timer.
    pub const fn second_timer() -> Self {
        Self::new(1.0)
    }

    /// Instantiate a new timer.
    ///
    /// `delay` is the number of seconds between ticks.
    ///
    /// # Panics
    ///
    /// Panics if `delay` is non-positive.
    pub const fn new(delay: f64) -> Self {
        assert!(delay > 0.0, "InterpolatedTimer::new with non-positive delay");
        Self {
            start: AtomicU64::new(0),
            delay: AtomicU64::new(delay.to_bits())
        }
    }

    /// Get the current tick rate.
    pub fn value(&self) -> (u64, f64) {
        let now = PerformanceCounter::now();
        let start = self.start.load(Ordering::Relaxed);
        let start_counter = PerformanceCounter { counter: start };

        let tick_length = PerformanceCounterDelta::from_seconds_f64(self.get_delay());

        let Some(delta) = now.counter.checked_sub(start_counter.counter) else {
            panic!("InterpolatedTimer went backwards! (now: {}, start: {})", now.counter, start_counter.counter);
        };

        let tick_count = delta / (tick_length.counter as u64);
        let tick_progress = delta % (tick_length.counter as u64);
        let tick_progress_float = tick_progress as f64 / (tick_length.counter as f64);

        (tick_count, tick_progress_float)
    }

    /// Start the timer.
    pub fn start(&self) {
        self.start.store(PerformanceCounter::now().counter, Ordering::Relaxed);
    }

    /// Get the delay of the timer.
    #[inline(always)]
    pub fn get_delay(&self) -> f64 {
        f64::from_bits(self.delay.load(Ordering::Relaxed))
    }

    /// Set the delay of the timer.
    #[inline(always)]
    pub fn set_delay(&self, new_delay: f64) {
        self.delay.store(new_delay.to_bits(), Ordering::Relaxed)
    }

    /// Get the number of seconds.
    #[inline(always)]
    pub fn seconds(&self) -> f64 {
        let (ticks, fraction) = self.value();
        let start = PerformanceCounter { counter: self.start.load(Ordering::Relaxed) };
        let now = PerformanceCounter::now();
        now.time_since(start).seconds_f64()
    }
}

impl Default for InterpolatedTimer {
    fn default() -> Self {
        Self::second_timer()
    }
}


#[repr(C)]
pub struct GameTimeGlobals {
    pub initialized: u8,
    pub active: u8,
    pub _unknown_0x02: u16,
    pub _unknown_0x04: u32,
    pub _unknown_0x08: u32,
    pub game_time: u32,
    pub _unknown_0x10: u32,
    pub _unknown_0x14: u32,
    pub game_speed: f32,
    pub time_since_last_tick: f32,
}

const _: () = assert!(size_of::<GameTimeGlobals>() == 0x20);

pub const GAME_TIME_GLOBALS: VariableProvider<Option<&GameTimeGlobals>> = variable! {
    name: "game_time_globals",
    cache_address: 0x00C5913C,
    tag_address: 0x00D106F4
};

/// Returns a tuple containing the number of ticks as well as a fractional part.
///
/// For example, it may return (30, 0.5) which means 30 and a half ticks.
///
/// The fractional part can be used for interpolation.
///
/// # Panics
///
/// Panics if `game_time_globals` has not yet been initialized.
pub unsafe fn get_game_time_fractional() -> (u32, f32) {
    let globals = GAME_TIME_GLOBALS.get().expect("get_game_time_fractional with null game_time_globals");
    assert_eq!(globals.initialized, 1, "get_game_time_fractional with uninitialized game_time_globals");
    (globals.game_time, (globals.time_since_last_tick * TICK_RATE * globals.game_speed).clamp(0.0, 1.0))
}
