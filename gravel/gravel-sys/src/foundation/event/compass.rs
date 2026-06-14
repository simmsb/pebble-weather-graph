unsafe extern "C" {
    pub fn compass_service_set_heading_filter(filter: CompassHeading);
    pub fn compass_service_subscribe(handler: CompassHeadingHandler);
    pub fn compass_service_unsubscribe();
    pub fn compass_service_peek(data: *mut CompassHeadingData) -> i32;
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct CompassHeadingData {
    pub magnetic_heading: CompassHeading,
    pub true_heading: CompassHeading,
    pub compass_status: CompassStatus,
    pub is_declination_valid: bool,
}

c_enum! {
    pub enum CompassStatus;
    UNAVAILABLE = 0;
    DATA_INVALID = 1;
    CALIBRATING = 2;
    CALIBRATED = 3;
}

impl Default for CompassStatus {
    fn default() -> Self {
        Self(0)
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct CompassHeading(pub i32);

pub type CompassHeadingHandler = extern "C" fn(heading: CompassHeadingData);
