use core::ffi::c_void;

unsafe extern "C" {
    pub fn touch_service_subscribe(handler: TouchServiceHandler, context: *mut c_void);
    pub fn touch_service_unsubscribe();
    pub fn touch_service_is_enabled() -> bool;
}

#[repr(C)]
pub struct TouchEvent {
    pub kind: TouchEventType,
    pub x: i16,
    pub y: i16,
}

c_enum! {
    pub enum TouchEventType;
    TOUCHDOWN = 0;
    LIFTOFF = 1;
    POSITION_UPDATE = 2;
}

callbacks! {
    pub fn TouchServiceHandler(event: *const TouchEvent, context: *mut c_void);
}
