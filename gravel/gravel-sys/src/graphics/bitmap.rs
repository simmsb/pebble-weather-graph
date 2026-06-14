use super::types::{GColor, GRect, GSize};
use crate::util::Opaque;

unsafe extern "C" {
    pub fn gbitmap_get_bytes_per_row(bitmap: *const GBitmap) -> u16;
    pub fn gbitmap_get_format(bitmap: *const GBitmap) -> GBitmapFormat;
    pub fn gbitmap_get_data(bitmap: *const GBitmap) -> *mut u8;
    pub fn gbitmap_set_data(
        bitmap: *mut GBitmap,
        data: *mut u8,
        format: GBitmapFormat,
        row_size_bytes: u16,
        free_on_destroy: bool,
    );
    pub fn gbitmap_get_bounds(bitmap: *const GBitmap) -> GRect;
    pub fn gbitmap_set_bounds(bitmap: *mut GBitmap, bounds: GRect);
    pub fn gbitmap_get_palette(bitmap: *const GBitmap) -> *mut GColor;
    pub fn gbitmap_set_palette(
        bitmap: *const GBitmap,
        patlette: *mut GColor,
        free_on_destroy: bool,
    );
    pub fn gbitmap_create_with_resource(resource_id: u32) -> *mut GBitmap;
    pub fn gbitmap_create_with_data(data: *const u8) -> GBitmap;
    pub fn gbitmap_create_as_sub_bitmap(
        base_bitmap: *const GBitmap,
        sub_rect: GRect,
    ) -> *mut GBitmap;
    pub fn gbitmap_create_from_png_data(png_data: *const u8, png_data_size: usize) -> *mut GBitmap;
    pub fn gbitmap_create_blank(size: GSize, format: GBitmapFormat);
    pub fn gbitmap_create_blank_with_palette(
        size: GSize,
        format: GBitmapFormat,
        palette: *mut GColor,
        free_on_destroy: bool,
    );
    pub fn gbitmap_create_palettized_from_1bit(src_bitmap: *const GBitmap) -> *mut GBitmap;
    pub fn gbitmap_destroy(bitmap: *mut GBitmap);
    pub fn gbitmap_sequence_create_with_resource(resource_id: u32) -> *mut GBitmapSequence;
    pub fn gbitmap_sequence_update_bitmap_next_frame(
        bitmap_sequence: *mut GBitmapSequence,
        bitmap: *mut GBitmap,
        delay_ms: *mut u32,
    ) -> bool;
    pub fn gbitmap_sequence_update_bitmap_by_elapsed(
        bitmap_sequence: *mut GBitmapSequence,
        bitmap: *mut GBitmap,
        elapsed_ms: *mut u32,
    ) -> bool;
    pub fn gbitmap_sequence_destroy(bitmap_sequence: *mut GBitmapSequence);
    pub fn gbitmap_sequence_restart(bitmap_sequence: *mut GBitmapSequence) -> bool;
    pub fn gbitmap_sequence_get_current_frame_idx(bitmap_sequence: *mut GBitmapSequence) -> i32;
    pub fn gbitmap_sequence_get_total_num_frames(bitmap_sequence: *mut GBitmapSequence) -> u32;
    pub fn gbitmap_sequence_get_play_count(bitmap_sequence: *mut GBitmapSequence) -> u32;
    pub fn gbitmap_sequence_set_play_count(bitmap_sequence: *mut GBitmapSequence, play_count: u32);
    pub fn gbitmap_sequence_get_bitmap_size(bitmap_sequence: *mut GBitmapSequence) -> GSize;
    pub fn gbitmap_get_data_row_info(bitmap: *mut GBitmap, y: u16) -> GBitmapDataRowInfo;
}

#[repr(C)]
pub struct GBitmapDataRowInfo {
    pub data: *mut u8,
    pub min_x: i16,
    pub max_x: i16,
}

c_enum! {
    pub enum GBitmapFormat;
    BIT_1 = 0;
    BIT_8 = 1;
    PAL_1 = 2;
    PAL_2 = 3;
    PAL_4 = 4;
    CIRC_8 = 5;
}

#[repr(C)]
pub struct GBitmap(Opaque);

#[repr(C)]
pub struct GBitmapSequence(Opaque);
