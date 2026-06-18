use core::{ffi::CStr, marker::PhantomPinned, pin::Pin};

use gravel_sys::foundation::resources::ResHandle;
use gravel_sys::graphics::types::{GColor, GRect};
use gravel_sys::ui::layer::text as sys;
use heapless::{CString, LenType};
use pin_project::{pin_project, pinned_drop};

use crate::graphics::fonts::{CustomFont, SystemFont};
use crate::ui::layer::LayerRef;

pub use gravel_sys::graphics::gdraw::GTextAlignment as TextAlignment;

#[pin_project(!Unpin, PinnedDrop)]
pub struct TextLayer<const N: usize, LenT: LenType = u8> {
    inner: *mut sys::TextLayer,
    custom_font: Option<CustomFont>,

    // the storage of this buf is used by the TextLayer.
    #[pin]
    buf: CString<N, LenT>,
}

impl<const N: usize, LenT: LenType + Unpin> TextLayer<N, LenT> {
    pub fn new(frame: GRect) -> Option<Self> {
        let inner = unsafe { sys::text_layer_create(frame) };
        (!inner.is_null()).then(|| Self {
            inner,
            custom_font: None,
            buf: CString::new(),
        })
    }
    pub fn update_text(mut self: Pin<&mut Self>, f: impl for<'a> FnOnce(&'a mut CString<N, LenT>)) {
        f(self.as_mut().project().buf.get_mut());
        unsafe { sys::text_layer_set_text(self.inner, self.buf.as_ptr()) }
    }

    pub fn text(&self) -> &CStr {
        &self.buf
    }

    pub fn set_system_font(&mut self, font: SystemFont) -> &mut Self {
        self.custom_font = None;
        unsafe { sys::text_layer_set_font(self.inner, font.load()) }
        self
    }

    pub fn set_custom_font(&mut self, resource: ResHandle) -> &mut Self {
        let font = CustomFont::from_resource(resource);
        let ptr = font.as_ptr();
        self.custom_font = Some(font);
        unsafe { sys::text_layer_set_font(self.inner, ptr) }
        self
    }

    pub fn set_text_color(&mut self, color: GColor) -> &mut Self {
        unsafe { sys::text_layer_set_text_color(self.inner, color) }
        self
    }

    pub fn set_background_color(&mut self, color: GColor) -> &mut Self {
        unsafe { sys::text_layer_set_background_color(self.inner, color) }
        self
    }

    pub fn set_alignment(&mut self, alignment: TextAlignment) -> &mut Self {
        unsafe { sys::text_layer_set_text_alignment(self.inner, alignment) }
        self
    }

    pub fn set_frame(&self, frame: GRect) {
        unsafe {
            gravel_sys::ui::layer::layer_set_frame(
                sys::text_layer_get_layer(self.inner).cast_mut(),
                frame,
            )
        }
    }

    pub fn layer(&self) -> LayerRef<()> {
        unsafe { LayerRef::from_raw(sys::text_layer_get_layer(self.inner).cast_mut()) }
    }
}

#[pinned_drop]
impl<const N: usize, LenT: LenType> PinnedDrop for TextLayer<N, LenT> {
    fn drop(self: Pin<&mut Self>) {
        unsafe { sys::text_layer_destroy(self.inner) }
    }
}
