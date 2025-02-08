use c_mine::c_mine;
use crate::rasterizer::hud::draw_hud;

#[c_mine]
pub unsafe extern "C" fn hud_draw_screen() {
    draw_hud();
}
