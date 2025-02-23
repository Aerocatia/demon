use c_mine::c_mine;
use tag_structs::{BitmapData, HUDInterfaceAnchor};
use crate::rasterizer::hud::draw_hud;

#[c_mine]
pub unsafe extern "C" fn hud_draw_screen() {
    draw_hud();
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct RenderRectangle {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32
}

#[c_mine]
pub extern "C" fn calculate_static_element_screen_position(
    bitmap_data: &BitmapData,
    anchor: u16,
    rectangle: &RenderRectangle,
    out: &mut RenderRectangle,
    do_not_scale: u8
) {
    let do_not_scale = do_not_scale != 0;
    let anchor = HUDInterfaceAnchor::try_from(anchor).expect("calculate_static_element_screen_position: invalid anchor");

    let dot_width;
    let dot_height;
    if do_not_scale {
        dot_width = 1;
        dot_height = 1;
    }
    else {
        dot_width = bitmap_data.width;
        dot_height = bitmap_data.height;
    }

    let width = (rectangle.right - rectangle.left) * (dot_width as f32);
    let height = (rectangle.bottom - rectangle.top) * (dot_height as f32);

    *out = match anchor {
        HUDInterfaceAnchor::TopLeft => RenderRectangle {
            right: width,
            bottom: height,
            ..RenderRectangle::default()
        },
        HUDInterfaceAnchor::TopRight => RenderRectangle {
            left: -width,
            bottom: height,
            ..RenderRectangle::default()
        },
        HUDInterfaceAnchor::BottomLeft => RenderRectangle {
            right: width,
            top: -height,
            ..RenderRectangle::default()
        },
        HUDInterfaceAnchor::BottomRight => RenderRectangle {
            left: -width,
            top: -height,
            ..RenderRectangle::default()
        },
        HUDInterfaceAnchor::Center => RenderRectangle {
            left: -width / 2.0,
            right: width / 2.0,
            top: -height / 2.0,
            bottom: height / 2.0
        },
        HUDInterfaceAnchor::TopCenter => RenderRectangle {
            left: -width / 2.0,
            right: width / 2.0,
            bottom: height,
            ..RenderRectangle::default()
        },
        HUDInterfaceAnchor::BottomCenter => RenderRectangle {
            left: -width / 2.0,
            right: width / 2.0,
            top: -height,
            ..RenderRectangle::default()
        },
        HUDInterfaceAnchor::LeftCenter => RenderRectangle {
            right: width,
            top: -height / 2.0,
            bottom: height / 2.0,
            ..RenderRectangle::default()
        },
        HUDInterfaceAnchor::RightCenter => RenderRectangle {
            left: -width,
            top: -height / 2.0,
            bottom: height / 2.0,
            ..RenderRectangle::default()
        },
    }
}
