use core::sync::atomic::{AtomicBool, AtomicI64, Ordering};
use windows_sys::Win32::Foundation::TRUE;
use windows_sys::Win32::System::Performance::{QueryPerformanceCounter, QueryPerformanceFrequency};
use c_mine::c_mine;
use crate::util::VariableProvider;

#[derive(Default, Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct PerformanceCounter {
    pub counter: i64
}
impl PerformanceCounter {
    pub fn now() -> Self {
        let mut counter = 0i64;

        // SAFETY: Points to a correct i64
        let success = unsafe { QueryPerformanceCounter(&mut counter) };
        assert_eq!(success, TRUE, "QueryPerformanceCounter error!");
        Self { counter }
    }
    pub fn time_since(self, when: PerformanceCounter) -> PerformanceCounterDelta {
        let counter = self.counter - when.counter;
        PerformanceCounterDelta { counter }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct PerformanceCounterDelta {
    pub counter: i64
}
impl PerformanceCounterDelta {
    pub fn seconds(self) -> f64 {
        let frequency = Self::get_frequency();
        (self.counter as f64) / (frequency as f64)
    }
    pub fn from_seconds(sec: f64) -> Self {
        let frequency = Self::get_frequency();
        Self { counter: (frequency as f64 * sec) as i64 }
    }
    fn get_frequency() -> i64 {
        let mut frequency = 0i64;

        // SAFETY: Points to a correct i64
        let success = unsafe { QueryPerformanceFrequency(&mut frequency) };
        assert_eq!(success, TRUE, "QueryPerformanceFrequency error!");

        frequency
    }
}

/// Primitive for un-tying things from frame rate.
pub struct FixedTimer {
    last_update: AtomicI64,
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
            last_update: AtomicI64::new(0),
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

        let tick_length = PerformanceCounterDelta::from_seconds(self.delay);
        assert!(tick_length.counter > 0, "delta is 0 when calculated...");

        let now = PerformanceCounter::now();
        let last_update_get = self.last_update.load(Ordering::Relaxed);
        let last_update_counter = PerformanceCounter { counter: last_update_get };

        let iterations = (now.counter - last_update_counter.counter) / tick_length.counter;

        if iterations <= 0 {
            self.unlock();
            return false;
        }

        let new_value;
        let max_backlog = self.max_ticks_behind as i64 + 1;
        if iterations > max_backlog {
            new_value = now.counter - (max_backlog * tick_length.counter);
        }
        else {
            new_value = last_update_get + tick_length.counter;
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

#[repr(C)]
pub struct GameTimeGlobals {
    pub initialized: u8,
    pub active: u8,
    pub _unknown_0x2: u16,
    pub _unknown_0x4: u32,
    pub _unknown_0x8: u32,
    pub game_time: u32
}

const _: () = assert!(size_of::<GameTimeGlobals>() == 0x10);

pub const GAME_TIME_GLOBALS: VariableProvider<Option<&GameTimeGlobals>> = variable! {
    name: "game_time_globals",
    cache_address: 0x00C5913C,
    tag_address: 0x00D106F4
};

#[c_mine]
pub unsafe extern "C" fn game_time_get() -> u32 {
    let globals = GAME_TIME_GLOBALS.get().expect("game_time_get with null game_time_globals");
    assert!(globals.initialized == 1, "game_time_globals with uninitialized game_time_globals");
    globals.game_time
}
