use c_mine::pointer_from_hook;
use crate::timing::{get_game_time_fractional, FixedTimer, TICK_RATE};
use crate::util::{PointerProvider, VariableProvider};

pub const MOTION_SENSOR_SWEEPER_SIZE: VariableProvider<f32> = variable! {
    name: "MOTION_SENSOR_SWEEPER_SIZE",
    cache_address: 0x00C83914,
    tag_address: 0x00D3AEB0
};

unsafe fn motion_sensor_sweeper_tick() {
    /// Equal to 2.1 * 30
    const MOTION_SENSOR_SWEEP_CYCLE_TICKS: u32 = 63;

    let (time, offset) = get_game_time_fractional();

    // The original math does ((float)time / 30.0) % 2.1, but this is susceptible to increasing
    // floating point rounding errors as time progresses.
    //
    // The code below is changed to do integer modulo of 2.1*30 (63) before doing floating point
    // math.
    //
    // We also use InterpolatedTimer so that the blip will update at any frame rate, not just 30.
    let sweeper_state = (time % MOTION_SENSOR_SWEEP_CYCLE_TICKS) as f32 + offset;
    let modulus = sweeper_state / TICK_RATE;
    if modulus >= 2.0375 {
        *MOTION_SENSOR_SWEEPER_SIZE.get_mut() = 0.4
    }
    else {
        *MOTION_SENSOR_SWEEPER_SIZE.get_mut() = 1.0 / ((modulus + 0.0625) * 1.1)
    }
}

unsafe fn motion_sensor_blip_tick() {
    const MOTION_SENSOR_BLIP_TICK: PointerProvider<unsafe extern "C" fn()> = pointer_from_hook!("motion_sensor_blip_tick");
    static BLIP_TIMER: FixedTimer = FixedTimer::new(1.0 / (TICK_RATE as f64), 4);
    BLIP_TIMER.run(|| MOTION_SENSOR_BLIP_TICK.get()());
}

pub unsafe fn update_motion_sensor() {
    motion_sensor_sweeper_tick();
    motion_sensor_blip_tick();
}
