pub mod uuid {
    unsafe extern "C" {
        pub fn uuid_equal(uu1: *const Uuid, uu2: *const Uuid) -> bool;
        pub fn uuid_to_string(uuid: *const Uuid, buffer: *mut core::ffi::c_char);
    }

    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    #[repr(C)]
    pub struct Uuid(pub [u8; 16]);
}
