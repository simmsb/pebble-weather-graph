use crate::SysResult;
use alloc::boxed::Box;
use core::{
    any::Any,
    ffi::{CStr, c_char, c_void},
};
use gravel_sys::foundation::dictation as sys;

pub struct Session {
    session: *mut sys::DictationSession,
    _context: Option<Box<dyn Any>>,
}

impl Session {
    pub fn new<F: FnMut(&Session, Result<&str, Error>) + 'static>(
        buffer_size: u32,
        callback: F,
    ) -> Option<Self> {
        Self::new_impl(buffer_size, |s, r, f| f(s, r), callback)
    }

    pub fn new_impl<T: 'static>(
        buffer_size: u32,
        callback: fn(&Session, Result<&str, Error>, &mut T),
        data: T,
    ) -> Option<Self> {
        unsafe extern "C" fn callback_sys<T>(
            session: *mut sys::DictationSession,
            status: sys::DictationSessionStatus,
            transcription: *const c_char,
            context: *mut c_void,
        ) {
            if !context.is_null() {
                unsafe {
                    let (func, ref mut data) = *context.cast::<(DictationCallback<T>, T)>();
                    let result = match status.into_nice() {
                        Ok(()) => CStr::from_ptr(transcription)
                            .to_str()
                            .map_err(|_| Error::InvalidUtf8),
                        Err(e) => Err(e),
                    };

                    let nice_session = Session {
                        session,
                        _context: None,
                    };

                    func(&nice_session, result, data);
                }
            }
        }

        let mut context = Box::new((callback, data));

        let session = unsafe {
            sys::dictation_session_create(
                buffer_size,
                callback_sys::<T>,
                (&raw mut *context).cast::<c_void>(),
            )
        };

        if session.is_null() {
            return None;
        }

        Some(Self {
            session,
            _context: Some(context),
        })
    }

    pub fn start(&self) -> Result<(), Error> {
        unsafe { sys::dictation_session_start(self.session) }.into_nice()
    }

    pub fn stop(&self) -> Result<(), Error> {
        unsafe { sys::dictation_session_stop(self.session) }.into_nice()
    }

    pub fn enable_confirmation(&self, is_enabled: bool) {
        unsafe { sys::dictation_session_enable_confirmation(self.session, is_enabled) }
    }

    pub fn enable_error_dialogs(&self, is_enabled: bool) {
        unsafe { sys::dictation_session_enable_error_dialogs(self.session, is_enabled) }
    }
}

pub type DictationCallback<T> = fn(&Session, Result<&str, Error>, &mut T);

impl Drop for Session {
    fn drop(&mut self) {
        unsafe { sys::dictation_session_destroy(self.session) }
    }
}

error_enum! {
    pub enum Error: sys::DictationSessionStatus;
    TranscriptionRejected = TRANSCRIPTION_REJECTED;
    TranscriptionRejectedWithError = TRANSCRIPTION_REJECTED_WITH_ERROR;
    SystemAborted = SYSTEM_ABORTED;
    NoSpeechDetected = NO_SPEECH_DETECTED;
    ConnectivityError = CONNECTIVITY_ERROR;
    Disabled = DISABLED;
    InternalError = INTERNAL_ERROR;
    RecognizerError = RECOGNIZER_ERROR;

    @ InvalidUtf8 = -1;
}
