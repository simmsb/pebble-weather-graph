#![no_std]
#![feature(maybe_uninit_as_bytes)]

extern crate alloc;

mod generated;
mod layers;
mod layout;
mod log_impl;
mod modules;

use core::cell::{Ref, RefCell, RefMut};

// use alloc::{boxed::Box, sync::Arc};
use gravel::foundation::dictionary::DictReader;
use gravel::foundation::event::battery_state::{self, BatteryChargeState};
use gravel::foundation::event::connection::{self, ConnectionHandlers};
use gravel::foundation::event::tick_timer::{self, TimeUnits};
use gravel::foundation::{
    app::message::{self, Error as MsgError, MessageHandler},
    event::health,
};
use gravel::graphics::WHITE;
use gravel::graphics::types::GRect;
use gravel::prelude::*;
use gravel::ui::window::{Window, WindowHandlers, WindowRef};
use gravel::ui::window_stack;
use gravel_sys::foundation::event::health::HealthEventType;

use crate::generated::message_keys;
use crate::layers::*;
use crate::layout::{
    TL_BIOMETRICS_H, TL_BIOMETRICS_Y_PAD, TL_TIME_H, TL_TIME_Y_TOP, TL_TOP_BAR_H, TL_TOP_BAR_Y_TOP,
    TL_WEATHER_Y_PAD,
};
use crate::modules::settings::Settings;

use self::modules::weather::Weather;

pub mod messages {
    #![allow(clippy::all)]
    #![allow(nonstandard_style, unused, irrefutable_let_patterns)]
    include!(concat!(env!("OUT_DIR"), "/messages.rs"));
}

pub struct SingleCoreCell<T> {
    value: RefCell<T>,
}

impl<T> SingleCoreCell<T> {
    pub const fn new(value: T) -> Self {
        Self {
            value: RefCell::new(value),
        }
    }

    pub fn get<'a>(&'a self) -> Ref<'a, T> {
        self.value.borrow()
    }

    pub fn get_mut<'a>(&'a self) -> RefMut<'a, T> {
        self.value.borrow_mut()
    }
}

// pebble apps are single threaded and non-reentrant I hope?
unsafe impl<T> Send for SingleCoreCell<T> {}
unsafe impl<T> Sync for SingleCoreCell<T> {}

pub struct AppState {
    pub time_layer: TimeLayer,
    pub top_bar: TopBarLayer,
    pub biometrics: BiometricsLayer,
    pub weather_layer: WeatherLayer,
    pub weather: Weather,
    pub settings: Settings,
    #[allow(dead_code)] // used for drop glue
    handlers: message::RegisteredHandlers,
}

pub static APP_STATE: SingleCoreCell<Option<AppState>> = SingleCoreCell::new(None);

struct AppWindowHandlers;

impl WindowHandlers<()> for AppWindowHandlers {
    extern "C" fn load(window: WindowRef<()>) {
        let root = window.root_layer();
        let bounds = root.bounds();
        let w = bounds.size.x;

        let mut y: i16 = 0;

        let mut time_layer = TimeLayer::new(GRect {
            origin: gravel::graphics::types::GPoint {
                x: 0,
                y: TL_TIME_Y_TOP,
            },
            size: gravel::graphics::types::GSize { x: w, y: TL_TIME_H },
        });
        root.add_child(time_layer.layer());
        y += TL_TIME_Y_TOP + TL_TIME_H;

        let mut top_bar = TopBarLayer::new(GRect {
            origin: gravel::graphics::types::GPoint {
                x: 0,
                y: TL_TOP_BAR_Y_TOP,
            },
            size: gravel::graphics::types::GSize {
                x: w,
                y: TL_TOP_BAR_H,
            },
        });
        root.add_child(top_bar.layer());

        y += TL_BIOMETRICS_Y_PAD;
        let mut biometrics = BiometricsLayer::new(GRect {
            origin: gravel::graphics::types::GPoint { x: 0, y },
            size: gravel::graphics::types::GSize {
                x: w,
                y: TL_BIOMETRICS_H,
            },
        });
        root.add_child(biometrics.layer());
        y += TL_BIOMETRICS_H;

        y += TL_WEATHER_Y_PAD;
        let weather_layer = WeatherLayer::new(GRect {
            origin: gravel::graphics::types::GPoint { x: 0, y },
            size: gravel::graphics::types::GSize {
                x: w,
                y: bounds.size.y - y,
            },
        });
        root.add_child(weather_layer.layer());

        let handlers = message::register(MsgHandler);
        let _ = message::open(2500, 64);

        request_weather();
        time_layer.update_now();
        biometrics.update();
        top_bar.peek();

        *APP_STATE.get_mut() = Some(AppState {
            time_layer,
            top_bar,
            biometrics,
            weather_layer,
            weather: Weather::new(),
            settings: Settings::init(),
            handlers,
        });
    }

