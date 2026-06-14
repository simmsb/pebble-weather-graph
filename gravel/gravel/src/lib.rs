#![no_std]
#![feature(abort_immediate)]

extern crate alloc;

#[macro_use]
mod util;

mod allocator;

use gravel_sys::{
    foundation::{
        app::app_event_loop,
        event::{
            battery_state::battery_state_service_unsubscribe,
            connection::connection_service_unsubscribe, health::health_service_events_unsubscribe,
            tick_timer::tick_timer_service_unsubscribe,
        },
        exit_reason::{AppExitReason, exit_reason_set},
        logging::app_log,
    },
    ui::window_stack::window_stack_pop_all,
};
use util::*;

use core::{ffi::CStr, fmt::Write};
use core::{ffi::c_char, u8};

pub use gravel_sys;

use self::{
    foundation::logging::{self, LogLevel},
    graphics::{BABY_BLUE_EYES, BLACK, GREEN, SystemFont, WHITE},
    prelude::{LayerHandle, WindowHandle as _},
    ui::{
        layer::{TextLayer, TextLayerBuilder},
        window::Window,
        window_stack,
    },
};

pub mod foundation {
    pub mod app {
        pub fn event_loop() {
            unsafe { gravel_sys::foundation::app::app_event_loop() }
        }

        pub mod comm;
        pub mod glance;
        pub mod message;
        pub mod sync;
        pub mod worker;
    }

    pub mod data_logging;
    pub mod dictation;
    pub mod dictionary;
    pub mod event {
        pub mod accelerometer;
        pub mod app_focus;
        pub mod battery_state;
        pub mod compass;
        pub mod connection;
        // pub mod health;
        pub mod health;
        pub mod tick_timer;
        pub mod touch;
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

pub mod prelude {
    pub use crate::ui::layer::LayerHandle;
    pub use crate::ui::window::WindowHandle;
}

struct PanicMessageBuf {
    buf: [u8; 128],
    len: usize,
}

impl core::fmt::Write for PanicMessageBuf {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let space = self.buf.len().saturating_sub(self.len + 1);
        let take = s.len().min(space);
        self.buf[self.len..self.len + take].copy_from_slice(&s.as_bytes()[..take]);
        self.len += take;
        Ok(())
    }
}

fn log_panic(info: &core::panic::PanicInfo<'_>) {
    let mut msg = PanicMessageBuf {
        buf: [0; 128],
        len: 0,
    };
    let _ = msg.write_fmt(format_args!("{info}"));
    msg.buf[msg.len] = 0;
    unsafe {
        app_log(
            LogLevel::Error as u8,
            c"panic".as_ptr(),
            0,
            c"Message: %s".as_ptr(),
            msg.buf.as_ptr(),
        );
    }
}

// extern, no_mangle so we can set a breakpoint
// #[unsafe(no_mangle)]
// extern "C" fn trigger_panic() -> ! {
//     // loop {}
//     core::process::abort_immediate();
// }

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // let mut msg = PanicMessageBuf {
    //     buf: [0; 128],
    //     len: 0,
    // };
    // let _ = msg.write_fmt(format_args!("{info}"));
    // msg.buf[msg.len] = 0;

    // log!(Error, "Panic");
    log_panic(info);
    unsafe {
        exit_reason_set(AppExitReason::NotSpecified);
        window_stack::pop_all(false);
        // battery_state_service_unsubscribe();
        // connection_service_unsubscribe();
        // tick_timer_service_unsubscribe();
        // health_service_events_unsubscribe();

        // if let Some(window) = window_stack::pop(false) {
        //     window_stack::push(&window, false);
        //     window.set_background_color(BABY_BLUE_EYES);

        //     let root = window.root_layer();
        //     let bounds = root.bounds();

        //     if let Some(mut text_layer) = TextLayer::new(bounds) {
        //         text_layer.set_system_font(SystemFont::Gothic18);
        //         text_layer.set_text_color(BLACK);
        //         text_layer.set_background_color(GREEN);
        //         text_layer.set_text(CStr::from_bytes_with_nul_unchecked(&msg.buf));
        //         text_layer.layer().mark_dirty();
        //         root.add_child(&text_layer.layer());
        //         window_stack::push(&window, true);

        //         app_log(
        //             LogLevel::Error as u8,
        //             c"panic".as_ptr(),
        //             0,
        //             c"Panic shown message %s".as_ptr(),
        //             msg.buf.as_ptr(),
        //         );
        //     } else {
        //         app_log(
        //             LogLevel::Error as u8,
        //             c"panic".as_ptr(),
        //             0,
        //             c"Panic couldn't make text layer".as_ptr(),
        //         );
        //     };
        // } else {
        //     app_log(
        //         LogLevel::Error as u8,
        //         c"panic".as_ptr(),
        //         0,
        //         c"Panic had no window stack to use".as_ptr(),
        //     );
        // }

        app_event_loop();
    };
    loop {}
}
