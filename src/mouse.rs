use c_mine::c_mine;

/// number of "dots" for a full rotation
pub const BASE_AIM_DOT_RATIO: f32 = 1.0 / 512.0;

#[c_mine]
pub extern "C" fn mouse_sensitivity(sensitivity: f32, raw_input: i32) -> f32 {
    sensitivity * (raw_input as f32) * BASE_AIM_DOT_RATIO
}
