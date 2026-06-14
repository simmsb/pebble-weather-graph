unsafe extern "C" {
    pub fn moddable_createMachine(creation: *const ModdableCreationRecord);
}

#[repr(C)]
pub struct ModdableCreationRecord {
    pub record_size: u32,
    pub stack: u32,
    pub slot: u32,
    pub chunk: u32,
    pub flags: u32,
}

pub const CREATION_FLAG_LOG_INSTRUMENTATION: u32 = 1 << 0;
