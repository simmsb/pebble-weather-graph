unsafe extern "C" {
    pub fn accel_service_peek(data: *mut AccelData) -> i32;
    pub fn accel_service_set_sampling_rate(rate: AccelSamplingRate) -> i32;
    pub fn accel_service_set_samples_per_update(num_samples: u32) -> i32;
    pub fn accel_data_service_subscribe(samples_per_update: u32, handler: AccelDataHandler);
    pub fn accel_data_service_unsubscribe();
    pub fn accel_tap_service_subscribe(handler: AccelTapHandler);
    pub fn accel_tap_service_unsubscribe();
    pub fn accel_raw_data_service_subscribe(samples_per_update: u32, handler: AccelRawDataHandler);
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct AccelData {
    pub x: i16,
    pub y: i16,
    pub z: i16,
    pub did_vibrate: bool,
    pub timestamp: u64,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct AccelRawData {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

c_enum! {
    pub enum AccelAxisType;
    X = 0;
    Y = 1;
    Z = 2;
}

c_enum! {
    pub enum AccelSamplingRate;
    HZ_10 = 10;
    HZ_25 = 25;
    HZ_50 = 50;
    HZ_100 = 100;
}

impl Default for AccelSamplingRate {
    fn default() -> Self {
        Self::HZ_25
    }
}

callbacks! {
    pub fn AccelDataHandler(data: *const AccelData, num_samples: u32);
    pub fn AccelRawDataHandler(data: *const AccelRawData, num_samples: u32, timestamp: u64);
    pub fn AccelTapHandler(axis: AccelAxisType, direction: i32);
}
