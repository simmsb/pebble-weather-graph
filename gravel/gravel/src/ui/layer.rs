use core::marker::PhantomData;

use gravel_sys::{graphics::types::GRect, ui::layer as sys};

// pub mod action_bar;
mod bitmap;
// pub mod menu;
// pub mod rot_bitmap;
// pub mod scroll;
// pub mod simple_menu;
mod text;
// pub mod status_bar;
// pub mod text;

pub use text::{TextAlignment, TextLayer, TextLayerBuilder, TextLayerRef};

pub use bitmap::BitmapLayer;

pub struct Layer<T> {
    inner: *mut sys::Layer,
    marker: PhantomData<*mut T>,
}

impl Layer<()> {
    pub fn new(frame: GRect) -> Self {
        unsafe { Self::from_raw(sys::layer_create(frame)) }
    }
}

impl<T> Layer<T> {
    pub unsafe fn from_raw(inner: *mut sys::Layer) -> Self {
        Self {
            inner,
            marker: PhantomData,
        }
    }

    pub fn new_with_data(frame: GRect, data: T) -> Self {
        unsafe {
            let inner = sys::layer_create_with_data(frame, core::mem::size_of::<T>());
            let ptr: *mut T = sys::layer_get_data(inner).cast::<T>();
            core::ptr::write(ptr, data);
            Self::from_raw(inner)
        }
    }
}

impl<T> Drop for Layer<T> {
    fn drop(&mut self) {
        unsafe {
            if core::mem::needs_drop::<T>() {
                let ptr: *mut T = sys::layer_get_data(self.inner).cast();
                if !ptr.is_null() {
                    core::ptr::drop_in_place(ptr);
                }
            }

            sys::layer_destroy(self.inner);
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
#[repr(transparent)]
pub struct LayerRef<T> {
    inner: *mut sys::Layer,
    marker: PhantomData<*mut T>,
}

impl<T> LayerRef<T> {
    pub(crate) unsafe fn from_raw(inner: *mut sys::Layer) -> Self {
        Self {
            inner,
            marker: PhantomData,
        }
    }
}

mod sealed {
    use super::sys;

    pub trait AsRawLayer {
        fn as_raw(&self) -> *mut sys::Layer;
    }

    impl<T> AsRawLayer for super::Layer<T> {
        fn as_raw(&self) -> *mut sys::Layer {
            self.inner
        }
    }

    impl<T> AsRawLayer for super::LayerRef<T> {
        fn as_raw(&self) -> *mut sys::Layer {
            self.inner
        }
    }
}

pub trait LayerHandle<T>: sealed::AsRawLayer {
    fn mark_dirty(&self) {
        unsafe { sys::layer_mark_dirty(self.as_raw()) }
    }

    fn set_update_proc(&self, proc: sys::LayerUpdateProc) {
        unsafe { sys::layer_set_update_proc(self.as_raw(), proc) }
    }

    fn bounds(&self) -> GRect {
        unsafe { sys::layer_get_bounds(self.as_raw()) }
    }

    fn set_frame(&self, frame: GRect) {
        unsafe { sys::layer_set_frame(self.as_raw(), frame) }
    }

    fn user_data(&self) -> &T {
        unsafe { &*sys::layer_get_data(self.as_raw()).cast::<T>() }
    }

    /// SAFETY: There's no way to verify that this is the only handle referencing this layer.
    unsafe fn user_data_mut(&mut self) -> &mut T {
        unsafe { &mut *sys::layer_get_data(self.as_raw()).cast::<T>() }
    }

    fn add_child<U>(&self, child: &impl LayerHandle<U>) {
        unsafe {
            sys::layer_add_child(self.as_raw(), child.as_raw());
        }
    }
}

impl<T> LayerHandle<T> for Layer<T> {}

impl<T> LayerHandle<T> for LayerRef<T> {}
