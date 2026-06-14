use gravel_sys::foundation::exit_reason as sys;
pub use sys::AppExitReason;

pub fn set(exit_reason: AppExitReason) {
    unsafe { sys::exit_reason_set(exit_reason) }
}
