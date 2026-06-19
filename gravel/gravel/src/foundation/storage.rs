use core::mem::MaybeUninit;

use gravel_sys::foundation::storage as sys;

use crate::SysResult;

pub fn exists(key: u32) -> bool {
    unsafe { sys::persist_exists(key) }
}

pub fn get_size(key: u32) -> i32 {
    unsafe { sys::persist_get_size(key) }
}

pub fn read(key: u32, buffer: &mut [u8]) -> Result<(), sys::StatusCode> {
    unsafe { sys::persist_read_data(key, buffer.as_mut_ptr(), buffer.len()) }.into_nice()
}

pub fn read_uninit(key: u32, buffer: &mut [MaybeUninit<u8>]) -> Result<(), sys::StatusCode> {
    unsafe { sys::persist_read_data(key, buffer.as_mut_ptr().cast(), buffer.len()) }.into_nice()
}

pub fn write(key: u32, data: &[u8]) -> Result<(), sys::StatusCode> {
    unsafe { sys::persist_write_data(key, data.as_ptr(), data.len()) }.into_nice()
}

impl crate::SysResult for sys::StatusCode {
    type NiceErr = sys::StatusCode;
    fn into_nice(self) -> Result<(), Self::NiceErr> {
        if self.0 >= 0 {
            Ok(())
        } else {
            Err(self)
        }
    }
}
