#![allow(non_camel_case_types)]

pub mod format {
    use core::ffi::{c_char, c_int};

    unsafe extern "C" {
        pub fn snprintf(str: *mut c_char, n: usize, fmt: *const c_char, ...) -> c_int;
    }
}

pub mod locale {
    use core::ffi::{c_char, c_int};

    unsafe extern "C" {
        pub fn setlocale(category: c_int, locale: *const c_char) -> *const c_char;
    }
}

pub mod math {
    use core::ffi::{c_int, c_uint};

    unsafe extern "C" {
        pub fn rand() -> c_int;
        pub fn srand(seed: c_uint);
    }
}

pub mod memory {
    use core::ffi::{c_int, c_void};

    unsafe extern "C" {
        pub fn malloc(size: usize) -> *mut c_void;
        pub fn calloc(count: usize, size: usize) -> *mut c_void;
        pub fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
        pub fn free(ptr: *mut c_void);
        pub fn memcmp(ptr1: *const c_void, ptr2: *const c_void, n: usize) -> c_int;
        pub fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
        pub fn memmove(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
        pub fn memset(dest: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    }
}

pub mod string {
    use core::ffi::{c_char, c_int};

    unsafe extern "C" {
        pub fn strcmp(str1: *const c_char, str2: *const c_char) -> c_int;
        pub fn strncmp(str1: *const c_char, str2: *const c_char, n: usize) -> c_int;
        pub fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
        pub fn strncpy(dest: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
        pub fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char;
        pub fn strncat(dest: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
        pub fn strlen(str: *const c_char) -> usize;
    }
}

pub mod time {
    use core::ffi::{c_char, c_int};

    unsafe extern "C" {
        pub fn strftime(s: *mut c_char, maxsize: usize, format: *const c_char, tm_p: *const tm);
        pub fn localtime(timep: *const time_t) -> *mut tm;
        pub fn gmtime(timep: *const time_t) -> *mut tm;
        pub fn mktime(tb: *const tm) -> time_t;
        pub fn time(tloc: *mut time_t) -> time_t;
        //pub fn difftime(end: time_t, beginning: time_t) -> f64;
        pub fn time_ms(t_utc: *mut time_t, out_s: *mut u16) -> u16;
        pub fn start_of_today() -> time_t;
    }

    pub type time_t = core::ffi::c_uint;

    #[repr(C)]
    pub struct tm {
        pub tm_sec: c_int,
        pub tm_min: c_int,
        pub tm_hour: c_int,
        pub tm_mday: c_int,
        pub tm_mon: c_int,
        pub tm_year: c_int,
        pub tm_wday: c_int,
        pub tm_yday: c_int,
        pub tm_isdst: c_int,

        pub tm_gmtoff: c_int,
        pub tm_zone: [c_char; 6],
    }
}
