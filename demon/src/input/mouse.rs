/// if >0, override mouse sensitivity
pub static mut MOUSE_SENSITIVITY: f32 = 0.0;

pub const BASE_RATIO: f64 = 0.4 * 0.00075;

pub fn scale_mouse_sensitivity(sensitivity: f32, raw_input: i32) -> f32 {
    let sensitivity_override = unsafe { MOUSE_SENSITIVITY };

    let delta = if sensitivity_override > 0.0 {
        sensitivity_override as f64 * (raw_input as f64) * BASE_RATIO
    }
    else {
        sensitivity as f64 * (raw_input as f64) * BASE_RATIO
    };

    delta as f32
}
