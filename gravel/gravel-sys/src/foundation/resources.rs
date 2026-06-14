use crate::util::Opaque;

pub type ResHandle = *const u8;

unsafe extern "C" {
    pub fn resource_get_handle(resource_id: u32) -> ResHandle;
}

#[repr(C)]
pub struct Resource(Opaque);
