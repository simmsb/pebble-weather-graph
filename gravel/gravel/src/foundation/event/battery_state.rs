use gravel_sys::foundation::event::battery_state as sys;
pub use sys::{BatteryChargeState, BatteryStateHandler};

pub fn subscribe(handler: BatteryStateHandler) {
    unsafe { sys::battery_state_service_subscribe(handler) }
}

pub fn unsubscribe() {
    unsafe { sys::battery_state_service_unsubscribe() }
}

pub fn peek() -> BatteryChargeState {
    unsafe { sys::battery_state_service_peek() }
}
