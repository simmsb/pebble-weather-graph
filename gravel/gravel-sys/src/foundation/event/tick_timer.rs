use crate::std_c::time;

unsafe extern "C" {
    pub fn tick_timer_service_subscribe(tick_units: TimeUnits, handler: TickHandler);
    pub fn tick_timer_service_unsubscribe();
}

// TODO: this is a bitfield
c_enum! {
    pub enum TimeUnits;
    SECOND = 1;
    MINUTE = 2;
    HOUR = 4;
    DAY = 8;
    MONTH = 16;
    YEAR = 32;
}

callbacks! {
    pub fn TickHandler(tick_time: *const time::tm, units_changed: TimeUnits);
}
