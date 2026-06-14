use crate::SysResult;
use core::marker::PhantomData;
use gravel_sys::foundation::data_logging as sys;

pub struct Session<T> {
    session: sys::DataLoggingSessionRef,
    _marker: PhantomData<fn(&T)>,
}

impl<T: SessionData> Session<T> {
    pub fn new(tag: u32, resume: bool) -> Self {
        let session =
            unsafe { sys::data_logging_create(tag, T::ITEM_TYPE, T::ITEM_LENGTH, resume) };
        Self {
            session,
            _marker: PhantomData,
        }
    }

    pub fn log(&self, data: &[T]) -> Result<(), Error> {
        unsafe { sys::data_logging_log(self.session, data.as_ptr().cast(), data.len() as u32) }
            .into_nice()
    }
}

impl<T> Drop for Session<T> {
    fn drop(&mut self) {
        unsafe { sys::data_logging_finish(self.session) }
    }
}

error_enum! {
    pub enum Error: sys::DataLoggingResult;
    Busy = BUSY;
    Full = FULL;
    NotFound = NOT_FOUND;
    Closed = CLOSED;
    InvalidParams = INVALID_PARAMS;
    InternalErr = INTERNAL_ERR;
}

pub trait SessionData: sealed::Sealed + Sized {
    const ITEM_TYPE: sys::DataLoggingItemType;
    const ITEM_LENGTH: u16 = core::mem::size_of::<Self>() as u16;
}

macro_rules! session_data {
    ($($item_type:ident : $($data_type:ty),*;)*) => {$($(
        impl sealed::Sealed for $data_type {}
        impl SessionData for $data_type {
            const ITEM_TYPE: sys::DataLoggingItemType = sys::DataLoggingItemType::$item_type;
        }
    )*)*}
}

impl<const N: usize> sealed::Sealed for [u8; N] {}
impl<const N: usize> SessionData for [u8; N] {
    const ITEM_TYPE: sys::DataLoggingItemType = sys::DataLoggingItemType::BYTE_ARRAY;
}

session_data! {
    UINT: u8, u16, u32;
    INT: i8, i16, i32;
}

mod sealed {
    pub trait Sealed {}
}
