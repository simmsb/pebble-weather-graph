use core::ffi::CStr;

use gravel_sys::{foundation::resources::ResHandle, graphics::fonts::{FONT_KEY_GOTHIC_14, FONT_KEY_GOTHIC_18}};
use gravel_sys::graphics::fonts as sys;
use gravel_sys::graphics::gdraw::GFont;

/// A Pebble system font key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SystemFont {
    Gothic14,
    Gothic18,
}

impl SystemFont {
    pub const fn key(self) -> &'static CStr {
        match self {
            Self::Gothic14 => FONT_KEY_GOTHIC_14,
            Self::Gothic18 => FONT_KEY_GOTHIC_18,
        }
    }

    pub fn load(self) -> *const GFont {
        unsafe { sys::fonts_get_system_font(self.key().as_ptr()) }
    }
}

/// A custom font loaded from a bundled resource. Unloaded on drop.
pub struct CustomFont {
    ptr: *const GFont,
}

unsafe impl Send for CustomFont {}
unsafe impl Sync for CustomFont {}

impl CustomFont {
    pub fn from_resource(resource: ResHandle) -> Self {
        Self {
            ptr: unsafe { sys::fonts_load_custom_font(resource) },
        }
    }

    pub fn as_ptr(&self) -> *const GFont {
        self.ptr
    }
}

impl Drop for CustomFont {
    fn drop(&mut self) {
        unsafe { sys::fonts_unload_custom_font(self.ptr) }
    }
}
