use gravel::foundation::dictionary::{DictReader, DictValue};
use gravel::foundation::storage;
use heapless::CString;

use crate::generated::message_keys;

fn value_as_i32(value: DictValue<'_>) -> Option<i32> {
    match value {
        DictValue::I32(v) => Some(v),
        DictValue::I8(v) => Some(v as i32),
        DictValue::U8(v) => Some(v as i32),
        DictValue::U32(v) => Some(v as i32),
        _ => None,
    }
}

fn copy_cstring<const N: usize>(value: DictValue<'_>, dest: &mut CString<N>) -> bool {
    if let DictValue::CString(s) = value {
        *dest = CString::from_bytes_truncating_at_nul(s.to_bytes_with_nul())
            .unwrap_or_else(|_| CString::new());
        true
    } else {
        false
    }
}

pub const WEATHER_NUM_HOURS: usize = 48;
pub const WEATHER_NUM_DAYS: usize = 2;
pub const TL_WEATHER_SEGS_PER_HOUR: usize = 4;
pub const WEATHER_NUM_SEGMENTS: usize = WEATHER_NUM_HOURS * TL_WEATHER_SEGS_PER_HOUR;
pub const WEATHER_CITY_MAX_LEN: usize = 24;

const STORAGE_KEY_WEATHER: u32 = 1;

#[derive(Clone, Copy, Default)]
pub struct WeatherSegment {
    pub temperature: i8,
    pub sun_level: u8,
    pub is_day: bool,
}

