use crate::graphics::types::{GContext, GPoint, GRect};
use crate::ui::window::Window;
use crate::util::Opaque;
use core::ffi::c_void;

// pub mod action_bar;
pub mod bitmap;
// pub mod menu;
// pub mod rot_bitmap;
// pub mod scroll;
// pub mod simple_menu;
// pub mod status_bar;
pub mod text;

unsafe extern "C" {
    pub fn layer_create(frame: GRect) -> *mut Layer;
    pub fn layer_create_with_data(frame: GRect, data_size: usize) -> *mut Layer;
    pub fn layer_destroy(layer: *mut Layer);
    pub fn layer_mark_dirty(layer: *mut Layer);
    pub fn layer_set_update_proc(layer: *mut Layer, update_proc: LayerUpdateProc);
    pub fn layer_set_frame(layer: *mut Layer, frame: GRect);
    pub fn layer_get_frame(layer: *const Layer) -> GRect;
    pub fn layer_set_bounds(layer: *mut Layer, bounds: GRect);
    pub fn layer_get_bounds(layer: *const Layer) -> GRect;
    pub fn layer_get_unobstructed_bounds(layer: *const Layer) -> GRect;
    pub fn layer_convert_point_to_screen(layer: *const Layer, point: GPoint) -> GPoint;
    pub fn layer_convert_rect_to_screen(layer: *const Layer, rect: GRect) -> GRect;
    pub fn layer_get_window(layer: *const Layer) -> *mut Window;
    pub fn layer_remove_from_parent(layer: *mut Layer);
    pub fn layer_remove_child_layers(parent: *mut Layer);
    pub fn layer_add_child(parent: *mut Layer, child: *mut Layer);
    pub fn layer_insert_below_sibling(layer_to_insert: *mut Layer, below_sibling_layer: *mut Layer);
    pub fn layer_insert_above_sibling(layer_to_insert: *mut Layer, above_sibling_layer: *mut Layer);
    pub fn layer_set_hidden(layer: *mut Layer, hidden: bool);
    pub fn layer_get_hidden(layer: *mut Layer) -> bool;
    pub fn layer_set_clips(layer: *mut Layer, clips: bool);
    pub fn layer_get_clips(layer: *mut Layer) -> bool;
    pub fn layer_get_data(layer: *mut Layer) -> *mut c_void;
}

#[repr(C)]
pub struct Layer(Opaque);

callbacks! {
    pub fn LayerUpdateProc(layer: *mut Layer, ctx: *mut GContext);
}
