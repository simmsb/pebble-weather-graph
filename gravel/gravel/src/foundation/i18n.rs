use core::ffi::CStr;

use gravel_sys::foundation::i18n as sys;

pub fn get_system_locale() -> &'static str {
    unsafe {
        let ptr = sys::i18n_get_system_locale();
        CStr::from_ptr(ptr).to_str().unwrap_or("")
    }
}
