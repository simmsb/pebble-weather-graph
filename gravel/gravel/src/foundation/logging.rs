// #[macro_export]
// macro_rules! log {
//     ($level:ident, $fmt:literal $(, $args:expr)* $(,)?) => {
//         unsafe { $crate::gravel_sys::foundation::logging::app_log(
//             $crate::foundation::logging::LogLevel::$level as u8,
//             concat!(file!(), "\0").as_ptr().cast(),
//             line!() as i32,
//             concat!($fmt, "\0").as_ptr(),
//             $($args),*
//         ) }
//     };

//     ($level:literal, $fmt:literal $(, $args:expr)* $(,)?) => {
//         unsafe { $crate::gravel_sys::foundation::logging::app_log(
//             $level,
//             concat!(file!(), "\0").as_ptr().cast(),
//             line!() as i32,
//             concat!($fmt, "\0").as_ptr(),
//             $($args),*
//         ) }
//     };
// }

#[repr(u8)]
pub enum LogLevel {
    Error = 1,
    Warning = 50,
    Info = 100,
    Debug = 200,
    Verbose = 255,
}
