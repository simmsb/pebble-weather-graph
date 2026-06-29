#![no_std]
#![feature(integer_casts)]
#![feature(integer_widen_truncate)]
#![feature(impl_trait_in_assoc_type)]
#![feature(trait_alias)]
#![feature(atomic_ptr_null)]
#![feature(type_alias_impl_trait)]
#![feature(maybe_uninit_as_bytes)]

mod layers;
mod layout;
mod modules;

use crate::layout::{
    TL_BIOMETRICS_H, TL_BIOMETRICS_Y_PAD, TL_TIME_H, TL_TIME_Y_TOP, TL_TOP_BAR_H, TL_TOP_BAR_Y_TOP,
    TL_WEATHER_Y_PAD,
};

pub mod messages {
    #![allow(clippy::all)]
    #![allow(nonstandard_style, unused, irrefutable_let_patterns)]
    include!(concat!(env!("OUT_DIR"), "/messages.rs"));
}

use pebble_async::{bindings::GRect, window};

pebble_async::main!(async_main);

// fn init(s: embassy_executor::Spawner) {
//     let token = async_main(unsafe { pebble_async::PebbleServices::steal() }, s).expect("Task spawn failed?");
//     s.spawn(token);
// }
// #[unsafe(no_mangle)]
// pub extern "C" fn main() {
//     pebble_async::debug!("Starting up");
//     pebble_async::executor::init();
//     pebble_async::executor::run(init);
// }

#[embassy_executor::task]
async fn async_main(services: pebble_async::PebbleServices, spawner: embassy_executor::Spawner) {
    async_main_(services, spawner).await;
}

async fn async_main_(
    mut services: pebble_async::PebbleServices,
    _spawner: embassy_executor::Spawner,
) {
    pebble_async::info!("Async main called!");
    window::with_window(async |mut h| {
        pin_init::stack_pin_init!(let app_messages = services.app_messages.open(2500, 64));

        let root_layer = h.root_layer();
        let bounds = root_layer.bounds();
        pebble_async::info!("Window bounds: {:?}", bounds);

        let w = bounds.size.w;
        let mut y = 0;

        let top_bar_layer = layers::top_bar::top_bar_layer(
            root_layer
                .new_child(GRect::new(0, TL_TOP_BAR_Y_TOP, w, TL_TOP_BAR_H))
                .unwrap(),
        );

        let time_layer = layers::time::time_layer(
            root_layer
                .new_child(GRect::new(0, TL_TIME_Y_TOP, w, TL_TIME_H))
                .unwrap(),
        );

        y += TL_TIME_Y_TOP + TL_TIME_H;
        y += TL_BIOMETRICS_Y_PAD;

        let biometrics_layer = layers::biometrics::biometrics_layer(
            root_layer
                .new_child(GRect::new(0, y, w, TL_BIOMETRICS_H))
                .unwrap(),
        );
        y += TL_BIOMETRICS_H;

        y += TL_WEATHER_Y_PAD;

        let weather_layer = layers::weather::weather_layer(
            root_layer
                .new_child(GRect::new(0, y, w, bounds.size.h - y))
                .unwrap(),
            &app_messages,
        );

        embassy_futures::join::join4(top_bar_layer, time_layer, biometrics_layer, weather_layer)
            .await;
    })
    .await
    .unwrap();
}
