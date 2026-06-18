use gravel::foundation::event::{
    battery_state::{self, BatteryChargeState},
    connection,
};
use gravel::graphics::draw::Context;
use gravel::graphics::types::{GPoint, GRect};
use gravel::graphics::{BLACK, GREEN};
use gravel::prelude::*;
use gravel::ui::layer::Layer;
use gravel_sys::ui::layer::{Layer as SysLayer, LayerUpdateProc, layer_get_data};

pub struct TopBarData {
    bluetooth_on: bool,
    battery: BatteryChargeState,
}

pub struct TopBarLayer {
    container: Layer<TopBarData>,
}

extern "C" fn update_proc(layer: *mut SysLayer, ctx: *mut gravel_sys::graphics::types::GContext) {
    let data = unsafe { &*layer_get_data(layer).cast::<TopBarData>() };
    let bounds = unsafe { gravel_sys::ui::layer::layer_get_bounds(layer) };
    let ctx = Context::from_raw(ctx);

    ctx.set_stroke_color(BLACK);
    ctx.set_stroke_width(1);

    if data.bluetooth_on {
        let x = 5;
        let y = 1;
        let w = 6;
        let h = 10;
        let cross_offset = 2;

        ctx.set_antialiased(false);
        ctx.draw_line(
            GPoint {
                x,
                y: y + cross_offset,
            },
            GPoint {
                x: x + w,
                y: y + h - cross_offset,
            },
        );
        ctx.draw_line(
            GPoint {
                x: x + w,
                y: y + h - cross_offset,
            },
            GPoint {
                x: x + w / 2,
                y: y + h + 1,
            },
        );
        ctx.draw_line(
            GPoint {
                x: x + w / 2,
                y: y + h,
            },
            GPoint { x: x + w / 2, y },
        );
        ctx.draw_line(
            GPoint {
                x: x + w / 2,
                y: y - 1,
            },
            GPoint {
                x: x + w,
                y: y + cross_offset,
            },
        );
        ctx.draw_line(
            GPoint {
                x: x + w,
                y: y + cross_offset,
            },
            GPoint {
                x,
                y: y + h - cross_offset,
            },
        );
    }

    let w = 12;
    let h = 7;
    let x = bounds.size.x - (w + 7);
    let y = 0;
    let bar_h = 3;
    let bar_inset = (h - bar_h) / 2;
    let num_bars = (w - 2) / 2;
    let pct_per_bar = 100 / (num_bars + 2);

    ctx.set_antialiased(false);
    if data.battery.is_charging {
        ctx.set_stroke_color(GREEN);
    }

    ctx.draw_line(GPoint { x, y }, GPoint { x, y: y + h });
    ctx.draw_line(GPoint { x, y: y + h }, GPoint { x: x + w, y: y + h });
    ctx.draw_line(GPoint { x: x + w, y: y + h }, GPoint { x: x + w, y });
    ctx.draw_line(GPoint { x: x + w, y }, GPoint { x, y });
    ctx.draw_line(
        GPoint {
            x: x + w + 1,
            y: y + bar_inset,
        },
        GPoint {
            x: x + w + 1,
            y: y + h - bar_inset,
        },
    );

    let pc = data.battery.charge_percent;
    for i in 0..num_bars {
        if pc >= (pct_per_bar * (i + 1)) as u8 {
            ctx.draw_line(
                GPoint {
                    x: x + 2 + i * 2,
                    y: y + bar_inset,
                },
                GPoint {
                    x: x + 2 + i * 2,
                    y: y + h - bar_inset,
                },
            );
        }
    }
}

impl TopBarLayer {
    pub fn new(frame: GRect) -> Self {
        let container = Layer::<TopBarData>::new_with_data(
            frame,
            TopBarData {
                bluetooth_on: false,
                battery: BatteryChargeState {
                    charge_percent: 0,
                    is_charging: false,
                    is_plugged: false,
                },
            },
        );
        container.set_update_proc(update_proc as LayerUpdateProc);
        Self { container }
    }

    pub fn layer(&self) -> &Layer<TopBarData> {
        &self.container
    }

    pub fn set_bt(&mut self, on: bool) {
        unsafe {
            self.container.user_data_mut().bluetooth_on = on;
        }
        self.container.mark_dirty();
    }

    pub fn set_battery(&mut self, state: BatteryChargeState) {
        unsafe {
            self.container.user_data_mut().battery = state;
        }
        self.container.mark_dirty();
    }

    pub fn peek(&mut self) {
        let battery = battery_state::peek();
        let bt = connection::peek_pebble_app();

        self.set_battery(battery);
        self.set_bt(bt);
    }
}
