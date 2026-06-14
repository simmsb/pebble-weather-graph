use super::LayerRef;
use crate::graphics::{
    Bitmap,
    types::{GAlign, GColor, GCompOp, GRect},
};
use gravel_sys::ui::layer::bitmap as sys;

pub struct BitmapLayer {
    inner: *mut sys::BitmapLayer,
}

impl BitmapLayer {
    pub unsafe fn try_from_raw(inner: *mut sys::BitmapLayer) -> Option<Self> {
        (!inner.is_null()).then_some(Self { inner })
    }

    pub fn new(frame: GRect) -> Option<Self> {
        unsafe { Self::try_from_raw(sys::bitmap_layer_create(frame)) }
    }

    pub fn get_layer(&self) -> LayerRef<()> {
        unsafe { LayerRef::from_raw(sys::bitmap_layer_get_layer(self.inner).cast_mut()) }
    }

    // TODO: define BitmapRef
    //pub fn get_bitmap(&self);

    pub fn set_bitmap(&mut self, bitmap: &Bitmap) {
        unsafe { sys::bitmap_layer_set_bitmap(self.inner, bitmap.as_raw().cast_const()) }
    }

    pub fn set_alignment(&mut self, alignment: GAlign) {
        unsafe { sys::bitmap_layer_set_alignment(self.inner, alignment) }
    }

    pub fn set_background_color(&mut self, color: GColor) {
        unsafe { sys::bitmap_layer_set_background_color(self.inner, color) }
    }

    pub fn set_compositing_mode(&mut self, mode: GCompOp) {
        unsafe { sys::bitmap_layer_set_compositing_mode(self.inner, mode) }
    }
}

impl Drop for BitmapLayer {
    fn drop(&mut self) {
        unsafe { sys::bitmap_layer_destroy(self.inner) }
    }
}
