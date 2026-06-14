unsafe extern "C" {
    pub fn app_worker_is_running() -> bool;
    pub fn app_worker_launch() -> AppWorkerResult;
    pub fn app_worker_kill() -> AppWorkerResult;
    pub fn app_worker_message_subscribe(handler: AppWorkerMessageHandler) -> bool;
    pub fn app_worker_message_unsubscribe() -> bool;
    pub fn app_worker_send_message(r#type: u8, data: *mut AppWorkerMessage);
}

#[repr(C)]
pub struct AppWorkerMessage(pub u16, pub u16, pub u16);

c_enum! {
    pub enum AppWorkerResult;
    SUCCESS = 0;
    NO_WORKER = 1;
    DIFFERENT_APP = 2;
    NOT_RUNNING = 3;
    ALREADY_RUNNING = 4;
    ASKING_CONFIRMATION = 5;
}

callbacks! {
    pub fn AppWorkerMessageHandler(r#type: u16, data: *mut AppWorkerMessage);
}
