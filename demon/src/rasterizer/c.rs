use core::mem::zeroed;
use core::ptr::null;
use core::sync::atomic::{AtomicBool, Ordering};
use spin::Lazy;
use c_mine::{c_mine, pointer_from_hook};
use tag_structs::primitives::float::FloatOps;
use tag_structs::primitives::vector::Angle;
use tag_structs::UICanvas;
use crate::init::{get_command_line_argument_value, has_command_line_argument_value};
use crate::rasterizer::draw_string::set_rasterizer_text_rendering_scaling_to_canvas;
use crate::rasterizer::{get_global_interface_canvas_bounds, get_render_camera, RenderCamera, RenderFrustum, RENDER_CAMERA};
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

#[c_mine]
pub extern "C" fn render_camera_get_adjusted_field_of_view_tangent(horizontal_fov: Angle) -> f32 {
    // Note: The original code does tan(horizontal_fov * 0.5)*0.85, but we want to correct the result
    // for aspect ratio.
    //
    // TODO: It might not be good to do it in this function, though, as the game eventually must
    //       get vfov, and *that* should just use 4/3 instead. Then we don't have to change this
    //       function!

    // Correct 4:3 hFOV at 4:3 to the current aspect ratio
    let fov = horizontal_fov.convert_horizontal_fov(
        UICanvas::_640x480.get_aspect_ratio(),
        get_render_camera().viewport_bounds.get_aspect_ratio()
    );

    // todo: removing the `* 0.85` makes the FoV closer to Xbox, but Xbox does `* 0.85`, too.
    //       investigate why this is, and why it still looks "correct" on Xbox
    (fov.radians() * 0.5).fw_tan() * 0.85
}

#[derive(Copy, Clone)]
struct VidmodeSettings {
    width: u32,
    height: u32,
    refresh_rate: u32,
    overridden_resolution: bool,
    overridden_refresh: bool,
    force_vsync: Option<bool>,
    windowed: bool
}

static VIDMODE_SETTINGS: Lazy<VidmodeSettings> = Lazy::new(|| {
    unsafe {
        let mut vidmode = VidmodeSettings {
            width: 800,
            height: 600,
            refresh_rate: 60,
            windowed: false,

            overridden_resolution: false,
            overridden_refresh: false,
            force_vsync: None
        };

        // First check the ini
        let width_ini = ini!("video", "width");
        let height_ini = ini!("video", "height");
        if width_ini.is_some() != height_ini.is_some() {
            panic!("Must specify either both OR neither video.width and video.height in the ini");
        }
        if let Some((width, height)) = width_ini.and_then(|w| height_ini.map(|h| (w, h))) {
            vidmode.width = width.parse().expect("video.width in the ini is not a valid number");
            vidmode.height = height.parse().expect("video.height in the ini is not a valid number");
            vidmode.overridden_resolution = true;
        }
        if let Some(refresh) = ini!("video", "refresh_rate") {
            vidmode.refresh_rate = refresh.parse().expect("video.refresh_rate in the ini is not a valid number");
            vidmode.overridden_refresh = true;
        }

        // Next, check if the user has overridden this with -vidmode/-refresh
        if let Some(vidmode_str) = get_command_line_argument_value("-vidmode") {
            let vidmode_str = vidmode_str.expect_str();
            let mut split_str = vidmode_str.split(",");

            let width = split_str.next().expect("-vidmode must be height,width or height,width,refresh");
            let height = split_str.next().expect("-vidmode must be height,width or height,width,refresh");

            vidmode.width = width.parse().expect("-vidmode width is not a valid number");
            vidmode.height = height.parse().expect("-vidmode height is not a valid number");

            if let Some(refresh_rate) = split_str.next() {
                vidmode.refresh_rate = refresh_rate.parse().expect("-vidmode refresh is not a valid number");
                vidmode.overridden_refresh = true;
            }

            if split_str.next().is_some() {
                panic!("-vidmode may only have three values at most");
            }

            vidmode.overridden_resolution = true;
        }
        if let Some(refresh_str) = get_command_line_argument_value("-refresh") {
            vidmode.refresh_rate = refresh_str.expect_str().parse().expect("-refresh is not a valid number");
            vidmode.overridden_refresh = true;
        }

        // Check vsync
        let force_vsync_on = has_command_line_argument_value("-vsync");
        let force_vsync_off = has_command_line_argument_value("-novsync");

        if force_vsync_on && force_vsync_off {
            panic!("May use up to only one of -vsync or -novsync");
        }

        vidmode.force_vsync = ini_bool!("video", "vsync");
        if force_vsync_on {
            vidmode.force_vsync = Some(true);
        }
        else if force_vsync_off {
            vidmode.force_vsync = Some(false);
        }

        // Windowed mode
        vidmode.windowed = ini_bool!("video", "windowed") != Some(false) || has_command_line_argument_value("-window");

        vidmode
    }
});

