use num_enum::TryFromPrimitive;
use spin::rwlock::RwLock;
use c_mine::pointer_from_hook;
use tag_structs::primitives::color::{ColorARGB, Pixel32};
use tag_structs::primitives::rectangle::Rectangle;
use tag_structs::primitives::vector::{Angle, Cube3D, Matrix4x3, Plane3D, ProjectionMatrix, Vector2D, Vector3D};
use tag_structs::UICanvas;
use crate::rasterizer::hud::c::Bounds2D;
use crate::util::PointerProvider;

pub mod scoreboard;
pub mod draw_string;
pub mod motion_sensor;
pub mod player_colors;
pub mod font;
pub mod hud;
pub mod c;
pub mod d3d9;

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct RenderCamera {
    pub position: Vector3D,
    pub forward: Vector3D,
    pub up: Vector3D,
    pub mirrored: u8,
    pub padding_0: u8,
    pub padding_1: u8,
    pub padding_2: u8,
    pub vertical_field_of_view: Angle,
    pub viewport_bounds: Rectangle,
    pub render_bounds: Rectangle,
    pub z_near: f32,
    pub z_far: f32,
    pub mirror_plane: Plane3D
}
const _: () = assert!(size_of::<RenderCamera>() == 0x54);

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct RenderFrustum {
    pub frustum_bounds: Bounds2D,
    pub world_to_view: Matrix4x3,
    pub view_to_world: Matrix4x3,
    pub world_planes: [Plane3D; 6],
    pub z_near: f32,
    pub z_far: f32,
    pub world_vertices: [Vector3D; 5],
    pub world_midpoint: Vector3D,
    pub world_bounds: Cube3D,
    pub projection_valid: u8,
    pub _padding_0x141: [u8; 0x3],
    pub projection_matrix: ProjectionMatrix,
    pub projection_world_to_screen: Vector2D
}
const _: () = assert!(size_of::<RenderFrustum>() == 0x18C);

pub static mut WIDESCREEN_TEST: u8 = 0;

static RENDER_CAMERA: RwLock<Option<RenderCamera>> = RwLock::new(None);

pub fn get_render_camera() -> RenderCamera {
    RENDER_CAMERA.read().clone().unwrap_or_else(|| RenderCamera {
        position: Vector3D::default(),
        forward: Vector3D { x: 1.0, y: 0.0, z: 0.0 },
        up: Vector3D { x: 0.0, y: 0.0, z: 1.0 },
        mirrored: 0u8,
        padding_0: 0u8,
        padding_1: 0u8,
        padding_2: 0u8,
        vertical_field_of_view: Angle::DEFAULT_VERTICAL_FOV,
        viewport_bounds: UICanvas::_640x480.get_bounds(),
        render_bounds: UICanvas::_640x480.get_bounds(),
        z_near: 0.005,
        z_far: 1000.0,
        mirror_plane: Plane3D { vector: Vector3D { x: 1.0, y: 0.0, z: 0.0 }, offset: 0.0 },
    })
}

/// Global canvas bounds for drawing interfaces.
pub fn get_global_interface_canvas_bounds() -> Rectangle {
    let (bounds, bounds_width, base_aspect_ratio) = const {
        let bounds = UICanvas::_640x480.get_bounds();
        let bounds_width = bounds.width();
        let base_aspect_ratio = bounds.get_aspect_ratio();
        (bounds, bounds_width, base_aspect_ratio)
    };

    if unsafe { WIDESCREEN_TEST == 0 } {
        return bounds
    }

    let new_aspect_ratio = get_render_camera().viewport_bounds.get_aspect_ratio();
    if new_aspect_ratio <= 0.0 {
        return bounds
    }

    Rectangle {
        right: ((bounds_width as f32) / base_aspect_ratio * new_aspect_ratio) as i16,
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
