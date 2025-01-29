use c_mine::c_mine;

/// number of "dots" for a full rotation
pub const BASE_AIM_DOT_RATIO: f32 = 1.0 / 512.0;

/// if >0, override mouse sensitivity
pub static mut MOUSE_SENSITIVITY: f32 = 0.0;

#[c_mine]
pub extern "C" fn mouse_sensitivity(sensitivity: f32, raw_input: i32) -> f32 {
    let sensitivity_override = unsafe { MOUSE_SENSITIVITY };

    if sensitivity_override > 0.0 {
        sensitivity_override * (raw_input as f32) * BASE_AIM_DOT_RATIO
    }
    else {
        sensitivity * (raw_input as f32) * BASE_AIM_DOT_RATIO
    }
}

