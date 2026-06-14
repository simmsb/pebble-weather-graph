use gravel::graphics::types::{GAlign, GRect, GSize};
use gravel::graphics::{BLACK, CLEAR, GTextOverflowMode, SystemFont, WHITE};
use gravel::prelude::*;
use gravel::ui::layer::{Layer, TextAlignment, TextLayer, TextLayerRef};
use gravel::{
    foundation::event::health,
    graphics::draw::{Context, align_rect, text_layout_size},
};
use gravel_sys::{
    foundation::event::health::{HealthEventType, HealthMetric},
    ui::layer::{Layer as SysLayer, LayerUpdateProc, layer_get_data},
};
use heapless::CString;

use crate::format_cstr;

const BORDER_H_PAD: i16 = 5;
const FONT_H: i16 = 18;
const LABEL_LEN: usize = 10;

pub struct BiometricsData {
    hr: TextLayerRef,
    steps: TextLayerRef,
    font: SystemFont,
}

pub struct BiometricsLayer {
    container: Layer<BiometricsData>,
    hr: TextLayer,
    steps: TextLayer,
    hr_buf: CString<LABEL_LEN>,
    steps_buf: CString<LABEL_LEN>,
}

extern "C" fn update_proc(layer: *mut SysLayer, ctx: *mut gravel_sys::graphics::types::GContext) {
    let data = unsafe { &*layer_get_data(layer).cast::<BiometricsData>() };
    let bounds = unsafe { gravel_sys::ui::layer::layer_get_bounds(layer) };
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

        let hr = TextLayer::builder(GRect {
            origin: Default::default(),
            size: GSize { x: w, y: FONT_H },
        })
        .system_font(font)
        .background_color(CLEAR)
        .text_color(WHITE)
        .build()
        .expect("hr text layer");

        let steps = TextLayer::builder(GRect {
            origin: Default::default(),
            size: GSize { x: w, y: FONT_H },
        })
        .system_font(font)
        .background_color(CLEAR)
        .text_color(WHITE)
        .build()
        .expect("steps text layer");

        let container = Layer::<BiometricsData>::new_with_data(
            frame,
            BiometricsData {
                hr: hr.as_ref(),
                steps: steps.as_ref(),
                font,
            },
        );
        container.add_child(&hr.layer());
        container.add_child(&steps.layer());
        container.set_update_proc(update_proc as LayerUpdateProc);

        Self {
            container,
            hr,
            steps,
            hr_buf: CString::new(),
            steps_buf: CString::new(),
        }
    }

    pub fn layer(&self) -> &Layer<BiometricsData> {
        &self.container
    }

    pub fn update(&mut self) {
        let hr = health::peek(HealthMetric::HEALTH_METRIC_HEART_RATE_BPM);
        let steps = health::peek(HealthMetric::HEALTH_METRIC_STEP_COUNT);
        self.hr
            .set_text(format_cstr!(&mut self.hr_buf, "H {} / ", hr));
        self.steps
            .set_text(format_cstr!(&mut self.steps_buf, "S {}", steps));

        self.container.mark_dirty();
        self.hr.layer().mark_dirty();
        self.steps.layer().mark_dirty();
    }
}
