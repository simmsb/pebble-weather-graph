#![no_std]

#[macro_use]
mod util;
use util::Opaque;

pub mod foundation {
    pub mod alloy;
    pub mod app;
    pub mod data_logging;
    pub mod data_structures;
    pub mod dictation;
    pub mod dictionary;
    pub mod event {
        pub mod accelerometer;
        pub mod app_focus;
        pub mod battery_state;
        pub mod compass;
        pub mod connection;
        // pub mod health;
        pub mod tick_timer;
        pub mod touch;
        pub mod health;
    }
    pub mod exit_reason;
    pub mod i18n;
    pub mod launch_reason;
    pub mod logging;
    pub mod math;
    pub mod memory_management;
    // pub mod platform;
    pub mod resources;
    pub mod storage;
    // pub mod timer;
    // pub mod wakeup;
    // pub mod wall_time;
    // pub mod watch_info;
}

pub mod graphics;

pub mod ui {
    // pub mod animation;
    pub mod clicks;
    pub mod layer;
    // pub mod light;
    // pub mod preferences;
    // pub mod speaker;
    // pub mod unobstructed_area;
    // pub mod vibes;
    pub mod window;
    pub mod window_stack;
}

pub mod std_c;
