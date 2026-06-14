unsafe extern "C" {
    pub fn health_service_events_subscribe(
        handler: HealthEventHandler,
        context: *mut core::ffi::c_void,
    ) -> bool;
    pub fn health_service_events_unsubscribe() -> bool;
    pub fn health_service_peek_current_value(metric: HealthMetric) -> HealthValue;
}

c_enum! {
    pub enum HealthEventType;
    HEALTH_EVENT_SIGNIFICANT_UPDATE = 0;
    HEALTH_EVENT_MOVEMENT_UPDATE = 1;
    HEALTH_EVENT_SLEEP_UPDATE = 2;
    HEALTH_EVENT_METRIC_ALERT = 3;
    HEALTH_EVENT_HEART_RATE_UPDATE = 3;
}

c_enum! {
    pub enum HealthMetric;
    HEALTH_METRIC_STEP_COUNT = 0;
    HEALTH_METRIC_ACTIVE_SECONDS = 1;
    HEALTH_METRIC_WALKED_DISTANCE_METERS = 2;
    HEALTH_METRIC_SLEEP_SECONDS = 3;
    HEALTH_METRIC_SLEEP_RESTFUL_SECONDS = 4;
    HEALTH_METRIC_RESTING_KCALORIES = 5;
    HEALTH_METRIC_ACTIVE_KCALORIES = 6;
    HEALTH_METRIC_HEART_RATE_BPM = 7;
    HEALTH_METRIC_HEART_RATE_RAW_BPM = 8;
    HEALTH_METRIC_SP_O2_PERCENT = 9;
}

pub type HealthValue = i32;

pub type HealthEventHandler =
    extern "C" fn(event: HealthEventType, context: *mut core::ffi::c_void);
