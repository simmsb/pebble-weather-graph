use crate::graphics::{
    gdraw::{GFont, GTextAlignment},
    types::{GColor, GRect},
};
use crate::ui::layer::Layer;
use crate::util::Opaque;
use core::ffi::c_char;

unsafe extern "C" {
    pub fn text_layer_create(frame: GRect) -> *mut TextLayer;
    pub fn text_layer_destroy(text_layer: *mut TextLayer);
    pub fn text_layer_get_layer(text_layer: *const TextLayer) -> *const Layer;
    pub fn text_layer_set_text(text_layer: *mut TextLayer, text: *const c_char);
    pub fn text_layer_set_font(text_layer: *mut TextLayer, font: *const GFont);
    pub fn text_layer_set_text_color(text_layer: *mut TextLayer, color: GColor);
    pub fn text_layer_set_background_color(text_layer: *mut TextLayer, color: GColor);
    pub fn text_layer_set_text_alignment(text_layer: *mut TextLayer, alignment: GTextAlignment);
    pub fn text_layer_get_text(text_layer: *const TextLayer) -> *const c_char;
}

#[repr(C)]
pub struct TextLayer(Opaque);
