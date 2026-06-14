use crate::graphics::gdraw::GFont;
use core::ffi::{CStr, c_char};

unsafe extern "C" {
    pub fn fonts_get_system_font(key: *const c_char) -> *const GFont;
    pub fn fonts_load_custom_font(resource: *const u8) -> *const GFont;
    pub fn fonts_unload_custom_font(font: *const GFont);
}

pub const FONT_KEY_GOTHIC_14: &CStr = c"GOTHIC_14";
pub const FONT_KEY_GOTHIC_18: &CStr = c"GOTHIC_18";
