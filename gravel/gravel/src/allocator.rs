use core::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;
use gravel_sys::std_c::memory as c;

struct PebbleAllocator;

// TODO: alignment lol
unsafe impl GlobalAlloc for PebbleAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { c::malloc(layout.size()).cast::<u8>() }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe { c::free(ptr.cast::<c_void>()) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        unsafe { c::calloc(1, layout.size()).cast::<u8>() }
    }

    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
        unsafe { c::realloc(ptr.cast::<c_void>(), new_size).cast::<u8>() }
    }
}

#[global_allocator]
static GLOBAL: PebbleAllocator = PebbleAllocator;
