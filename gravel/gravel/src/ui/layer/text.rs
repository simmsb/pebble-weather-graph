use core::ffi::CStr;

use gravel_sys::foundation::resources::ResHandle;
use gravel_sys::graphics::types::{GColor, GRect};
use gravel_sys::ui::layer::text as sys;

use crate::graphics::fonts::{CustomFont, SystemFont};
use crate::graphics::{BLACK, CLEAR, GTextAlignment};
use crate::ui::layer::LayerRef;

pub use gravel_sys::graphics::gdraw::GTextAlignment as TextAlignment;

/// Non-owning handle to a Pebble text layer.
#[derive(Debug, Clone, Copy)]
pub struct TextLayerRef {
    inner: *mut sys::TextLayer,
}

impl TextLayerRef {
    pub(crate) unsafe fn from_raw(inner: *mut sys::TextLayer) -> Self {
        Self { inner }
    }

    pub fn text(&self) -> &CStr {
        unsafe {
            let ptr = sys::text_layer_get_text(self.inner);
            if ptr.is_null() {
                c""
            } else {
                CStr::from_ptr(ptr)
            }
        }
    }

    pub fn layer(&self) -> LayerRef<()> {
        unsafe { LayerRef::from_raw(sys::text_layer_get_layer(self.inner).cast_mut()) }
    }

    pub fn set_frame(&self, frame: GRect) {
        unsafe {
            gravel_sys::ui::layer::layer_set_frame(
                sys::text_layer_get_layer(self.inner).cast_mut(),
                frame,
            )
        }
    }
}

/// Fluent configuration for [`TextLayer`].
pub struct TextLayerBuilder {
    frame: GRect,
    text: Option<&'static CStr>,
    text_color: GColor,
    background_color: GColor,
    alignment: TextAlignment,
    system_font: Option<SystemFont>,
    custom_resource: Option<ResHandle>,
}

impl TextLayerBuilder {
    pub fn new(frame: GRect) -> Self {
        Self {
            frame,
            text: None,
            text_color: BLACK,
            background_color: CLEAR,
            alignment: TextAlignment::Left,
            system_font: None,
            custom_resource: None,
        }
    }

    pub fn text(mut self, text: &'static CStr) -> Self {
        self.text = Some(text);
        self
    }

    pub fn text_color(mut self, color: GColor) -> Self {
        self.text_color = color;
        self
    }

    pub fn background_color(mut self, color: GColor) -> Self {
        self.background_color = color;
        self
    }

    pub fn alignment(mut self, alignment: TextAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn system_font(mut self, font: SystemFont) -> Self {
        self.system_font = Some(font);
        self.custom_resource = None;
        self
    }

    pub fn custom_font(mut self, resource: ResHandle) -> Self {
        self.custom_resource = Some(resource);
        self.system_font = None;
        self
    }

    pub fn build(self) -> Option<TextLayer> {
        let mut layer = TextLayer::new(self.frame)?;
        layer.set_text_color(self.text_color);
        layer.set_background_color(self.background_color);
        layer.set_alignment(self.alignment);

        if let Some(resource) = self.custom_resource {
            layer.set_custom_font(resource);
        } else if let Some(font) = self.system_font {
            layer.set_system_font(font);
        }

        if let Some(text) = self.text {
            layer.set_text(text);
        }

        Some(layer)
    }
}

pub struct TextLayer {
    inner: *mut sys::TextLayer,
    custom_font: Option<CustomFont>,
}

impl TextLayer {
    pub fn new(frame: GRect) -> Option<Self> {
        let inner = unsafe { sys::text_layer_create(frame) };
        (!inner.is_null()).then(|| Self {
            inner,
            custom_font: None,
        })
    }

    pub fn builder(frame: GRect) -> TextLayerBuilder {
        TextLayerBuilder::new(frame)
    }

    pub fn as_ref(&self) -> TextLayerRef {
        TextLayerRef { inner: self.inner }
    }

    pub fn set_text(&self, text: &CStr) {
        unsafe { sys::text_layer_set_text(self.inner, text.as_ptr()) }
    }

    pub fn text(&self) -> &CStr {
        unsafe { CStr::from_ptr(sys::text_layer_get_text(self.inner)) }
    }

    pub fn set_system_font(&mut self, font: SystemFont) {
        self.custom_font = None;
        unsafe { sys::text_layer_set_font(self.inner, font.load()) }
    }

    pub fn set_custom_font(&mut self, resource: ResHandle) {
        let font = CustomFont::from_resource(resource);
        let ptr = font.as_ptr();
        self.custom_font = Some(font);
        unsafe { sys::text_layer_set_font(self.inner, ptr) }
    }

    pub fn set_text_color(&self, color: GColor) {
        unsafe { sys::text_layer_set_text_color(self.inner, color) }
    }

    pub fn set_background_color(&self, color: GColor) {
        unsafe { sys::text_layer_set_background_color(self.inner, color) }
    }

    pub fn set_alignment(&self, alignment: TextAlignment) {
        unsafe { sys::text_layer_set_text_alignment(self.inner, alignment) }
    }

    pub fn layer(&self) -> LayerRef<()> {
        self.as_ref().layer()
    }
}

impl Drop for TextLayer {
    fn drop(&mut self) {
        unsafe { sys::text_layer_destroy(self.inner) }
    }
}
