use core::ffi::c_void;

unsafe extern "C" {
    pub fn heap_bytes_free() -> usize;
    pub fn heap_bytes_used() -> usize;
    pub fn memory_cache_flush(start: *mut c_void, size: usize);
}
