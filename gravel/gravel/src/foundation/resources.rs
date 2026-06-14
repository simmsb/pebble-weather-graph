use gravel_sys::foundation::resources as sys;

pub type ResHandle = sys::ResHandle;

pub fn get_handle(resource_id: u32) -> ResHandle {
    unsafe { sys::resource_get_handle(resource_id) }
}
