use core::ffi::c_int;

c_enum! {
    pub enum StatusCode : c_int;
    S_SUCCESS = 0;
    E_ERROR = -1;
    E_UNKNOWN = -2;
    E_INTERNAL = -3;
    E_INVALID_ARGUMENT = -4;
    E_OUT_OF_MEMORY = -5;
    E_OUT_OF_STORAGE = -6;
    E_OUT_OF_RESOURCES = -7;
    E_RANGE = -8;
    E_DOES_NOT_EXIST = -9;
    E_INVALID_OPERATION = -10;
    E_BUSY = -11;
    E_AGAIN = -12;
    // S_TRUE = 1;
    // S_FALSE = 0;
    // S_NO_MORE_ITEMS = 2;
    // S_NO_ACTION_REQUIRED = 3;
}

unsafe extern "C" {
    pub fn persist_exists(key: u32) -> bool;
    pub fn persist_get_size(key: u32) -> c_int;
    pub fn persist_read_data(key: u32, buffer: *mut u8, buffer_size: usize) -> StatusCode;
    pub fn persist_write_data(key: u32, data: *const u8, data_size: usize) -> StatusCode;
}
