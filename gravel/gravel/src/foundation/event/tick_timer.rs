use gravel_sys::foundation::event::tick_timer as sys;
pub use gravel_sys::std_c::time::tm as Timestamp;
pub use sys::TimeUnits;

pub fn subscribe<H: TickHandler>(tick_units: TimeUnits) {
    unsafe { sys::tick_timer_service_subscribe(tick_units, H::handle_sys) }
}

pub fn unsubscribe() {
    unsafe { sys::tick_timer_service_unsubscribe() }
}

pub trait TickHandler {
    fn handle(time: &Timestamp, units_changed: TimeUnits);

    extern "C" fn handle_sys(time: *const Timestamp, units_changed: TimeUnits) {
        Self::handle(unsafe { &*time }, units_changed)
    }
}
