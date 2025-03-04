use core::mem::zeroed;
use core::ptr::null;
use c_mine::{c_mine, pointer_from_hook};
use crate::rasterizer::draw_string::set_rasterizer_text_rendering_scaling_to_canvas;
use crate::rasterizer::{get_global_interface_canvas_bounds, RenderCamera, RenderFrustum, RENDER_CAMERA};
use crate::rasterizer::scoreboard::game_engine_nonplayer_post_rasterize;
use crate::util::{PointerProvider, VariableProvider};

const RASTERIZER_WIDTH_SCALE: VariableProvider<f32> = variable! {
    name: "rasterizer_width_scale",
    cache_address: 0x00955718
};

const RASTERIZER_HEIGHT_SCALE: VariableProvider<f32> = variable! {
    name: "rasterizer_height_scale",
    cache_address: 0x00955714
};

#[c_mine]
pub unsafe extern "C" fn rasterizer_text_rendering_globals_setup() {
    let bounds = get_global_interface_canvas_bounds();
    set_rasterizer_text_rendering_scaling_to_canvas(bounds);
}

#[derive(Copy, Clone)]
#[repr(C)]
struct RasterizerWindowParameters {
    pub render_target: u16,
    pub window_index: u16,
    pub has_mirror: u8,
    pub suppress_clear: u8,
    pub _padding_0x05: [u8; 2],
    pub render_camera: RenderCamera,
    pub render_frustum: RenderFrustum,
    pub unknown_0x1e8: [u8; 0x70]
}
const _: () = assert!(size_of::<RasterizerWindowParameters>() == 0x258);

impl RasterizerWindowParameters {
    pub const fn zeroed() -> Self {
        // SAFETY: It is only a logical error if this is zeroed out, not a safety one.
        unsafe { zeroed() }
    }
}

impl Default for RasterizerWindowParameters {
    fn default() -> Self {
        Self::zeroed()
    }
}

#[repr(C)]
pub struct RenderFullscreenOverlaysInput {
    pub unknown_0x00: u32,
    pub a: RenderCamera,
    pub b: RenderCamera
}

const RENDER_CAMERA_C: VariableProvider<RenderCamera> = variable! {
    name: "render_camera_c",
    cache_address: 0x00ECFE0C,
    tag_address: 0x00F873CC
};

const FRUSTUM_BUFFER: VariableProvider<RenderFrustum> = variable! {
    name: "FRUSTUM_BUFFER",
    cache_address: 0x00ECFE60,
    tag_address: 0x00F87420
};

const PROFILE_RENDER_WINDOW_START: PointerProvider<unsafe extern "C" fn(u32)> = pointer_from_hook!("profile_render_window_start");
const RASTERIZER_WINDOW_BEGIN: PointerProvider<unsafe extern "C" fn(&mut RasterizerWindowParameters)> = pointer_from_hook!("rasterizer_window_begin");
const INTERFACE_DRAW_FULLSCREEN_OVERLAYS: PointerProvider<unsafe extern "C" fn()> = pointer_from_hook!("interface_draw_fullscreen_overlays");
const RASTERIZER_DEBUG_DRAW: PointerProvider<unsafe extern "C" fn()> = pointer_from_hook!("rasterizer_debug_draw");
const RASTERIZER_WINDOW_END: PointerProvider<unsafe extern "C" fn()> = pointer_from_hook!("rasterizer_window_end");
const PROFILE_RENDER_WINDOW_END: PointerProvider<unsafe extern "C" fn()> = pointer_from_hook!("profile_render_window_end");
const RENDER_WINDOW_BUILD_FRUSTUM: PointerProvider<unsafe extern "C" fn(&RenderCamera, *const f32, &mut RenderFrustum, bool)> = pointer_from_hook!("render_window_build_frustum");

#[c_mine]
pub unsafe extern "C" fn render_fullscreen_overlays(parameters: &mut RenderFullscreenOverlaysInput, a: u32) {
    *RENDER_CAMERA.write() = Some(parameters.a);

    PROFILE_RENDER_WINDOW_START.get()(0);
    *RENDER_CAMERA_C.get_mut() = parameters.a;
    RENDER_WINDOW_BUILD_FRUSTUM.get()(&parameters.a, null(), FRUSTUM_BUFFER.get_mut(), true);

    let mut params = RasterizerWindowParameters {
        render_target: 1,
        suppress_clear: (a == 0) as u8,
        window_index: 0xFFFF,
        render_camera: parameters.b,
        ..RasterizerWindowParameters::zeroed()
    };

    RENDER_WINDOW_BUILD_FRUSTUM.get()(&params.render_camera, null(), &mut params.render_frustum, true);
    RASTERIZER_WINDOW_BEGIN.get()(&mut params);

    match a {
        0 => {
            INTERFACE_DRAW_FULLSCREEN_OVERLAYS.get()();
            RASTERIZER_DEBUG_DRAW.get()();
        },
        1 => {
            game_engine_nonplayer_post_rasterize();
        },
        n => unreachable!("render_fullscreen_overlays called with second parameter {n}")
    }

    RASTERIZER_WINDOW_END.get()();
    PROFILE_RENDER_WINDOW_END.get()();
}

