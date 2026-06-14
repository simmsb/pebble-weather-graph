use core::ffi::c_char;

unsafe extern "C" {
    pub fn app_log(
        log_level: u8,
        src_filename: *const c_char,
        src_line_number: i32,
        fmt: *const c_char,
        ...
    );
}
