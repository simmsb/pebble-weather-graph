use gravel_sys::foundation::event::connection as sys;

pub fn peek_pebble_app() -> bool {
    unsafe { sys::connection_service_peek_pebble_app_connection() }
}

pub fn peek_pebblekit() -> bool {
    unsafe { sys::connection_service_peek_pebblekit_connection() }
}

pub fn subscribe<H: ConnectionHandlers>() {
    unsafe {
        sys::connection_service_subscribe(sys::ConnectionHandlers {
            pebble_app_connection_handler: Some(H::pebble_app),
            pebblekit_connection_handler: Some(H::pebblekit),
        })
    }
}

pub fn unsubscribe() {
    unsafe { sys::connection_service_unsubscribe() }
}

#[allow(unused_variables)]
pub trait ConnectionHandlers {
    extern "C" fn pebble_app(connected: bool) {}
    extern "C" fn pebblekit(connected: bool) {}
}
