use core::cell::Cell;

use futures::StreamExt as _;
use heapless::CString;
use pebble_async::{
    IsLayer as _,
    bindings::{
        GAlign, GCornerMask, GRect, GSize, GTextAlignment, GTextOverflowMode, HealthMetric,
    },
    colour::{self, GColor},
    events,
    font::Font,
    graphics_context::GContext,
    layers::{Layer, TextLayer},
};

use crate::ufmt_cstr;

const BORDER_H_PAD: i16 = 5;
const FONT_H: i16 = 18;
const LABEL_LEN: usize = 10;

pub async fn biometrics_layer(container: Layer<'_>) {
    let bounds = container.bounds();
    let font = Font::get_system_font(pebble_async::bindings::FONT_KEY_GOTHIC_18);

    let mut hr_layer = container.new_child::<TextLayer>(bounds).unwrap();
    hr_layer.set_font(font);
    hr_layer.set_background_colour(colour::GColor::CLEAR);
    hr_layer.set_text_colour(colour::GColor::WHITE);

    let mut steps_layer = container.new_child::<TextLayer>(bounds).unwrap();
    steps_layer.set_font(font);
    steps_layer.set_background_colour(colour::GColor::CLEAR);
    steps_layer.set_text_colour(colour::GColor::WHITE);

    let aligned = Cell::new(GRect::new(0, 0, 0, 0));

    pin_init::stack_pin_init!(let container = container.with_update_proc(|_layer, mut ctx| {
        ctx.set_antialiased(true);
        ctx.set_stroke_width(0);
        ctx.set_fill_colour(GColor::BLACK);
        ctx.fill_rect(aligned.get(), 6, GCornerMask::GCornersAll);
    }));

    pin_init::stack_pin_init!(let health_stream = events::health::stream());

    loop {
        let mut hr_buf = CString::<12>::new();
        let mut steps_buf = CString::<12>::new();

        let hr = events::health::health_service_peek_current_value(
            HealthMetric::HealthMetricHeartRateBPM,
        );
        let steps = events::health::health_service_today(HealthMetric::HealthMetricStepCount);

        let hr_text = ufmt_cstr!(&mut hr_buf, "H {} / ", hr);
        let steps_text = ufmt_cstr!(&mut steps_buf, "S {}", steps);

        let bounds = container.bounds();
        let hr_size = GContext::get_text_content_size(
            hr_text,
            font,
            bounds,
            GTextOverflowMode::GTextOverflowModeFill,
            GTextAlignment::GTextAlignmentLeft,
        );
        let steps_size = GContext::get_text_content_size(
            steps_text,
            font,
            bounds,
            GTextOverflowMode::GTextOverflowModeFill,
            GTextAlignment::GTextAlignmentLeft,
        );

        let total_width = hr_size.w + steps_size.w + 2 * BORDER_H_PAD;

        aligned.set(GRect::new(0, 0, total_width, bounds.size.h).align(
            &bounds,
            GAlign::GAlignCenter,
            false,
        ));

        hr_layer.layer_mut().set_frame(
            hr_size.with_origin(aligned.get().origin.offset(GSize::new(BORDER_H_PAD, -3))),
        );
        steps_layer.layer_mut().set_frame(
            hr_size.with_origin(
                aligned
                    .get()
                    .origin
                    .offset(GSize::new(hr_size.w + BORDER_H_PAD, -3)),
            ),
        );

        let _hr_h = hr_layer.set_text(hr_text);
        let _steps_h = steps_layer.set_text(steps_text);

        container.mark_dirty();

        let _ = health_stream.next().await;
    }
}
