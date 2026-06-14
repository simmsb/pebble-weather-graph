use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn dictation_session_create(
        buffer_size: u32,
        callback: DictationSessionStatusCallback,
        callback_context: *mut c_void,
    ) -> *mut DictationSession;
    pub fn dictation_session_destroy(session: *mut DictationSession);
    pub fn dictation_session_start(session: *mut DictationSession) -> DictationSessionStatus;
    pub fn dictation_session_stop(session: *mut DictationSession) -> DictationSessionStatus;
    pub fn dictation_session_enable_confirmation(session: *mut DictationSession, is_enabled: bool);
    pub fn dictation_session_enable_error_dialogs(session: *mut DictationSession, is_enabled: bool);
}

c_enum! {
    pub enum DictationSessionStatus;
    SUCCESS = 0;
    TRANSCRIPTION_REJECTED = 1;
    TRANSCRIPTION_REJECTED_WITH_ERROR = 2;
    SYSTEM_ABORTED = 3;
    NO_SPEECH_DETECTED = 4;
    CONNECTIVITY_ERROR = 5;
    DISABLED = 6;
    INTERNAL_ERROR = 7;
    RECOGNIZER_ERROR = 8;
}

#[repr(C)]
pub struct DictationSession(crate::Opaque);

callbacks! {
    pub fn DictationSessionStatusCallback(
        session: *mut DictationSession,
        status: DictationSessionStatus,
        transcription: *const c_char,
        context: *mut c_void,
    );
}
