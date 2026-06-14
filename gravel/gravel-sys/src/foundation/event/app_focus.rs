unsafe extern "C" {
    pub fn app_focus_service_subscribe_handlers(handlers: AppFocusHandlers);
    pub fn app_focus_service_subscribe(handler: extern "C" fn(in_focus: bool));
    pub fn app_focus_service_unsubscribe();
}

#[repr(C)]
pub struct AppFocusHandlers {
    pub will_focus: Option<extern "C" fn(in_focus: bool)>,
    pub did_focus: Option<extern "C" fn(in_focus: bool)>,
}
