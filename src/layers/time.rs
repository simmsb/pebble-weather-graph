use gravel::graphics::types::GRect;
use gravel::graphics::{BLACK, CLEAR};
use gravel::prelude::*;
use gravel::ui::layer::{Layer, TextAlignment, TextLayer};
use heapless::CString;

use crate::layout::TL_TIME_H;
use crate::modules::clock::{Datetime, local_now, timestamp_from_pebble_tm};
use crate::{generated::resource_ids, ufmt_cstr};
use gravel_sys::std_c::time::tm;

#[pin_init::pin_data]
pub struct TimeData {
    #[pin]
    time_label: TextLayer<12>,
}

pub struct TimeLayer {
    container: Layer<TimeData>,
}

impl TimeLayer {
    pub fn new(frame: GRect) -> Self {
        let w = frame.size.x;

        let resource =
            gravel::foundation::resources::get_handle(resource_ids::RESOURCE_ID_LM_MONO_48);
        let mut time_label = TextLayer::new(GRect {
            origin: Default::default(),
            size: gravel::graphics::types::GSize { x: w, y: TL_TIME_H },
        })
        .unwrap();

        time_label
            .set_custom_font(resource)
            .set_background_color(CLEAR)
            .set_text_color(BLACK)
            .set_alignment(TextAlignment::Center);

        let container = Layer::new_with_data(frame, TimeData { time_label });

        container.add_child(&container.user_data().time_label.layer());

        Self { container }
    }

    pub fn layer(&self) -> &Layer<TimeData> {
        &self.container
    }

    pub fn update(&mut self, tick_time: Datetime) {
        let data = unsafe { self.container.user_data_mut() };

        data.project().time_label.update_text(|s| {
            ufmt_cstr!(s, "{}:{}", tick_time.hour, tick_time.min);
        });
    }

    pub fn update_from_tm(&mut self, tick_time: &tm) {
        self.update(timestamp_from_pebble_tm(tick_time));
    }

    pub fn update_now(&mut self) {
        self.update(local_now());
    }
}
