use num_enum::TryFromPrimitive;
use c_mine::pointer_from_hook;
use tag_structs::primitives::color::{ColorARGB, Pixel32};
use tag_structs::primitives::rectangle::Rectangle;
use tag_structs::primitives::vector::{Plane3D, Vector3D};
use tag_structs::UICanvas;
use crate::util::{PointerProvider, VariableProvider};

pub mod scoreboard;
pub mod draw_string;
pub mod motion_sensor;
pub mod player_colors;
pub mod font;
pub mod hud;
pub mod c;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct RenderCamera {
    pub position: Vector3D,
    pub forward: Vector3D,
    pub up: Vector3D,
    pub mirrored: u8,
    pub padding: [u8; 3],
    pub vertical_field_of_view: f32,
    pub viewport_bounds: Rectangle,
    pub render_bounds: Rectangle,
    pub z_near: f32,
    pub z_far: f32,
    pub mirror_plane: Plane3D
}
const _: () = assert!(size_of::<RenderCamera>() == 0x54);

pub static mut WIDESCREEN_TEST: u8 = 0;

const RENDER_BOUNDS_THING: VariableProvider<RenderCamera> = variable! {
    name: "render_bounds_thing",
    cache_address: 0x00ECFE0C
};

pub fn get_render_camera() -> RenderCamera {
    // SAFETY: Not actually safe! This struct is copied around; we should intercept it and create a
    //         "safe" copy that is backed by a mutex.
    unsafe { *RENDER_BOUNDS_THING.get() }
}

/// Global canvas bounds for drawing interfaces.
pub fn get_global_interface_canvas_bounds() -> Rectangle {
    let (bounds, bounds_width, base_aspect_ratio) = const {
        let bounds = UICanvas::_640x480.get_bounds();
        let bounds_width = bounds.width();
        let base_aspect_ratio = (bounds.width() as f64) / (bounds.height() as f64);
        (bounds, bounds_width, base_aspect_ratio)
    };

    if unsafe { WIDESCREEN_TEST == 0 } {
        return bounds
    }

    let camera = get_render_camera();
    let width = camera.viewport_bounds.width();
    let height = camera.viewport_bounds.height();

    if width <= 0 || height <= 0 {
        return bounds
    }

    let new_aspect_ratio = (width as f64) / (height as f64);
    Rectangle {
        right: ((bounds_width as f64) / base_aspect_ratio * new_aspect_ratio) as i16,
        ..bounds
    }
}

/// Centered UI bounds.
///
/// This centers the interface to the screen.
///
/// This is a stopgap until proper canvas support is added.
pub fn get_fallback_ui_bounds(rectangle: Rectangle) -> Rectangle {
    let Rectangle { top, left, .. } = UICanvas::_640x480.get_bounds()
        .centered_inside(get_global_interface_canvas_bounds());
    Rectangle {
        left: rectangle.left + left,
        right: rectangle.right + left,
        top: rectangle.top + top,
        bottom: rectangle.bottom + top
    }
}

const DIRECTOR_GET_PERSPECTIVE: PointerProvider<extern "C" fn(u16) -> u16> = pointer_from_hook!("director_get_perspective");

#[derive(Copy, Clone, Debug, PartialEq, TryFromPrimitive)]
#[repr(u16)]
pub enum Perspective {
    FirstPerson,
    ThirdPerson,
    Cinematic,
    Flycam
}
impl Perspective {
    pub unsafe fn from_local_player(player: u16) -> Self {
        let perspective_raw = DIRECTOR_GET_PERSPECTIVE.get()(player);
        perspective_raw.try_into().expect("invalid perspective")
    }
    pub const fn player_has_camera_control(self) -> bool {
        match self {
            Perspective::FirstPerson => true,
            Perspective::ThirdPerson => true,
            _ => false
        }
    }
}


const DRAW_BOX: PointerProvider<unsafe extern "C" fn(bounds: &Rectangle, color: Pixel32)> = pointer_from_hook!("draw_box");

#[inline(always)]
pub unsafe fn draw_box(color: ColorARGB, bounds: Rectangle) {
    DRAW_BOX.get()(&bounds, color.to_pixel32());
}
