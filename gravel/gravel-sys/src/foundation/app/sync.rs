use core::ffi::c_void;

use crate::foundation::{
    app::message::AppMessageResult,
    dictionary::{Dictionary, DictionaryIterator, DictionaryResult, Tuple, Tuplet},
};

unsafe extern "C" {
    pub fn app_sync_init(
        s: *mut AppSync,
        buffer: *mut u8,
        buffer_size: u16,
        keys_and_initial_values: *const Tuplet,
        count: u8,
        tuple_changed_callback: AppSyncTupleChangedCallback,
        error_callback: AppSyncErrorCallback,
        context: *mut c_void,
    );
    pub fn app_sync_deinit(s: *mut AppSync);
    pub fn app_sync_set(
        s: *mut AppSync,
        keys_and_values_to_update: *const Tuplet,
        count: u8,
    ) -> AppMessageResult;
    pub fn app_sync_get(s: *const AppSync, key: u32) -> *const Tuple;
}

#[repr(C)]
pub struct AppSync {
    pub current_iter: DictionaryIterator,
    pub current_or_buffer: AppSyncCurrentOrBuffer,
    pub buffer_size: u16,
    pub callback: AppSyncCallback,
}

#[repr(C)]
pub union AppSyncCurrentOrBuffer {
    pub current: *mut Dictionary,
    pub buffer: *mut u8,
}

#[repr(C)]
pub struct AppSyncCallback {
    pub value_changed: AppSyncTupleChangedCallback,
    pub error: AppSyncErrorCallback,
    pub context: *mut c_void,
}

callbacks! {
    pub fn AppSyncTupleChangedCallback(
        key: u32,
        new_tuple: *const Tuple,
        old_tuple: *const Tuple,
        context: *mut c_void,
    );
    pub fn AppSyncErrorCallback(
        dict_error: DictionaryResult,
        app_message_error: AppMessageResult,
        context: *mut c_void,
    );
}
