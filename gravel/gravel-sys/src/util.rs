macro_rules! c_enum {
    (
        $vis:vis enum $T:ident : $inner:ty;
        $($variant:ident = $value:literal;)*
    ) => {
        #[repr(transparent)]
        #[derive(Debug, /* Default, */ Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        $vis struct $T(pub $inner);

        impl $T {
            $($vis const $variant: Self = Self($value);)*
        }
    };

    (
        $vis:vis enum $T:ident;
        $($variant:ident = $value:literal;)*
    ) => {
        c_enum! {
            $vis enum $T : core::ffi::c_int;
            $($variant = $value;)*
        }
    }
}

macro_rules! callbacks {
    ($(
        $vis:vis fn $func:ident ( $($arg:ident : $arg_ty:ty),* $(,)? ) $(-> $ret:ty)? ;
    )*) => {$(
        $vis type $func = unsafe extern "C" fn ( $($arg : $arg_ty),* ) $(-> $ret)? ;
    )*}
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct Opaque {
    _data: (),
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}