const BORDERLESS_WINDOW: VariableProvider<u8> = variable! {
    name: "BORDERLESS_WINDOW",
    cache_address: 0x00DFB46F,
    tag_address: 0x00EB2A2F
};

#[c_mine]
pub unsafe extern "C" fn vidmode_settings(width: Option<&mut u32>, height: Option<&mut u32>, refresh_rate: Option<&mut u32>) {
    static VIDMODE_SETTINGS_READ: AtomicBool = AtomicBool::new(false);

    const RESOLUTION_OVERRIDDEN: VariableProvider<bool> = variable! {
        name: "RESOLUTION_OVERRIDDEN",
        cache_address: 0x00DFB470,
        tag_address: 0x00EB2A30
    };

    const REFRESH_RATE_OVERRIDDEN: VariableProvider<bool> = variable! {
        name: "REFRESH_RATE_OVERRIDDEN",
        cache_address: 0x00DFB473,
        tag_address: 0x00EB2A33
    };

    let vidmode = *VIDMODE_SETTINGS;

    // Currently an issue with this if we don't have windowed mode where it's stretched to cover the window decoration.
    //
    // Should be looked into as more is reverse engineered.
    *BORDERLESS_WINDOW.get_mut() = if !vidmode.windowed { 2 } else { 0 };

    if let Some(w) = width {
        *w = vidmode.width;
    }

    if let Some(h) = height {
        *h = vidmode.height;
    }

    if let Some(r) = refresh_rate {
        *r = vidmode.refresh_rate;
    }

    if !VIDMODE_SETTINGS_READ.swap(true, Ordering::Relaxed) {
        *RESOLUTION_OVERRIDDEN.get_mut() = vidmode.overridden_resolution;
        *REFRESH_RATE_OVERRIDDEN.get_mut() = vidmode.overridden_refresh;
    }


}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PresentParametersParameters {
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u32,
    pub vsync: u8
}

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct PresentParameters {
    pub width: u32,
    pub height: u32,
    pub parameter_0x8: u32,
    pub parameter_0xc: u32,
    pub parameter_0x10: u8,
    pub parameter_0x11: u8,
    pub parameter_0x12: u8,
    pub parameter_0x13: u8,
    pub parameter_0x14: u8,
    pub parameter_0x15: u8,
    pub parameter_0x16: u8,
    pub parameter_0x17: u8,
    pub parameter_0x18: u32,
    pub parameter_0x1c: u32,
    pub windowed_mode: u32,
    pub parameter_0x24: u32,
    pub parameter_0x28: u32,
    pub parameter_0x2c: u32,
    pub refresh_rate: u32,
    pub vsync: u32
}

impl PresentParameters {
    fn apply_overrides(&mut self) {
        let overridden_values = *VIDMODE_SETTINGS;

        if let Some(v) = overridden_values.force_vsync {
            self.vsync = if v { 1 } else { 0x80000000 };
        }

        if overridden_values.overridden_resolution {
            self.width = overridden_values.width;
            self.height = overridden_values.height;
        }

        if overridden_values.overridden_refresh {
            self.refresh_rate = overridden_values.refresh_rate;
        }

        if overridden_values.windowed {
            self.windowed_mode = 1;

            // refresh rate MUST be set to 0 if windowed mode
            self.refresh_rate = 0;
        }
    }
}

#[c_mine]
pub unsafe extern "C" fn set_present_parameters(parameters: &mut PresentParameters, parameters_parameters: Option<&PresentParametersParameters>) {
    let Some(pp) = parameters_parameters else {
        const PRESENT_PARAMETERS_PARAMETERS_PARAMETERS: VariableProvider<Option<&mut PresentParameters>> = variable! {
            name: "PRESENT_PARAMETERS_PARAMETERS_PARAMETERS",
            cache_address: 0x00DFB474,
            tag_address: 0x00EB2A34
        };

        *parameters = *PRESENT_PARAMETERS_PARAMETERS_PARAMETERS.get_copied().unwrap();
        parameters.apply_overrides();
        return
    };

    let vsync = !has_command_line_argument_value("-timedemo") && pp.vsync != 0;

    *parameters = PresentParameters::default();

    parameters.parameter_0x2c = 1;
    parameters.parameter_0x18 = if BORDERLESS_WINDOW.get_copied() == 0 { 3 } else { 1 };

    parameters.parameter_0x24 = 1;
    parameters.width = pp.width;
    parameters.height = pp.height;
    parameters.refresh_rate = pp.refresh_rate;
    parameters.parameter_0x8 = 0x16;
    parameters.parameter_0xc = 1;
    parameters.parameter_0x28 = 0x4b;

    const UNKNOWN_WINDOWSERY: VariableProvider<u32> = variable! {
        name: "UNKNOWN_WINDOWSERY",
        cache_address: 0x00F1BBB4,
        tag_address: 0x00FD3184
    };
    parameters.parameter_0x1c = UNKNOWN_WINDOWSERY.get_copied();
    parameters.vsync = if vsync { 1 } else { 0x80000000 };

    parameters.apply_overrides();
}
