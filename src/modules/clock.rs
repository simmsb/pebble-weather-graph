use core::{ops::Add, time::Duration};

use gravel_sys::std_c::time::{localtime, time, tm};

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Timestamp(u32);

impl Timestamp {
    pub fn as_secs(self) -> u32 {
        self.0
    }

    pub fn from_secs(secs: u32) -> Self {
        Self(secs)
    }

    pub fn as_datetime_in_local(&self) -> Datetime {
        unsafe {
            let t = localtime(&raw const self.0);
            if t.is_null() {
                return Datetime::default();
            }
            timestamp_from_pebble_tm(&*t)
        }
    }

    pub fn round_to(self, duration: Duration) -> Self {
        let secs = duration.as_secs() as u32;

        Self(self.0.next_multiple_of(secs).saturating_sub(secs))
    }
}

impl Add<Duration> for Timestamp {
    type Output = Timestamp;

    fn add(self, rhs: Duration) -> Self::Output {
        Timestamp(self.0 + rhs.as_secs() as u32)
    }
}

#[derive(Clone, Eq, PartialEq, Default, Debug)]
pub struct Datetime {
    pub year: i16,
    pub month: u8,
    pub mday: u8,
    pub wday: u8,
    pub hour: u8,
    pub min: u8,
    pub sec: u8,
}

/// Current UTC time from the Pebble clock.
pub fn utc_now() -> Timestamp {
    let mut now = 0u32;

    unsafe {
        time(&raw mut now);
    }

    Timestamp(now)
}

/// Current local wall time from the Pebble clock.
pub fn local_now() -> Datetime {
    utc_now().as_datetime_in_local()
}

pub fn timestamp_from_pebble_tm(tm: &tm) -> Datetime {
    Datetime {
        year: tm.tm_year as i16,
        month: tm.tm_mon as u8,
        mday: tm.tm_mday as u8,
        wday: tm.tm_wday as u8,
        hour: tm.tm_hour as u8,
        min: tm.tm_min as u8,
        sec: tm.tm_sec as u8,
    }
}
