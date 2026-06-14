use chrono::NaiveDateTime;

use gravel::graphics::types::GRect;
use gravel::graphics::{BLACK, CLEAR};
use gravel::prelude::*;
use gravel::ui::layer::{Layer, TextAlignment, TextLayer};
use heapless::CString;

use crate::format_cstr;
use crate::generated::resource_ids;
use crate::layout::TL_TIME_H;
use crate::modules::clock::{local_now, naive_from_pebble_tm};
use crate::modules::settings::Settings;
use gravel_sys::std_c::time::tm;

pub struct TimeLayer {
    container: Layer<()>,
    time_label: TextLayer,
    time_buf: CString<8, u8>,
}

impl TimeLayer {
    pub fn new(frame: GRect) -> Self {
        let container = Layer::new(frame);
        let w = frame.size.x;

        let resource =
            gravel::foundation::resources::get_handle(resource_ids::RESOURCE_ID_LM_MONO_48);
        let time_label = TextLayer::builder(GRect {
            origin: Default::default(),
            size: gravel::graphics::types::GSize { x: w, y: TL_TIME_H },
        })
        .custom_font(resource)
        .background_color(CLEAR)
        .text_color(BLACK)
        .alignment(TextAlignment::Center)
        .build()
        .expect("text layer");

        let mut time_buf = CString::<8, u8>::new();
        let tick_time = local_now();
        time_label.set_text(format_cstr!(&mut time_buf, "{}", tick_time.format("%H:%M")));

        container.add_child(&time_label.layer());

        Self {
            container,
            time_label,
            time_buf: CString::new(),
        }
    }

    pub fn layer(&self) -> &Layer<()> {
        &self.container
    }

    pub fn update(&mut self, tick_time: NaiveDateTime) {
        self.time_label.set_text(format_cstr!(
            &mut self.time_buf,
            "{}",
            tick_time.format("%H:%M")
        ));
        self.container.mark_dirty();
    }

    pub fn update_from_tm(&mut self, tick_time: &tm) {
        self.update(naive_from_pebble_tm(tick_time));
    }

    pub fn update_now(&mut self) {
        self.update(local_now());
    }
}
