use gravel_sys::foundation::event::health::{
    self as sys, HealthEventHandler, HealthMetric, HealthValue,
};

pub fn subscribe(handler: HealthEventHandler) {
    unsafe {
        sys::health_service_events_subscribe(handler, core::ptr::null_mut());
    }
}

pub fn unsubscribe() {
    unsafe {
        sys::health_service_events_unsubscribe();
    }
}

pub fn peek(metric: HealthMetric) -> HealthValue {
    unsafe { sys::health_service_peek_current_value(metric) }
}
