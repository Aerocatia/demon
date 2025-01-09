use core::sync::atomic::{AtomicI64, Ordering};
use windows_sys::Win32::Foundation::TRUE;
use windows_sys::Win32::System::Performance::{QueryPerformanceCounter, QueryPerformanceFrequency};

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
        let mut frequency = 0i64;

        // SAFETY: Points to a correct i64
        let success = unsafe { QueryPerformanceFrequency(&mut frequency) };
        assert_eq!(success, TRUE, "QueryPerformanceFrequency error!");

        (self.counter as f64) / (frequency as f64)
    }
}

/// Primitive for un-tying things from frame rate.
///
/// The `test` function returns true once every 1/rate of a second.
pub struct FixedTimer {
    last_update: AtomicI64,
    rate: f64
}
impl FixedTimer {
    pub const fn new(rate: f64) -> Self {
        assert!(rate > 0.0);
        Self {
            last_update: AtomicI64::new(0),
            rate
        }
    }
    pub fn test(&self) -> bool {
        let now = PerformanceCounter::now();
        let last_update_get = self.last_update.load(Ordering::Relaxed);
        let last_update_counter = PerformanceCounter { counter: last_update_get };

        if now.time_since(last_update_counter).seconds() < 1.0 / self.rate {
            return false
        }

        self.last_update.swap(now.counter, Ordering::Relaxed);
        true
    }
}
