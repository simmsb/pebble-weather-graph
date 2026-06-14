use crate::SysResult;
use crate::foundation::dictionary::{DictReader, DictWriter};
use alloc::boxed::Box;
use core::ffi::c_void;
use gravel_sys::foundation::app::message as sys;
use gravel_sys::foundation::dictionary::DictionaryIterator as DictIter;

pub fn open(size_inbound: u32, size_outbound: u32) -> Result<(), Error> {
    unsafe { sys::app_message_open(size_inbound, size_outbound) }.into_nice()
}
pub fn inbox_size_maximum() -> u32 {
    unsafe { sys::app_message_inbox_size_maximum() }
}
pub fn outbox_size_maximum() -> u32 {
    unsafe { sys::app_message_outbox_size_maximum() }
}

pub fn send<F: FnOnce(&mut DictWriter<'_>) -> R, R>(func: F) -> Result<R, Error> {
    unsafe {
        let mut iter = core::ptr::null_mut::<DictIter>();
        sys::app_message_outbox_begin(&raw mut iter).into_nice()?;
        let result = func(&mut *iter.cast::<DictWriter>());
        sys::app_message_outbox_send().into_nice()?;
        Ok(result)
    }
}

pub trait MessageHandler: 'static {
    fn inbox_received(&mut self, message: &mut DictReader<'_>);
    fn outbox_sent(&mut self, message: &mut DictReader<'_>);
    fn inbox_dropped(&mut self, reason: Error);
    fn outbox_failed(&mut self, message: &mut DictReader<'_>, reason: Error);
}

pub struct RegisteredHandlers {
    handlers: *mut Box<dyn MessageHandler>,
}

impl Drop for RegisteredHandlers {
    fn drop(&mut self) {
        unsafe {
            sys::app_message_set_context(core::ptr::null_mut());
            sys::app_message_deregister_callbacks();
            drop(Box::from_raw(self.handlers));
        }
    }
}

#[must_use]
pub fn register<H: MessageHandler>(handler: H) -> RegisteredHandlers {
    type DynHandler = Box<dyn MessageHandler>;
    type Reason = sys::AppMessageResult;

    fn make_err(reason: sys::AppMessageResult) -> Error {
        reason.into_nice().err().unwrap_or(Error::Other(0))
    }

    unsafe extern "C" fn inbox_received_sys(iter: *mut DictIter, context: *mut c_void) {
        if context.is_null() {
            return;
        }

        let handler = unsafe { &mut *context.cast::<DynHandler>() };
        handler.inbox_received(unsafe { &mut *iter.cast() })
    }
    unsafe extern "C" fn outbox_sent_sys(iter: *mut DictIter, context: *mut c_void) {
        if context.is_null() {
            return;
        }

        let handler = unsafe { &mut *context.cast::<DynHandler>() };
        handler.outbox_sent(unsafe { &mut *iter.cast() })
    }
    unsafe extern "C" fn inbox_dropped_sys(reason: Reason, context: *mut c_void) {
        if context.is_null() {
            return;
        }

        let handler = unsafe { &mut *context.cast::<DynHandler>() };
        handler.inbox_dropped(make_err(reason));
    }
    unsafe extern "C" fn outbox_failed_sys(
        iter: *mut DictIter,
        reason: Reason,
        context: *mut c_void,
    ) {
        if context.is_null() {
            return;
        }

        let handler = unsafe { &mut *context.cast::<DynHandler>() };
        handler.outbox_failed(unsafe { &mut *iter.cast() }, make_err(reason));
    }

    let new_ctx: *mut Box<dyn MessageHandler> =
        Box::into_raw(Box::new(Box::new(handler) as DynHandler));

    unsafe {
        let old_ctx = sys::app_message_set_context(new_ctx.cast());
        if !old_ctx.is_null() {
            let ptr = old_ctx.cast::<DynHandler>();
            drop(Box::from_raw(ptr));
        }

        sys::app_message_register_inbox_received(inbox_received_sys);
        sys::app_message_register_outbox_sent(outbox_sent_sys);
        sys::app_message_register_inbox_dropped(inbox_dropped_sys);
        sys::app_message_register_outbox_failed(outbox_failed_sys);
    };

    RegisteredHandlers { handlers: new_ctx }
}

error_enum! {
    pub enum Error: sys::AppMessageResult;
    SendTimeout = SEND_TIMEOUT;
    SendRejected = SEND_REJECTED;
    NotConnected = NOT_CONNECTED;
    AppNotRunning = APP_NOT_RUNNING;
    InvalidArgs = INVALID_ARGS;
    Busy = BUSY;
    BufferOverflow = BUFFER_OVERFLOW;
    AlreadyReleased = ALREADY_RELEASED;
    CallbackAlreadyRegistered = CALLBACK_ALREADY_REGISTERED;
    CallbackNotRegistered = CALLBACK_NOT_REGISTERED;
    OutOfMemory = OUT_OF_MEMORY;
    Closed = CLOSED;
    InternalError = INTERNAL_ERROR;
    InvalidState = INVALID_STATE;
}
