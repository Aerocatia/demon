use c_mine::c_mine;
use crate::timing::{game_time_get, FixedTimer};
use crate::util::{PointerProvider, VariableProvider};

pub const MOTION_SENSOR_SWEEPER_THING: VariableProvider<f32> = variable! {
    name: "MOTION_SENSOR_SWEEPER_THING",
    cache_address: 0x00A287A8,
    tag_address: 0x00AC1E28
};

pub const MOTION_SENSOR_SWEEPER_SIZE: VariableProvider<f32> = variable! {
    name: "MOTION_SENSOR_SWEEPER_SIZE",
    cache_address: 0x00C83914,
    tag_address: 0x00D3AEB0
};

const MOTION_SENSOR_BLIPS: PointerProvider<unsafe extern "C" fn()> = pointer! {
    name: "MOTION_SENSOR_BLIPS",
    cache_address: 0x00641B60,
    tag_address: 0x006493C0
};

#[c_mine]
pub unsafe extern "C" fn motion_sensor_tick() {
    static BLIP_TIMER: FixedTimer = FixedTimer::new(1.0 / 30.0, 1);

    let time = game_time_get.get()() as f32;
    let modulus = (time / 30.0) % 2.1;
    if modulus >= 2.0375 {
        *MOTION_SENSOR_SWEEPER_SIZE.get_mut() = 0.4
    }
    else {
        *MOTION_SENSOR_SWEEPER_SIZE.get_mut() = 1.0 / ((modulus + 0.0625) * *MOTION_SENSOR_SWEEPER_THING.get())
    }

    BLIP_TIMER.run(|| MOTION_SENSOR_BLIPS.get()());
}
