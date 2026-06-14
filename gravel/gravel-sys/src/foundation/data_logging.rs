use core::ffi::c_void;

unsafe extern "C" {
    pub fn data_logging_create(
        tag: u32,
        item_type: DataLoggingItemType,
        item_length: u16,
        resume: bool,
    ) -> DataLoggingSessionRef;
    pub fn data_logging_finish(logging_session: DataLoggingSessionRef);
    pub fn data_logging_log(
        logging_session: DataLoggingSessionRef,
        data: *const c_void,
        num_items: u32,
    ) -> DataLoggingResult;
}

c_enum! {
    pub enum DataLoggingItemType;
    BYTE_ARRAY = 0;
    UINT = 1;
    INT = 2;
}

c_enum! {
    pub enum DataLoggingResult;
    SUCCESS = 0;
    BUSY = 1;
    FULL = 2;
    NOT_FOUND = 3;
    CLOSED = 4;
    INVALID_PARAMS = 5;
    INTERNAL_ERR = 6;
}

pub type DataLoggingSessionRef = *mut c_void;
