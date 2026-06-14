use gravel_sys::graphics::bitmap as sys;

pub struct Bitmap {
    inner: *mut sys::GBitmap,
}

impl Bitmap {
    pub unsafe fn try_from_raw(inner: *mut sys::GBitmap) -> Option<Self> {
        (!inner.is_null()).then_some(Self { inner })
    }

    pub unsafe fn as_raw(&self) -> *mut sys::GBitmap {
        self.inner
    }

    pub fn from_png(data: &[u8]) -> Option<Self> {
        unsafe { Self::try_from_raw(sys::gbitmap_create_from_png_data(data.as_ptr(), data.len())) }
    }
}

impl Drop for Bitmap {
    fn drop(&mut self) {
        unsafe { sys::gbitmap_destroy(self.inner) }
    }
}
