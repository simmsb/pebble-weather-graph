use gravel::graphics::{BLACK, CLEAR, GTextOverflowMode, SystemFont, WHITE};
use gravel::prelude::*;
use gravel::ui::layer::{Layer, TextAlignment, TextLayer};
use gravel::{
    foundation::event::health,
    graphics::draw::{Context, align_rect, text_layout_size},
};
use gravel::{
    graphics::types::{GAlign, GRect, GSize},
    ui::layer::LayerRef,
};
use gravel_sys::{
    foundation::event::health::HealthMetric,
    ui::layer::{Layer as SysLayer, LayerUpdateProc},
};

use crate::ufmt_cstr;

const BORDER_H_PAD: i16 = 5;
const FONT_H: i16 = 18;
const LABEL_LEN: usize = 10;

#[pin_init::pin_data]
pub struct BiometricsData {
    #[pin]
    hr: TextLayer<LABEL_LEN>,
    #[pin]
    steps: TextLayer<LABEL_LEN>,
    font: SystemFont,
}

pub struct BiometricsLayer {
    container: Layer<BiometricsData>,
}

extern "C" fn update_proc(
    raw_layer: *mut SysLayer,
    ctx: *mut gravel_sys::graphics::types::GContext,
) {
    let mut layer = unsafe { LayerRef::<BiometricsData>::from_raw(raw_layer) };
    let bounds = layer.bounds();
    let data = unsafe { layer.user_data_mut() };
    let ctx = Context::from_raw(ctx);
    let font = data.font.load();

    let hr_text = data.hr.text();
    let steps_text = data.steps.text();

    let hr_size = text_layout_size(
        hr_text,
        font,
        bounds,
        GTextOverflowMode::Fill,
        TextAlignment::Left,
    );
    let steps_size = text_layout_size(
        steps_text,
        font,
        bounds,
        GTextOverflowMode::Fill,
        TextAlignment::Left,
    );

    let total_width = hr_size.x + steps_size.x + 2 * BORDER_H_PAD;

    let aligned = GRect {
        origin: Default::default(),
        size: GSize {
            x: total_width,
            y: bounds.size.y,
        },
    };
    let aligned = align_rect(aligned, &bounds, GAlign::Center);

    ctx.set_stroke_color(BLACK);
    ctx.set_antialiased(true);
    ctx.set_fill_color(BLACK);
    ctx.set_stroke_width(0);
    ctx.fill_rect(aligned, 6);
    ctx.set_antialiased(false);

    data.hr.set_frame(GRect {
        origin: gravel::graphics::types::GPoint {
            x: aligned.origin.x + BORDER_H_PAD,
            y: aligned.origin.y,
        },
        size: hr_size,
    });
    data.steps.set_frame(GRect {
        origin: gravel::graphics::types::GPoint {
            x: aligned.origin.x + hr_size.x + BORDER_H_PAD,
            y: aligned.origin.y,
        },
        size: steps_size,
    });
}

impl BiometricsLayer {
    pub fn new(frame: GRect) -> Self {
        let w = frame.size.x;
        let font = SystemFont::Gothic18;

        let mut hr = TextLayer::new(GRect {
            origin: Default::default(),
            size: GSize { x: w, y: FONT_H },
        })
        .unwrap();
        hr.set_system_font(font)
            .set_background_color(CLEAR)
            .set_text_color(WHITE);

        let mut steps = TextLayer::new(GRect {
            origin: Default::default(),
            size: GSize { x: w, y: FONT_H },
        })
        .unwrap();
        steps
            .set_system_font(font)
            .set_background_color(CLEAR)
            .set_text_color(WHITE);

        let container =
            Layer::<BiometricsData>::new_with_data(frame, BiometricsData { hr, steps, font });
        container.add_child(&container.user_data().hr.layer());
        container.add_child(&container.user_data().steps.layer());
        container.set_update_proc(update_proc as LayerUpdateProc);

        Self { container }
    }

    pub fn layer(&self) -> &Layer<BiometricsData> {
        &self.container
    }

    pub fn update(&mut self) {
        let mut data = unsafe { self.container.user_data_mut() };
        let hr = health::peek(HealthMetric::HEALTH_METRIC_HEART_RATE_BPM);
        let steps: i32 = health::sum_today(HealthMetric::HEALTH_METRIC_STEP_COUNT);

        data.as_mut().project().hr.update_text(|s| {
            ufmt_cstr!(s, "H {} / ", hr);
        });
        data.as_mut().project().steps.update_text(|s| {
            ufmt_cstr!(s, "S {}", steps);
        });

        self.container.mark_dirty();
    }
}
