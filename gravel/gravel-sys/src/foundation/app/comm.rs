unsafe extern "C" {
    pub fn app_comm_set_sniff_interval(interval: SniffInterval);
    pub fn app_comm_get_sniff_interval() -> SniffInterval;
}

c_enum! {
    pub enum SniffInterval;
    NORMAL = 0;
    REDUCED = 1;
}
