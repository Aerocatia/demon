use num_enum::TryFromPrimitive;
use c_mine::pointer_from_hook;
use crate::math::ColorARGB;
use crate::rasterizer::draw_string::DrawStringBounds;
use crate::util::PointerProvider;

pub mod scoreboard;
pub mod draw_string;
pub mod motion_sensor;
pub mod player_colors;
pub mod font;
pub mod hud;

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


const DRAW_BOX: PointerProvider<unsafe extern "C" fn(bounds: *const u16, color: u32)> = pointer_from_hook!("draw_box");

pub unsafe fn draw_box(bounds: DrawStringBounds, color: ColorARGB) {
    let b = [
        bounds.top,
        bounds.left,
        bounds.bottom,
        bounds.right
    ];

    DRAW_BOX.get()(b.as_ptr(), color.to_a8r8g8b8());
}