impl WeatherSegment {
    pub const fn new() -> Self {
        Self {
            temperature: 0,
            sun_level: 0,
            is_day: false,
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct WeatherHourlySegment {
    pub rain_amount: i8,
    pub rain_chance: u8,
    pub cloud_cover: u8,
}

impl WeatherHourlySegment {
    pub const fn new() -> Self {
        Self {
            rain_amount: 0,
            rain_chance: 0,
            cloud_cover: 0,
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct WeatherDaily {
    pub temp_high: i8,
    pub temp_low: i8,
    pub day: u8,
}

impl WeatherDaily {
    pub const fn new() -> Self {
        Self {
            temp_high: 0,
            temp_low: 0,
            day: 0,
        }
    }
}

#[derive(Clone)]
pub struct Weather {
    pub segments: [WeatherSegment; WEATHER_NUM_SEGMENTS],
    pub hourly_segments: [WeatherHourlySegment; WEATHER_NUM_HOURS],
    pub daily: [WeatherDaily; WEATHER_NUM_DAYS],
    pub report_start: u32,
    pub current_temperature: i8,
    pub location_name: CString<WEATHER_CITY_MAX_LEN>,
}

impl Default for Weather {
    fn default() -> Self {
        Self::new()
    }
}

impl Weather {
    pub fn new() -> Self {
        let mut s = Self {
            segments: [WeatherSegment::new(); WEATHER_NUM_SEGMENTS],
            hourly_segments: [WeatherHourlySegment::new(); WEATHER_NUM_HOURS],
            daily: [WeatherDaily::new(); WEATHER_NUM_DAYS],
            report_start: 0,
            current_temperature: 0,
            location_name: CString::new(),
        };

        if storage::exists(STORAGE_KEY_WEATHER)
            && storage::get_size(STORAGE_KEY_WEATHER) as usize == core::mem::size_of::<Self>()
        {
            unsafe {
                let _ = storage::read(
                    STORAGE_KEY_WEATHER,
                    core::slice::from_raw_parts_mut(
                        &mut s as *mut _ as *mut _,
                        core::mem::size_of::<Self>(),
                    ),
                );
            }
        }

        s
    }

    pub fn save(&self) {
        let bytes = unsafe {
            core::slice::from_raw_parts(
                self as *const Self as *const u8,
                core::mem::size_of::<Self>(),
            )
        };
        let _ = storage::write(STORAGE_KEY_WEATHER, bytes);
    }

    fn update_seg_from_message(&mut self, reader: DictReader<'_>) {
        let mut idx = None;
        let mut temperature = None;
        let mut sun_level = None;
        let mut is_day = None;

        for (key, value) in reader {
            if key == message_keys::MESSAGE_KEY_WEATHER_SEG_UPDATE_IDX() {
                idx = value_as_i32(value).map(|v| v as usize);
            } else if key == message_keys::MESSAGE_KEY_WEATHER_SEG_TEMPERATURE() {
                temperature = value_as_i32(value).map(|v| v as i8);
            } else if key == message_keys::MESSAGE_KEY_WEATHER_SEG_SUN_LEVEL() {
                sun_level = value_as_i32(value).map(|v| v as u8);
            } else if key == message_keys::MESSAGE_KEY_WEATHER_SEG_IS_DAY() {
                is_day = value_as_i32(value).map(|v| v != 0);
            }
        }

        let Some(idx) = idx else {
            return;
        };
        if idx >= WEATHER_NUM_SEGMENTS {
            return;
        }
        let seg = &mut self.segments[idx];
        if let Some(v) = temperature {
            seg.temperature = v;
        }
        if let Some(v) = sun_level {
            seg.sun_level = v;
        }
        if let Some(v) = is_day {
            seg.is_day = v;
        }
    }

    fn update_hour_from_message(&mut self, reader: DictReader<'_>) {
        let mut idx = None;
        let mut rain_amount = None;
        let mut rain_chance = None;
        let mut cloud_cover = None;

        for (key, value) in reader {
            if key == message_keys::MESSAGE_KEY_WEATHER_HOUR_UPDATE_IDX() {
                idx = value_as_i32(value).map(|v| v as usize);
            } else if key == message_keys::MESSAGE_KEY_WEATHER_HOUR_RAIN_AMOUNT() {
                rain_amount = value_as_i32(value).map(|v| v as i8);
            } else if key == message_keys::MESSAGE_KEY_WEATHER_HOUR_RAIN_CHANCE() {
                rain_chance = value_as_i32(value).map(|v| v as u8);
            } else if key == message_keys::MESSAGE_KEY_WEATHER_HOUR_CLOUD_COVER() {
                cloud_cover = value_as_i32(value).map(|v| v as u8);
            }
        }

        let Some(idx) = idx else {
            return;
        };
        if idx >= WEATHER_NUM_HOURS {
            return;
        }
        let seg = &mut self.hourly_segments[idx];
        if let Some(v) = rain_amount {
            seg.rain_amount = v;
        }
        if let Some(v) = rain_chance {
            seg.rain_chance = v;
        }
        if let Some(v) = cloud_cover {
            seg.cloud_cover = v;
        }
    }

    fn update_day_from_message(&mut self, reader: DictReader<'_>) {
        let mut idx = None;
        let mut temp_high = None;
        let mut temp_low = None;
        let mut day = None;

        for (key, value) in reader {
            if key == message_keys::MESSAGE_KEY_WEATHER_DAY_UPDATE_IDX() {
                idx = value_as_i32(value).map(|v| v as usize);
            } else if key == message_keys::MESSAGE_KEY_WEATHER_DAY_TEMP_HIGH() {
                temp_high = value_as_i32(value).map(|v| v as i8);
            } else if key == message_keys::MESSAGE_KEY_WEATHER_DAY_TEMP_LOW() {
                temp_low = value_as_i32(value).map(|v| v as i8);
            } else if key == message_keys::MESSAGE_KEY_WEATHER_DAY_NAME() {
                day = value_as_i32(value).map(|v| v as u8);
            }
        }

        let Some(idx) = idx else {
            return;
        };
        if idx >= WEATHER_NUM_DAYS {
            return;
        }
        let daily = &mut self.daily[idx];
        if let Some(v) = temp_high {
            daily.temp_high = v;
        }
        if let Some(v) = temp_low {
            daily.temp_low = v;
        }
        if let Some(v) = day {
            daily.day = v;
        }
    }

    fn update_from_message(&mut self, reader: DictReader<'_>) -> bool {
        let mut report_start: Option<u32> = None;
        let mut location_name = None;
        let mut current_temperature = None;

        for (key, value) in reader {
            if key == message_keys::MESSAGE_KEY_WEATHER_REPORT_START() {
                crate::info!("Weather report start: {:?}, val: {:?}", report_start, value);
                report_start = value_as_i32(value).map(|v| v as u32);
            } else if key == message_keys::MESSAGE_KEY_WEATHER_LOCATION_NAME() {
                location_name = Some(value);
            } else if key == message_keys::MESSAGE_KEY_WEATHER_CURRENT_TEMP() {
                current_temperature = value_as_i32(value).map(|v| v as i8);
            }
        }

        let mut updated = false;
        if let Some(v) = report_start {
            self.report_start = v;
            updated = true;
        }
        if let Some(v) = location_name {
            if copy_cstring(v, &mut self.location_name) {
                updated = true;
            }
        }
        if let Some(v) = current_temperature {
            self.current_temperature = v;
            updated = true;
        }
        updated
    }

    pub fn update_from_messages(&mut self, reader: &mut DictReader<'_>) -> bool {
        self.update_seg_from_message(reader.clone());
        self.update_hour_from_message(reader.clone());
        self.update_day_from_message(reader.clone());
        let updated = self.update_from_message(reader.clone());
        if updated {
            self.save();
        }
        updated
    }
}
