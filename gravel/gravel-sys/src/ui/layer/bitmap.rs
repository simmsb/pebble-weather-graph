use crate::{
    graphics::{
        bitmap::GBitmap,
        types::{GAlign, GColor, GCompOp, GRect},
    },
    util::Opaque,
};

unsafe extern "C" {
    pub fn bitmap_layer_create(frame: GRect) -> *mut BitmapLayer;
    pub fn bitmap_layer_destroy(bitmap_layer: *mut BitmapLayer);
    pub fn bitmap_layer_get_layer(bitmap_layer: *const BitmapLayer) -> *const super::Layer;
    pub fn bitmap_layer_get_bitmap(bitmap_layer: *mut BitmapLayer) -> *const GBitmap;
    pub fn bitmap_layer_set_bitmap(bitmap_layer: *mut BitmapLayer, bitmap: *const GBitmap);
    pub fn bitmap_layer_set_alignment(bitmap_layer: *mut BitmapLayer, alignment: GAlign);
    pub fn bitmap_layer_set_background_color(bitmap_layer: *mut BitmapLayer, color: GColor);
    pub fn bitmap_layer_set_compositing_mode(bitmap_layer: *mut BitmapLayer, mode: GCompOp);
}

#[repr(C)]
pub struct BitmapLayer(Opaque);
