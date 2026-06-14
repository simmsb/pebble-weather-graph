use alloc::boxed::Box;
use core::ffi::c_void;
use gravel_sys::foundation::event::touch as sys;
pub use sys::TouchEvent;

// It's kinda nuts that I needed to implement this using a global variable.
static mut CLEANUP_DATA: Option<(*mut c_void, unsafe fn(*mut c_void))> = None;

fn cleanup() {
    unsafe {
        let Some((data, func)) = CLEANUP_DATA else {
            return;
        };
        CLEANUP_DATA = None;
        func(data);
    }
}

pub fn subscribe<H: TouchServiceHandler>(handler: H) {
    cleanup();

    let data = Box::into_raw(Box::new(handler)).cast::<c_void>();
    unsafe {
        CLEANUP_DATA = Some((data, |data: *mut c_void| {
            let data: *mut H = data.cast();
            drop(Box::from_raw(data))
        }));
        sys::touch_service_subscribe(H::handle_sys, data);
    }
}

pub fn unsubscribe() {
    cleanup();
    unsafe { sys::touch_service_unsubscribe() }
}

pub fn is_enabled() -> bool {
    unsafe { sys::touch_service_is_enabled() }
}

pub trait TouchServiceHandler: FnMut(&TouchEvent) + Sized + 'static {
    unsafe extern "C" fn handle_sys(event: *const TouchEvent, context: *mut c_void) {
        unsafe {
            let this = &mut *context.cast::<Self>();
            let event = &*event;
            this(event);
        }
    }
}

impl<F: FnMut(&TouchEvent) + 'static> TouchServiceHandler for F {}
