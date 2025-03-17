use core::intrinsics::transmute;
use tag_structs::primitives::color::ColorRGB;
use crate::script::{HS_MACRO_FUNCTION_EVALUATE, HS_RETURN};

pub unsafe extern "C" fn console_set_color_eval(a: u16, b: u32, c: u8) {
    let v: Option<&ColorRGB> = transmute(HS_MACRO_FUNCTION_EVALUATE.get()(a,b,c));
    if let Some(&color) = v {
        let clamped = color.clamped();
        if clamped != color {
            warn!("Input color ({} {} {}) had to be clamped to {} {} {}", color.r, color.g, color.b, clamped.r, clamped.g, clamped.b);
        }
        crate::console::set_console_color(clamped);
        HS_RETURN.get()(b, 0);
    }
}
