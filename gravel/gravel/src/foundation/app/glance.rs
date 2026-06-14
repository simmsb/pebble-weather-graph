use crate::SysResult;
use core::ffi::{CStr, c_void};
use gravel_sys::foundation::app::glance as sys;
use gravel_sys::std_c::time::time_t;

pub use sys::PublishedId;

pub fn reload<F: FnOnce(&mut ReloadSession, usize)>(func: F) {
    unsafe extern "C" fn callback_sys<F: FnOnce(&mut ReloadSession, usize)>(
        session: *mut sys::AppGlanceReloadSession,
        limit: usize,
        context: *mut c_void,
    ) {
        unsafe {
            let func: F = core::ptr::read(context.cast());
            let mut session = ReloadSession { session };
            func(&mut session, limit);
        }
    }

    unsafe {
        let mut context = core::mem::MaybeUninit::new(func);
        sys::app_glance_reload(callback_sys::<F>, context.as_mut_ptr().cast());
    }
}

pub struct ReloadSession {
    session: *mut sys::AppGlanceReloadSession,
}

impl ReloadSession {
    pub fn add_slice(
        &mut self,
        icon: PublishedId,
        subtitle_template_string: Option<&CStr>,
        expiration_time: Option<time_t>,
    ) -> Result<(), Error> {
        let slice = sys::AppGlanceSlice {
            layout: sys::AppGlanceSliceLayout {
                icon,
                subtitle_template_string: subtitle_template_string
                    .map_or(core::ptr::null(), CStr::as_ptr),
            },
            expiration_time: expiration_time.unwrap_or(0),
        };

        unsafe { sys::app_glance_add_slice(self.session, slice) }.into_nice()
    }
}

error_enum! {
    pub enum Error: sys::AppGlanceResult;
    InvalidTemplateString = INVALID_TEMPLATE_STRING;
    TemplateStringTooLong = TEMPLATE_STRING_TOO_LONG;
    InvalidIcon = INVALID_ICON;
    SliceCapacityExceeded = SLICE_CAPACITY_EXCEEDED;
    ExpiresInThePast = EXPIRES_IN_THE_PAST;
    InvalidSession = INVALID_SESSION;
}
