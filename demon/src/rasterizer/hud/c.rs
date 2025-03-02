use c_mine::c_mine;
use tag_structs::{BitmapData, HUDInterfaceAnchor};
use tag_structs::primitives::float::FloatFunctions;
use tag_structs::primitives::vector::Vector2DInt;
use crate::rasterizer::get_global_interface_canvas_bounds;
use crate::rasterizer::hud::{draw_hud, HUD_BASE_SCALE, HUD_SAFE_ZONE_BOTTOM, HUD_SAFE_ZONE_LEFT, HUD_SAFE_ZONE_RIGHT, HUD_SAFE_ZONE_TOP};

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

#[c_mine]
pub unsafe extern "C" fn hud_calculate_point(
    local_player_index: u16,
    absolute_placement: &u16,
    placement: &Vector2DInt,
    _param_4: usize,
    override_scale: u8,
    scale: f32,
    output: &mut Vector2DInt
) {
    if _param_4 != 0 {
        panic!("param_4 was nonzero! (0x{_param_4:08X}); report this and how this happened and what map please!");
    }

    // is this used for split screen?
    let scale = if override_scale == 0 || scale == 0.0 || scale.is_nan() { HUD_BASE_SCALE } else { scale };

    let anchor = HUDInterfaceAnchor::try_from(*absolute_placement).expect("hud_calculate_point: invalid anchor given");
    let global_bounds = get_global_interface_canvas_bounds();

    let x = match anchor {
        HUDInterfaceAnchor::Center | HUDInterfaceAnchor::BottomCenter | HUDInterfaceAnchor::TopCenter => {
            ((global_bounds.left + global_bounds.right) as f32 * 0.5) + (placement.x as f32) * scale
        },
        HUDInterfaceAnchor::TopLeft | HUDInterfaceAnchor::LeftCenter | HUDInterfaceAnchor::BottomLeft => {
            (global_bounds.left as f32) + (placement.x as f32) * scale + HUD_SAFE_ZONE_LEFT
        },
        HUDInterfaceAnchor::TopRight | HUDInterfaceAnchor::RightCenter | HUDInterfaceAnchor::BottomRight => {
            (global_bounds.right as f32) - (placement.x as f32) * scale - HUD_SAFE_ZONE_RIGHT
        },
    };

    let y = match anchor {
        HUDInterfaceAnchor::Center | HUDInterfaceAnchor::LeftCenter | HUDInterfaceAnchor::RightCenter => {
            ((global_bounds.bottom + global_bounds.top) as f32 * 0.5) + (placement.y as f32) * scale
        },
        HUDInterfaceAnchor::TopLeft | HUDInterfaceAnchor::TopCenter | HUDInterfaceAnchor::TopRight => {
            (global_bounds.top as f32) + (placement.y as f32) * scale + HUD_SAFE_ZONE_TOP
        },
        HUDInterfaceAnchor::BottomLeft | HUDInterfaceAnchor::BottomCenter | HUDInterfaceAnchor::BottomRight => {
            (global_bounds.bottom as f32) - (placement.y as f32) * scale - HUD_SAFE_ZONE_BOTTOM
        },
    };

    output.x = x.round_ties_even_to_int().clamp(i16::MIN as i32, i16::MAX as i32) as i16;
    output.y = y.round_ties_even_to_int().clamp(i16::MIN as i32, i16::MAX as i32) as i16;
}

#[c_mine]
pub unsafe extern "C" fn hud_globals_get_scale() -> f32 {
    HUD_BASE_SCALE
}
