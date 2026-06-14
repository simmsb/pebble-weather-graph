use alloc::boxed::Box;

pub(crate) unsafe fn drop_raw_box<T>(ptr: *mut T) {
    if !ptr.is_null() {
        unsafe { drop(Box::from_raw(ptr)) }
    }
}

pub(crate) trait SysResult {
    type NiceErr;
    fn into_nice(self) -> Result<(), Self::NiceErr>;
}

macro_rules! error_enum {
    (
        $vis:vis enum $nice:ident : $sys:path;
        $($nice_variant:ident = $sys_variant:ident;)*
        $(@ $custom_variant:ident = $custom_value:literal;)*
    ) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(i32)]
        $vis enum $nice {
            $($nice_variant = <$sys>::$sys_variant.0,)*
            $($custom_variant = $custom_value,)*
            Other(core::ffi::c_int),
        }

        impl SysResult for $sys {
            type NiceErr = $nice;
            fn into_nice(self) -> Result<(), Self::NiceErr> {
                $(const $sys_variant: core::ffi::c_int = <$sys>::$sys_variant.0;)*

                match self.0 {
                    0 => Ok(()),
                    $($sys_variant => Err($nice::$nice_variant),)*
                    n => Err($nice::Other(n)),
                }
            }
        }
    };
}

macro_rules! nice_enum {
    (
        $vis:vis enum $nice:ident : $sys:path;
        $($nice_variant:ident = $sys_variant:ident;)*
    ) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(i32)]
        $vis enum $nice {
            $($nice_variant = <$sys>::$sys_variant.0,)*
            Other(core::ffi::c_int),
        }

        impl From<$sys> for $nice {
            fn from(value: $sys) -> Self {
                $(const $sys_variant: core::ffi::c_int = <$sys>::$sys_variant.0;)*
                match value.0 {
                    $($sys_variant => $nice::$nice_variant,)*
                    n => $nice::Other(n),
                }
            }
        }

        impl From<$nice> for $sys {
            fn from(value: $nice) -> Self {
                match value {
                    $($nice::$nice_variant => <$sys>::$sys_variant,)*
                    $nice::Other(n) => $sys(n),
                }
            }
        }
    }
}
