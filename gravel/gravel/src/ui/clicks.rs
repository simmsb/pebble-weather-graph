use gravel_sys::ui::clicks as sys;

#[derive(Debug)]
#[repr(transparent)]
pub struct ClickRecognizerRef(pub sys::ClickRecognizerRef);

impl ClickRecognizerRef {
    pub fn number_of_clicks_counted(&self) -> u8 {
        unsafe { sys::click_number_of_clicks_counted(self.0) }
    }

    pub fn button_id(&self) -> ButtonId {
        unsafe { sys::click_recognizer_get_button_id(self.0) }.into()
    }

    pub fn is_repeating(&self) -> bool {
        unsafe { sys::click_recognizer_is_repeating(self.0) }
    }
}

nice_enum! {
    pub enum ButtonId: sys::ButtonId;
    Back = BACK;
    Up = UP;
    Select = SELECT;
    Down = DOWN;
}
