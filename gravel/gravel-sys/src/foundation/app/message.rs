use crate::foundation::dictionary::DictionaryIterator;
use core::ffi::c_void;

unsafe extern "C" {
    pub fn app_message_open(size_inbound: u32, size_outbound: u32) -> AppMessageResult;
    pub fn app_message_deregister_callbacks();
    pub fn app_message_get_context() -> *mut c_void;
    pub fn app_message_set_context(context: *mut c_void) -> *mut c_void;
    pub fn app_message_register_inbox_received(callback: AppMessageInboxReceived);
    pub fn app_message_register_inbox_dropped(callback: AppMessageInboxDropped);
    pub fn app_message_register_outbox_sent(callback: AppMessageOutboxSent);
    pub fn app_message_register_outbox_failed(callback: AppMessageOutboxFailed);
    pub fn app_message_inbox_size_maximum() -> u32;
    pub fn app_message_outbox_size_maximum() -> u32;
    pub fn app_message_outbox_begin(iterator: *mut *mut DictionaryIterator) -> AppMessageResult;
    pub fn app_message_outbox_send() -> AppMessageResult;
}

// TODO: bitfields lmao?
c_enum! {
    pub enum AppMessageResult;
    OK = 0;
    SEND_TIMEOUT = 2;
    SEND_REJECTED = 4;
    NOT_CONNECTED = 8;
    APP_NOT_RUNNING = 16;
    INVALID_ARGS = 32;
    BUSY = 64;
    BUFFER_OVERFLOW = 128;
    ALREADY_RELEASED = 512;
    CALLBACK_ALREADY_REGISTERED = 1024;
    CALLBACK_NOT_REGISTERED = 2048;
    OUT_OF_MEMORY = 4096;
    CLOSED = 8192;
    INTERNAL_ERROR = 16384;
    INVALID_STATE = 32768;
}

callbacks! {
    pub fn AppMessageInboxReceived(iterator: *mut DictionaryIterator, context: *mut c_void);
    pub fn AppMessageInboxDropped(reason: AppMessageResult, context: *mut c_void);
    pub fn AppMessageOutboxSent(iterator: *mut DictionaryIterator, context: *mut c_void);
    pub fn AppMessageOutboxFailed(iterator: *mut DictionaryIterator, reason: AppMessageResult, context: *mut c_void);
}

pub const APP_MESSAGE_INBOX_SIZE_MINIMUM: u32 = 124;
pub const APP_MESSAGE_OUTBOX_SIZE_MINIMUM: u32 = 636;
