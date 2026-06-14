use gravel_sys::graphics::{
    gdraw::{self as sys, GFont, GTextAlignment, GTextOverflowMode, GCornersAll},
    types::{GColor, GContext, GPoint, GRect, GSize},
};

pub use gravel_sys::graphics::fonts;

pub struct Context<'a> {
    inner: *mut GContext,
    _marker: core::marker::PhantomData<&'a mut GContext>,
}

impl Context<'_> {
    pub fn from_raw(inner: *mut GContext) -> Self {
        Self {
            inner,
            _marker: core::marker::PhantomData,
        }
    }

    pub fn set_stroke_color(&self, color: GColor) {
        unsafe { sys::graphics_context_set_stroke_color(self.inner, color) }
    }

    pub fn set_fill_color(&self, color: GColor) {
        unsafe { sys::graphics_context_set_fill_color(self.inner, color) }
    }

    pub fn set_text_color(&self, color: GColor) {
        unsafe { sys::graphics_context_set_text_color(self.inner, color) }
    }

    pub fn set_stroke_width(&self, width: u8) {
        unsafe { sys::graphics_context_set_stroke_width(self.inner, width) }
    }

    pub fn set_antialiased(&self, enable: bool) {
        unsafe { sys::graphics_context_set_antialiased(self.inner, enable) }
    }

    pub fn draw_line(&self, p0: GPoint, p1: GPoint) {
        unsafe { sys::graphics_draw_line(self.inner, p0, p1) }
    }

    pub fn draw_pixel(&self, point: GPoint) {
        unsafe { sys::graphics_draw_pixel(self.inner, point) }
    }

    pub fn fill_rect(&self, rect: GRect, corner_radius: u16) {
        unsafe {
            sys::graphics_fill_rect(self.inner, rect, corner_radius, GCornersAll);
        }
    }

    pub fn draw_text(
        &self,
        text: &core::ffi::CStr,
        font: *const GFont,
        rect: GRect,
        overflow: GTextOverflowMode,
        alignment: GTextAlignment,
    ) {
        unsafe {
            sys::graphics_draw_text(
                self.inner,
                text.as_ptr(),
                font,
                rect,
                overflow,
                alignment,
                core::ptr::null(),
            )
        }
    }
}

pub fn text_layout_size(
    text: &core::ffi::CStr,
    font: *const GFont,
    bounds: GRect,
    overflow: GTextOverflowMode,
    alignment: GTextAlignment,
) -> GSize {
    unsafe {
        sys::graphics_text_layout_get_content_size(
            text.as_ptr(),
            font,
            bounds,
            overflow,
            alignment,
        )
    }
}

pub fn align_rect(rect: GRect, inside: &GRect, alignment: gravel_sys::graphics::types::GAlign) -> GRect {
    let mut aligned = rect;
    unsafe { sys::grect_align(&mut aligned, inside, alignment, false); }
    aligned
}
