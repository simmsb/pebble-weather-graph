unsafe extern "C" {
    pub fn connection_service_peek_pebble_app_connection() -> bool;
    pub fn connection_service_peek_pebblekit_connection() -> bool;
    pub fn connection_service_subscribe(conn_handlers: ConnectionHandlers);
    pub fn connection_service_unsubscribe();
}

#[repr(C)]
pub struct ConnectionHandlers {
    pub pebble_app_connection_handler: Option<ConnectionHandler>,
    pub pebblekit_connection_handler: Option<ConnectionHandler>,
}

pub type ConnectionHandler = extern "C" fn(connected: bool);
