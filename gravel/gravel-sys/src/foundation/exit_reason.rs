unsafe extern "C" {
    pub fn exit_reason_set(exit_reason: AppExitReason);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum AppExitReason {
    NotSpecified,
    PerformedSuccessfully,
}
