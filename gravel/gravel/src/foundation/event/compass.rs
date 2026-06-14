use gravel_sys::foundation::event::compass as sys;
pub use sys::{CompassHeading, CompassHeadingData, CompassHeadingHandler, CompassStatus};

pub fn set_heading_filter(filter: CompassHeading) {
    unsafe { sys::compass_service_set_heading_filter(filter) }
}

pub fn subscribe(handler: CompassHeadingHandler) {
    unsafe { sys::compass_service_subscribe(handler) }
}

pub fn unsubscribe() {
    unsafe { sys::compass_service_unsubscribe() }
}

pub fn peek() -> CompassHeadingData {
    let mut data = CompassHeadingData::default();
    unsafe { sys::compass_service_peek(&mut data) };
    data
}
