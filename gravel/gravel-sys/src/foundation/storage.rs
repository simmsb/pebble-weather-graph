use core::ffi::c_int;

c_enum! {
    pub enum StatusCode : c_int;
    Success = 0;
    EError = 1;
    EInvalidArgs = 2;
    EOutOfMemory = 3;
    EDoesNotExist = 4;
    EInvalidOperation = 5;
    EAlreadyInProgress = 6;
    EOverflow = 7;
    EInternal = 8;
    EBusy = 9;
}

unsafe extern "C" {
    pub fn persist_exists(key: u32) -> bool;
    pub fn persist_get_size(key: u32) -> c_int;
    pub fn persist_read_data(key: u32, buffer: *mut u8, buffer_size: usize) -> StatusCode;
    pub fn persist_write_data(key: u32, data: *const u8, data_size: usize) -> StatusCode;
}
