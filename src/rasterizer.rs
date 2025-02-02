use c_mine::pointer_from_hook;
use crate::math::ColorARGB;
use crate::rasterizer::draw_string::DrawStringBounds;
use crate::util::PointerProvider;

pub mod scoreboard;
pub mod draw_string;
pub mod motion_sensor;
pub mod player_colors;
pub mod font;

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
