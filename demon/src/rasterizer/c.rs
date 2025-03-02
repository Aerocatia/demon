use c_mine::c_mine;
use tag_structs::primitives::rectangle::Rectangle;
use crate::rasterizer::get_global_interface_canvas_bounds;
use crate::util::VariableProvider;

#[derive(Default)]
#[repr(C)]
struct RasterizerTextRenderingGlobals {
    pub horizontal_scale: f32,
    pub unknown_0x04: f32,
    pub unknown_0x08: f32,
    pub unknown_0x0c: f32,
    pub unknown_0x10: f32,
    pub vertical_scale_neg: f32,
    pub unknown_0x18: f32,
    pub vertical_offset: f32,
    pub unknown_0x20: f32,
    pub unknown_0x24: f32,
    pub unknown_0x28: f32,
    pub unknown_0x2c: f32,
    pub unknown_0x30: f32,
    pub unknown_0x34: f32,
    pub unknown_0x38: f32,
    pub text_scaling: f32,
    pub unknown_0x40: f32,
    pub unknown_0x44: f32,
    pub unknown_0x48: f32,
    pub unknown_0x4c: f32,
}

const RASTERIZER_TEXT_RENDERING_GLOBALS: VariableProvider<RasterizerTextRenderingGlobals> = variable! {
    name: "rasterizer_text_rendering_globals",
    cache_address: 0x00E10900,
    tag_address: 0x00EC7EC0
};

const RASTERIZER_WIDTH_SCALE: VariableProvider<f32> = variable! {
    name: "rasterizer_width_scale",
    cache_address: 0x00955718
};

const RASTERIZER_HEIGHT_SCALE: VariableProvider<f32> = variable! {
    name: "rasterizer_height_scale",
    cache_address: 0x00955714
};

pub unsafe fn set_rasterizer_text_rendering_scaling_to_canvas(canvas: Rectangle) {
    let width = canvas.width() as f32;
    let height = canvas.height() as f32;

    *RASTERIZER_TEXT_RENDERING_GLOBALS.get_mut() = RasterizerTextRenderingGlobals {
        horizontal_scale: 2.0 / width,
        unknown_0x0c: -(1.0 + 1.0 / width),
        vertical_scale_neg: -2.0 / height,
        vertical_offset: 1.0 + 1.0 / height,
        unknown_0x2c: 0.5,
        text_scaling: 1.0,
        unknown_0x4c: 1.0,
        ..RasterizerTextRenderingGlobals::default()
    }
}

#[c_mine]
pub unsafe extern "C" fn rasterizer_text_rendering_globals_setup() {
    let bounds = get_global_interface_canvas_bounds();
    set_rasterizer_text_rendering_scaling_to_canvas(bounds);
}
