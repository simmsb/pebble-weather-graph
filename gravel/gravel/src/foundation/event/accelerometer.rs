use gravel_sys::foundation::event::accelerometer as sys;
pub use sys::AccelData;

pub fn peek() -> Result<AccelData, PeekError> {
    let mut data = AccelData::default();
    match unsafe { sys::accel_service_peek(&mut data) } {
        0 => Ok(data),
        -1 => Err(PeekError::NotRunning),
        -2 => Err(PeekError::Subscribed),
        n => Err(PeekError::Other(n)),
    }
}

pub fn set_sampling_rate(rate: AccelSamplingRate) -> i32 {
    unsafe { sys::accel_service_set_sampling_rate(rate.into()) }
}

pub fn set_samples_per_update(num_samples: u32) -> i32 {
    unsafe { sys::accel_service_set_samples_per_update(num_samples) }
}

#[derive(Debug)]
#[repr(i32)]
pub enum PeekError {
    Other(i32),
    NotRunning = -1,
    Subscribed = -2,
}

nice_enum! {
    pub enum AccelSamplingRate: sys::AccelSamplingRate;
    Hz10 = HZ_10;
    Hz25 = HZ_25;
    Hz50 = HZ_50;
    Hz100 = HZ_100;
}

pub mod data_service {
    use super::sys;
    pub use sys::AccelData;

    pub fn subscribe<H: AccelDataHandler>(samples_per_update: u32) {
        unsafe { sys::accel_data_service_subscribe(samples_per_update, H::handle_sys) }
    }

    pub fn unsubscribe() {
        unsafe { sys::accel_data_service_unsubscribe() }
    }

    pub trait AccelDataHandler {
        fn handle(data: &[AccelData]);

        unsafe extern "C" fn handle_sys(data: *const AccelData, num_samples: u32) {
            let slice = unsafe { core::slice::from_raw_parts(data, num_samples as usize) };
            Self::handle(slice)
        }
    }
}

pub mod tap_service {
    use super::sys;

    nice_enum! {
        pub enum AccelAxisType: sys::AccelAxisType;
        X = X;
        Y = Y;
        Z = Z;
    }

    pub fn subscribe<H: AccelTapHandler>() {
        unsafe { sys::accel_tap_service_subscribe(H::handle_sys) }
    }

    pub fn unsubscribe() {
        unsafe { sys::accel_tap_service_unsubscribe() }
    }

    pub trait AccelTapHandler {
        fn handle(axis: AccelAxisType, direction: i32);

        unsafe extern "C" fn handle_sys(axis: sys::AccelAxisType, direction: i32) {
            Self::handle(axis.into(), direction)
        }
    }
}

pub mod raw_data_service {
    use super::sys;
    pub use sys::AccelRawData;

    pub fn subscribe<H: AccelRawDataHandler>(samples_per_update: u32) {
        unsafe { sys::accel_raw_data_service_subscribe(samples_per_update, H::handle_sys) }
    }

    // No unsubscribe?

    pub trait AccelRawDataHandler {
        fn handle(data: &[AccelRawData], timestamp: u64);

        unsafe extern "C" fn handle_sys(
            data: *const AccelRawData,
            num_samples: u32,
            timestamp: u64,
        ) {
            let slice = unsafe { core::slice::from_raw_parts(data, num_samples as usize) };
            Self::handle(slice, timestamp)
        }
    }
}
