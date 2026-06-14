use core::ffi::c_char;

use crate::std_c::time::time_t;

unsafe extern "C" {
    pub fn app_glance_add_slice(
        session: *mut AppGlanceReloadSession,
        slice: AppGlanceSlice,
    ) -> AppGlanceResult;
    pub fn app_glance_reload(callback: AppGlanceReloadCallback, context: *mut core::ffi::c_void);
}

#[repr(C)]
pub struct AppGlanceSlice {
    pub layout: AppGlanceSliceLayout,
    pub expiration_time: time_t,
}

#[repr(C)]
pub struct AppGlanceSliceLayout {
    pub icon: PublishedId,
    pub subtitle_template_string: *const c_char,
}

c_enum! {
    pub enum AppGlanceResult;
    SUCCESS = 0;
    INVALID_TEMPLATE_STRING = 1;
    TEMPLATE_STRING_TOO_LONG = 2;
    INVALID_ICON = 3;
    SLICE_CAPACITY_EXCEEDED = 4;
    EXPIRES_IN_THE_PAST = 5;
    INVALID_SESSION = 6;
}

pub type PublishedId = u32;

#[derive(Debug)]
#[repr(C)]
pub struct AppGlanceReloadSession(crate::Opaque);

callbacks! {
    pub fn AppGlanceReloadCallback(
        session: *mut AppGlanceReloadSession,
        limit: usize,
        context: *mut core::ffi::c_void,
    );
}
