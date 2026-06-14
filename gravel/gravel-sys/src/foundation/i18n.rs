use core::ffi::c_char;

unsafe extern "C" {
    pub fn i18n_get_system_locale() -> *const c_char;
}
