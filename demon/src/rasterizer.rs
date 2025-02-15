use num_enum::TryFromPrimitive;
use c_mine::pointer_from_hook;
use tag_structs::primitives::color::{ColorARGB, Pixel32};
use tag_structs::primitives::vector::Rectangle;
use crate::util::PointerProvider;

pub mod scoreboard;
pub mod draw_string;
pub mod motion_sensor;
pub mod player_colors;
pub mod font;
pub mod hud;

/// Global canvas bounds for drawing interfaces.
///
/// TODO: Currently this is just 640x480, but this is intended to be adapted to the user's current
///       aspect ratio, and sub-interfaces will use their own internal scaling.
pub fn get_global_interface_canvas_bounds() -> Rectangle {
    Rectangle {
        top: 0,
        left: 0,
        right: 640,
        bottom: 480
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
