use gravel_sys::foundation::app::comm as sys;

pub fn set_sniff_interval(interval: SniffInterval) {
    unsafe { sys::app_comm_set_sniff_interval(interval.into()) }
}

pub fn get_sniff_interval() -> SniffInterval {
    unsafe { sys::app_comm_get_sniff_interval().into() }
}

nice_enum! {
    pub enum SniffInterval : sys::SniffInterval;
    Normal = NORMAL;
    Reduced = REDUCED;
}
