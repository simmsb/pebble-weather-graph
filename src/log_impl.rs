use core::fmt::{self, Write};

use gravel::foundation::logging::LogLevel;

use crate::{SingleCoreCell, modules::util::CStringWriter};

struct LogBuffers {
    path: heapless::CString<32, u8>,
    buf: heapless::CString<128, u8>,
}

static LOG_BUFFERS: SingleCoreCell<Option<LogBuffers>> = SingleCoreCell::new(None);

pub fn log_at(level: LogLevel, file: &str, line: u32, args: fmt::Arguments<'_>) {
    let mut buffers_ref = LOG_BUFFERS.get_mut();
    let Some(LogBuffers { path, buf }) = buffers_ref.as_mut() else {
        return;
    };

    *path = heapless::CString::new();
    *buf = heapless::CString::new();

    let mut writer = CStringWriter { buf };
    let _ = writer.write_fmt(args);

    let mut path_writer = CStringWriter { buf: path };
    let _ = write!(path_writer, "{}", file);

    unsafe {
        gravel_sys::foundation::logging::app_log(
            level as u8,
            path.as_ptr().cast(),
            line as i32,
            buf.as_ptr().cast(),
        );
    }
}

pub fn init() {
    *LOG_BUFFERS.get_mut() = Some(LogBuffers {
        path: heapless::CString::new(),
        buf: heapless::CString::new(),
    });
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::log_impl::log_at(
            ::gravel::foundation::logging::LogLevel::Error,
            file!(),
            line!(),
            format_args!($($arg)*),
        )
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::log_impl::log_at(
            ::gravel::foundation::logging::LogLevel::Warning,
            file!(),
            line!(),
            format_args!($($arg)*),
        )
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::log_impl::log_at(
            ::gravel::foundation::logging::LogLevel::Info,
            file!(),
            line!(),
            format_args!($($arg)*),
        )
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::log_impl::log_at(
            ::gravel::foundation::logging::LogLevel::Debug,
            file!(),
            line!(),
            format_args!($($arg)*),
        )
    };
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        $crate::log_impl::log_at(
            ::gravel::foundation::logging::LogLevel::Verbose,
            file!(),
            line!(),
            format_args!($($arg)*),
        )
    };
}
