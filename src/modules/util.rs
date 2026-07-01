

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
