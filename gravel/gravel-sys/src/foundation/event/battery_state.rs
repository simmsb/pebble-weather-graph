unsafe extern "C" {
    pub fn battery_state_service_subscribe(handler: BatteryStateHandler);
    pub fn battery_state_service_unsubscribe();
    pub fn battery_state_service_peek() -> BatteryChargeState;
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct BatteryChargeState {
    pub charge_percent: u8,
    pub is_charging: bool,
    pub is_plugged: bool,
}

pub type BatteryStateHandler = extern "C" fn(charge: BatteryChargeState);
