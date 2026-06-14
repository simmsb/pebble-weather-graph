use gravel_sys::foundation::launch_reason as sys;

pub fn launch_reason() -> AppLaunchReason {
    unsafe { sys::launch_reason() }.into()
}

pub fn get_args() -> u32 {
    unsafe { sys::launch_get_args() }
}

nice_enum! {
    pub enum AppLaunchReason: sys::AppLaunchReason;
    System = SYSTEM;
    User = USER;
    Phone = PHONE;
    Wakeup = WAKEUP;
    Worker = WORKER;
    QuickLaunch = QUICK_LAUNCH;
    TimelineAction = TIMELINE_ACTION;
    Smartstrap = SMARTSTRAP;
}
