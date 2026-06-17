use gravel_sys::std_c::time::{localtime, time, tm};

/// Current UTC time from the Pebble clock.
pub fn utc_now() -> jiff::Timestamp {
    let mut now = 0u32;
    unsafe {
        time(&raw mut now);
    }
    jiff::Timestamp::from_second(now as i64).unwrap()
}

/// Current local wall time from the Pebble clock.
pub fn local_now() -> jiff::civil::DateTime {
    let mut now = 0u32;
    unsafe {
        time(&raw mut now);
        let t = localtime(&raw const now);
        if t.is_null() {
            return jiff::civil::DateTime::default();
        }
        timestamp_from_pebble_tm(&*t)
    }
}

pub fn timestamp_from_pebble_tm(tm: &tm) -> jiff::civil::DateTime {
    jiff::civil::datetime(
        tm.tm_year as i16,
        tm.tm_mon as i8,
        tm.tm_mday as i8,
        tm.tm_hour as i8,
        tm.tm_min as i8,
        tm.tm_sec as i8,
        0,
    )
}
