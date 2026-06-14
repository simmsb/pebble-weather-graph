use gravel_sys::foundation::memory_management as sys;

pub fn heap_bytes_free() -> usize {
    unsafe { sys::heap_bytes_free() }
}

pub fn heap_bytes_used() -> usize {
    unsafe { sys::heap_bytes_used() }
}

pub use sys::memory_cache_flush;
