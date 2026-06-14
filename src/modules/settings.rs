use core::mem;

use gravel::foundation::dictionary::DictReader;
use gravel::foundation::storage;
use gravel::graphics::{GColor, WHITE, from_hex};

use crate::generated::message_keys;
use crate::modules::util::find_i32;

const STORAGE_KEY_SETTINGS: u32 = 0;

#[derive(Clone, Copy)]
pub struct Settings {
    pub accent_color: GColor,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            accent_color: WHITE,
        }
    }
}

impl Settings {
    pub fn init() -> Self {
        let mut settings = Self::default();
        if storage::exists(STORAGE_KEY_SETTINGS) {
            let stored_size = storage::get_size(STORAGE_KEY_SETTINGS);
            if stored_size > 0 {
                let read_size = stored_size as usize;
                let struct_size = mem::size_of::<Settings>();
                let mut buf = [0u8; mem::size_of::<Settings>()];
                let n = read_size.min(struct_size);
                let _ = storage::read(STORAGE_KEY_SETTINGS, &mut buf[..n]);
                settings = unsafe { core::ptr::read(buf.as_ptr().cast()) };
            }
        }
        settings
    }

    pub fn save(&self) {
        let bytes = unsafe {
            core::slice::from_raw_parts(self as *const Self as *const u8, mem::size_of::<Self>())
        };
        let _ = storage::write(STORAGE_KEY_SETTINGS, bytes);
    }

    pub fn apply_from_message(&mut self, reader: &mut DictReader) -> bool {
        let mut changed = false;
        if let Some(hex) = find_i32(reader, message_keys::MESSAGE_KEY_SETTING_ACCENT_COLOR()) {
            self.accent_color = from_hex(hex as u32);
            changed = true;
        }
        if changed {
            self.save();
        }
        changed
    }
}
