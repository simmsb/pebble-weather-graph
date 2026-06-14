use crate::foundation::app::message;
use crate::foundation::dictionary::{self, DictValue};

// TODO

pub trait AppSyncState {
    fn tuple_changed(&mut self, key: u32, new: DictValue, old: DictValue);
    fn error(
        &mut self,
        dict_error: Option<dictionary::Error>,
        message_error: Option<message::Error>,
    );
}
