use gravel_sys::foundation::math as sys;

pub fn sin_lookup(angle: i32) -> i32 {
    unsafe { sys::sin_lookup(angle) }
}

pub fn cos_lookup(angle: i32) -> i32 {
    unsafe { sys::cos_lookup(angle) }
}

pub fn atan2_lookup(y: i32, x: i32) -> i32 {
    unsafe { sys::atan2_lookup(y, x) }
}

/// The largest value that can result from a call to [sin_lookup] or [cos_lookup].
pub const TRIG_MAX_RATIO: i32 = 0xFFFF;

/// Angle value that corresponds to 360 degrees or 2π radians.
pub const TRIG_MAX_ANGLE: i32 = 0x10000;

pub const fn trig_angle_to_deg(trig_angle: i32) -> i32 {
    (trig_angle * 360) / TRIG_MAX_ANGLE
}

pub const fn deg_to_trig_angle(angle: i32) -> i32 {
    (angle * TRIG_MAX_ANGLE) / 360
}
