use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use gravel_sys::std_c::time::{localtime, time, tm};

/// Current UTC time from the Pebble clock.
pub fn utc_now() -> DateTime<Utc> {
    let mut now = 0u32;
    unsafe {
        time(&raw mut now);
    }
    DateTime::from_timestamp(now as i64, 0).unwrap_or(DateTime::UNIX_EPOCH)
}

/// Current local wall time from the Pebble clock.
pub fn local_now() -> NaiveDateTime {
    let mut now = 0u32;
    unsafe {
        time(&raw mut now);
        let t = localtime(&raw const now);
        if t.is_null() {
            return NaiveDateTime::default();
        }
        naive_from_pebble_tm(&*t)
    }
}

/// Convert a Pebble tick-time `tm` (local) into chrono.
pub fn naive_from_pebble_tm(tm: &tm) -> NaiveDateTime {
    let date =
        NaiveDate::from_ymd_opt(tm.tm_year + 1900, (tm.tm_mon + 1) as u32, tm.tm_mday as u32)
            .unwrap_or_else(|| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());

    date.and_hms_opt(tm.tm_hour as u32, tm.tm_min as u32, tm.tm_sec as u32)
        .unwrap_or_else(|| date.and_hms_opt(0, 0, 0).unwrap())
}

/// UTC datetime for a unix timestamp.
pub fn utc_from_timestamp(ts: u32) -> DateTime<Utc> {
    DateTime::from_timestamp(ts as i64, 0).unwrap_or(DateTime::UNIX_EPOCH)
}

/// UTC datetime for a 15-minute slot index anchored at `start_15`.
pub fn utc_from_15_min_slot(start_15: u32, segment_idx: usize) -> DateTime<Utc> {
    let ts = (start_15 as u64 + segment_idx as u64) * 15 * 60;
    DateTime::from_timestamp(ts as i64, 0).unwrap_or(DateTime::UNIX_EPOCH)
}
