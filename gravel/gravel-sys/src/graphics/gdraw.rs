use crate::graphics::types::{GAlign, GColor, GContext, GPoint, GRect, GSize};
use core::ffi::c_char;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum GTextAlignment {
    Left,
    Center,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum GTextOverflowMode {
    WordWrap,
    TrailingEllipsis,
    Clip,
    Fill,
    Dot,
}

pub type GCornerMask = u16;
pub const GCornersAll: GCornerMask = 0xF;

#[repr(C)]
pub struct GTextAttributes {
    _data: [u8; 0],
}

unsafe extern "C" {
    pub fn graphics_context_set_stroke_color(ctx: *mut GContext, color: GColor);
    pub fn graphics_context_set_fill_color(ctx: *mut GContext, color: GColor);
    pub fn graphics_context_set_text_color(ctx: *mut GContext, color: GColor);
    pub fn graphics_context_set_stroke_width(ctx: *mut GContext, stroke_width: u8);
    pub fn graphics_context_set_antialiased(ctx: *mut GContext, enable: bool);
    pub fn graphics_draw_line(ctx: *mut GContext, p0: GPoint, p1: GPoint);
    pub fn graphics_draw_pixel(ctx: *mut GContext, point: GPoint);
    pub fn graphics_fill_rect(
        ctx: *mut GContext,
        rect: GRect,
        corner_radius: u16,
        corners: GCornerMask,
    );
    pub fn graphics_draw_text(
        ctx: *mut GContext,
        text: *const c_char,
        font: *const GFont,
        box_: GRect,
        overflow_mode: GTextOverflowMode,
        alignment: GTextAlignment,
        attrs: *const GTextAttributes,
    );
    pub fn graphics_text_layout_get_content_size(
        text: *const c_char,
        font: *const GFont,
        bounds: GRect,
        overflow_mode: GTextOverflowMode,
        alignment: GTextAlignment,
    ) -> GSize;
    pub fn grect_align(rect: *mut GRect, inside: *const GRect, alignment: GAlign, clip: bool);
}

#[repr(C)]
pub struct GFont {
    _data: [u8; 0],
}
