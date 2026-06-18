use core::ffi::CStr;
use core::fmt::{self, Write};

use gravel::foundation::dictionary::{DictReader, DictValue};
use heapless::{CString, LenType};

pub struct CStringWriter<'a, const N: usize, T: LenType = usize> {
    pub buf: &'a mut CString<N, T>,
}

impl<const N: usize, T: LenType> Write for CStringWriter<'_, N, T> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let used = self.buf.as_bytes().len();
        let space = N.saturating_sub(1).saturating_sub(used);
        let take = s.len().min(space);
        if take > 0 {
            let _ = self.buf.extend_from_bytes(&s.as_bytes()[..take]);
        }
        Ok(())
    }
}

/// Format into a null-terminated buffer using Rust formatting (`format_args!` / `write!`).
pub fn format_cstr<'a, const N: usize, T: LenType>(
    buf: &'a mut CString<N, T>,
    args: fmt::Arguments<'_>,
) -> &'a CStr {
    *buf = CString::new();
    let mut writer = CStringWriter { buf };
    let _ = writer.write_fmt(args);
    buf.as_c_str()
}

#[macro_export]
macro_rules! format_cstr {
    ($buf:expr, $($arg:tt)*) => {
        $crate::modules::util::format_cstr($buf, format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! ufmt_cstr {
    ($buf:expr, $($arg:tt)*) => {
        {
            *$buf = ::heapless::CString::new();
            let _ = ::ufmt::uwrite!($buf, $($arg)*);
            $buf.as_c_str()
        }
    };
}

pub fn find_i32(reader: &mut DictReader, key: u32) -> Option<i32> {
    unsafe {
        match reader.find(key) {
            Some(DictValue::I32(v)) => Some(v),
            Some(DictValue::I8(v)) => Some(v as i32),
            Some(DictValue::U8(v)) => Some(v as i32),
            Some(DictValue::U32(v)) => Some(v as i32),
            _ => None,
        }
    }
}
