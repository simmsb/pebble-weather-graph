use core::ffi::c_void;

unsafe extern "C" {
    pub fn click_number_of_clicks_counted(recognizer: ClickRecognizerRef) -> u8;
    pub fn click_recognizer_get_button_id(recognizer: ClickRecognizerRef) -> ButtonId;
    pub fn click_recognizer_is_repeating(recognizer: ClickRecognizerRef) -> bool;
}

c_enum! {
    pub enum ButtonId;
    BACK = 0;
    UP = 1;
    SELECT = 2;
    DOWN = 3;
    NUM_BUTTONS = 4;
}

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct ClickRecognizerRef(pub *mut c_void);

callbacks! {
    pub fn ClickHandler(recognizer: ClickRecognizerRef, context: *mut c_void);
    pub fn ClickConfigProvider(context: *mut c_void);
}
