unsafe extern "C" {
    pub fn launch_reason() -> AppLaunchReason;
    pub fn launch_get_args() -> u32;
}

c_enum! {
    pub enum AppLaunchReason;
    SYSTEM = 0;
    USER = 1;
    PHONE = 2;
    WAKEUP = 3;
    WORKER = 4;
    QUICK_LAUNCH = 5;
    TIMELINE_ACTION = 6;
    SMARTSTRAP = 7;
}
