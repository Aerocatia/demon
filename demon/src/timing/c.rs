use c_mine::c_mine;
use crate::timing::{PerformanceCounter, PerformanceCounterDelta, GAME_TIME_GLOBALS};

#[c_mine]
pub unsafe extern "C" fn system_seconds() -> u32 {
    PerformanceCounter::now().time_since(PerformanceCounter::default()).seconds() as u32
}

#[c_mine]
pub unsafe extern "C" fn system_milliseconds() -> u32 {
    PerformanceCounter::now().time_since(PerformanceCounter::default()).milliseconds() as u32
}

#[c_mine]
pub unsafe extern "C" fn system_microseconds() -> u32 {
    PerformanceCounter::now().time_since(PerformanceCounter::default()).microseconds() as u32
}

#[c_mine]
pub unsafe extern "C" fn game_time_get() -> u32 {
    let globals = GAME_TIME_GLOBALS.get().expect("game_time_get with null game_time_globals");
    assert_eq!(globals.initialized, 1, "game_time_globals with uninitialized game_time_globals");
    globals.game_time
}

#[c_mine]
pub unsafe extern "C" fn qpc_seconds_float(a: i64, b: i64) -> f32 {
    let delta = a - b;
    let delta = PerformanceCounterDelta {
        backwards: delta.is_negative(),
        counter: delta.unsigned_abs()
    };
    delta.seconds_f64() as f32
}

#[c_mine]
pub extern "C" fn qpc_to_milliseconds(counter: i64) -> i64 {
    PerformanceCounterDelta {
        backwards: counter.is_negative(),
        counter: counter.unsigned_abs()
    }.milliseconds() as i64 * if counter >= 0 { 1 } else { -1 }
}

#[c_mine]
pub unsafe extern "C" fn query_performance_counter() -> u64 {
    PerformanceCounter::now().counter
}
