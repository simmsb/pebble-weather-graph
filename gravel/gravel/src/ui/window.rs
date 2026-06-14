use alloc::boxed::Box;
use core::{ffi::c_void, marker::PhantomData};
use gravel_sys::{graphics::types::GColor, ui::window as sys};

use crate::{
    ui::{
        clicks::{ButtonId, ClickRecognizerRef},
        layer::LayerRef,
    },
    util::drop_raw_box,
};

#[repr(transparent)]
pub struct Window<T = ()> {
    inner: *mut sys::Window,
    marker: PhantomData<*mut T>,
}

impl Window<()> {
    pub fn new() -> Self {
        unsafe { Self::from_raw(sys::window_create()) }
    }
}

impl<T> Window<T> {
    pub fn new_with(user_data: T) -> Self {
        unsafe {
            let inner = sys::window_create();
            let data: *mut T = Box::into_raw(Box::new(user_data));
            sys::window_set_user_data(inner, data.cast::<c_void>());
            Self::from_raw(inner)
        }
    }

    pub unsafe fn from_raw(inner: *mut sys::Window) -> Self {
        Self {
            inner,
            marker: PhantomData,
        }
    }

    pub fn set_click_config_provider(&self, func: extern "C" fn(WindowRef<T>)) {
        unsafe { sys::window_set_click_config_provider(self.inner, transmute_handler_void(func)) }
    }

    pub fn set_window_handlers<H: WindowHandlers<T>>(&self) {
        unsafe {
            let handlers = sys::WindowHandlers {
                load: transmute_handler(H::load),
                appear: transmute_handler(H::appear),
                disappear: transmute_handler(H::disappear),
                unload: transmute_handler(H::unload),
            };
            sys::window_set_window_handlers(self.inner, handlers);
        }
    }
}

impl<T> Drop for Window<T> {
    fn drop(&mut self) {
        unsafe {
            let user_data = sys::window_get_user_data(self.inner).cast::<T>();
            drop_raw_box(user_data);
            sys::window_destroy(self.inner);
        }
    }
}

#[repr(transparent)]
pub struct WindowRef<T = ()> {
    inner: *mut sys::Window,
    marker: PhantomData<T>,
}

pub type ClickHandler<T> = extern "C" fn(ClickRecognizerRef, WindowRef<T>);

impl<T> WindowRef<T> {
    pub unsafe fn from_raw(inner: *mut sys::Window) -> Self {
        Self {
            inner,
            marker: PhantomData,
        }
    }

    pub unsafe fn try_from_raw(inner: *mut sys::Window) -> Option<Self> {
        unsafe { (!inner.is_null()).then(|| Self::from_raw(inner)) }
    }

    pub fn single_click_subscribe(&self, button_id: ButtonId, handler: ClickHandler<T>) {
        unsafe {
            sys::window_single_click_subscribe(button_id.into(), transmute_handler_click(handler))
        }
    }

    pub fn single_repeating_click_subscribe(
        &self,
        button_id: ButtonId,
        repeat_interval_ms: u16,
        handler: ClickHandler<T>,
    ) {
        unsafe {
            sys::window_single_repeating_click_subscribe(
                button_id.into(),
                repeat_interval_ms,
                transmute_handler_click(handler),
            )
        }
    }

    pub fn multi_click_subscribe(
        &self,
        button_id: ButtonId,
        clicks: core::ops::RangeInclusive<u8>,
        timeout: u16,
        last_click_only: bool,
        handler: ClickHandler<T>,
    ) {
        unsafe {
            sys::window_multi_click_subscribe(
                button_id.into(),
                *clicks.start(),
                *clicks.end(),
                timeout,
                last_click_only,
                transmute_handler_click(handler),
            )
        }
    }

    pub fn long_click_subscribe(
        &self,
        button_id: ButtonId,
        delay_ms: u16,
        down_handler: ClickHandler<T>,
        up_handler: ClickHandler<T>,
    ) {
        unsafe {
            sys::window_long_click_subscribe(
                button_id.into(),
                delay_ms,
                transmute_handler_click(down_handler),
                transmute_handler_click(up_handler),
            )
        }
    }
}

mod sealed {
    use super::sys;

    pub trait AsRawWindow {
        fn as_raw(&self) -> *mut sys::Window;
    }

    impl<T> AsRawWindow for super::Window<T> {
        fn as_raw(&self) -> *mut sys::Window {
            self.inner
        }
    }

    impl<T> AsRawWindow for super::WindowRef<T> {
        fn as_raw(&self) -> *mut sys::Window {
            self.inner
        }
    }
}

pub trait WindowHandle<T>: sealed::AsRawWindow {
    fn root_layer(&self) -> LayerRef<()> {
        unsafe { LayerRef::from_raw(sys::window_get_root_layer(self.as_raw())) }
    }

    fn set_background_color(&self, background_color: GColor) {
        unsafe { sys::window_set_background_color(self.as_raw(), background_color) }
    }

    fn is_loaded(&self) -> bool {
        unsafe { sys::window_is_loaded(self.as_raw()) }
    }

    fn user_data(&self) -> &T {
        unsafe { &*sys::window_get_user_data(self.as_raw()).cast::<T>() }
    }

    fn user_data_ptr(&self) -> *mut T {
        unsafe { sys::window_get_user_data(self.as_raw()).cast::<T>() }
    }

    /// SAFETY: There's no way to verify that this is the only handle referencing this window.
    unsafe fn user_data_mut(&mut self) -> &mut T {
        unsafe { &mut *sys::window_get_user_data(self.as_raw()).cast::<T>() }
    }
}

impl<T> WindowHandle<T> for Window<T> {}
impl<T> WindowHandle<T> for WindowRef<T> {}

pub trait WindowHandlers<T> {
    extern "C" fn load(window: WindowRef<T>);
    extern "C" fn appear(window: WindowRef<T>);
    extern "C" fn disappear(window: WindowRef<T>);
    extern "C" fn unload(window: WindowRef<T>);
}

unsafe fn transmute_handler<T>(
    f: extern "C" fn(WindowRef<T>),
) -> unsafe extern "C" fn(*mut sys::Window) {
    unsafe { core::mem::transmute(f) }
}

unsafe fn transmute_handler_void<T>(
    f: extern "C" fn(WindowRef<T>),
) -> unsafe extern "C" fn(*mut c_void) {
    unsafe { core::mem::transmute(f) }
}

unsafe fn transmute_handler_click<T>(
    f: extern "C" fn(ClickRecognizerRef, WindowRef<T>),
) -> unsafe extern "C" fn(gravel_sys::ui::clicks::ClickRecognizerRef, *mut c_void) {
    unsafe { core::mem::transmute(f) }
}