    extern "C" fn appear(_window: WindowRef) {}

    extern "C" fn disappear(_window: WindowRef) {}

    extern "C" fn unload(_window: WindowRef) {
        let Some(y) = APP_STATE.get_mut().take() else {
            return;
        };
        drop(y);
    }
}

struct MsgHandler;

#[unsafe(no_mangle)]
pub extern "C" fn handle_inbox_received(reader: &mut DictReader) {
    let mut app_state_ref = APP_STATE.get_mut();
    let Some(app_state) = app_state_ref.as_mut() else {
        return;
    };
    app_state.settings.apply_from_message(reader);
    if app_state.weather.update_from_message(reader) {
        app_state.weather_layer.update();
    }
}

impl MessageHandler for MsgHandler {
    fn inbox_received(&mut self, reader: &mut DictReader) {
        handle_inbox_received(reader);
    }

    fn outbox_sent(&mut self, _message: &mut DictReader) {}

    fn inbox_dropped(&mut self, _reason: MsgError) {
        crate::warn!("Inbox dropped");
    }

    fn outbox_failed(&mut self, _message: &mut DictReader, _reason: MsgError) {}
}

struct TickHandler;

impl tick_timer::TickHandler for TickHandler {
    fn handle(time: &gravel_sys::std_c::time::tm, units_changed: TimeUnits) {
        let mut app_state_ref = APP_STATE.get_mut();
        let Some(app_state) = app_state_ref.as_mut() else {
            return;
        };
        app_state.time_layer.update_from_tm(time);

        if (units_changed.0 & TimeUnits::HOUR.0) != 0 {
            request_weather();
        }
    }
}

struct ConnHandlers;

impl ConnectionHandlers for ConnHandlers {
    extern "C" fn pebble_app(connected_in: bool) {
        let mut app_state_ref = APP_STATE.get_mut();
        let Some(app_state) = app_state_ref.as_mut() else {
            return;
        };
        let connected = connection::peek_pebble_app();
        crate::debug!("Connection callback: {connected_in}, peek: {connected}");
        app_state.top_bar.set_bt(connected);
    }
}

fn request_weather() {
    let _ = message::send(|dict| {
        dict.u8(message_keys::MESSAGE_KEY_WEATHER_REQUEST(), 1)
            .map(|_| ())
    });
}

extern "C" fn on_battery(state: BatteryChargeState) {
    let mut app_state_ref = APP_STATE.get_mut();
    let Some(app_state) = app_state_ref.as_mut() else {
        return;
    };
    crate::debug!("Battery callback: {state:?}");
    app_state.top_bar.set_battery(state);
}

extern "C" fn on_health(_event: HealthEventType, _contex: *mut core::ffi::c_void) {
    let mut app_state_ref = APP_STATE.get_mut();
    let Some(app_state) = app_state_ref.as_mut() else {
        return;
    };
    app_state.biometrics.update();
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> isize {
    log_impl::init();

    let window = Window::new();
    window.set_background_color(gravel::graphics::WHITE);
    window.set_window_handlers::<AppWindowHandlers>();
    window_stack::push(&window, true);

    tick_timer::subscribe::<TickHandler>(TimeUnits::MINUTE);
    battery_state::subscribe(on_battery);
    health::subscribe(on_health);
    connection::subscribe::<ConnHandlers>();

    crate::info!("Hello world");

    gravel::foundation::app::event_loop();
    0
}
