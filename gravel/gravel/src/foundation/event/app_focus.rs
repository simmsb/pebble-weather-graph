use gravel_sys::foundation::event::app_focus as sys;

pub fn subscribe_handlers<H: AppFocusHandlers>() {
    unsafe {
        sys::app_focus_service_subscribe_handlers(sys::AppFocusHandlers {
            will_focus: Some(H::will_focus),
            did_focus: Some(H::did_focus),
        })
    }
}

pub fn subscribe(handler: extern "C" fn(bool)) {
    unsafe { sys::app_focus_service_subscribe(handler) }
}

pub fn unsubscribe() {
    unsafe { sys::app_focus_service_unsubscribe() }
}

#[allow(unused_variables)]
pub trait AppFocusHandlers {
    extern "C" fn will_focus(in_focus: bool) {}
    extern "C" fn did_focus(in_focus: bool) {}
}
