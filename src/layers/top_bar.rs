use core::cell::Cell;

use futures::StreamExt;
use bordstein::{
    bindings::{BatteryChargeState, GPoint, GRect},
    colour::GColor,
    events,
    graphics_context::GContext,
    layers::Layer,
};

fn draw_bluetooth_icon(ctx: &mut GContext) {
    ctx.set_stroke_colour(GColor::BLACK);
    ctx.set_stroke_width(1);

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

fn draw_battery(ctx: &mut GContext, bounds: &GRect, state: &BatteryChargeState) {
    let w = 12;
    let h = 7;
    let x = bounds.size.w - (w + 7);
    let y = 0;
    let bar_h = 3;
    let bar_inset = (h - bar_h) / 2;
    let num_bars = (w - 2) / 2;
    let pct_per_bar = 100 / (num_bars + 2);

    ctx.set_stroke_width(1);
    ctx.set_antialiased(false);
    ctx.set_stroke_colour(GColor::BLACK);
    if state.is_charging {
        ctx.set_stroke_colour(GColor::GREEN);
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

    let pc = state.charge_percent;
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

pub async fn top_bar_layer(container: Layer<'_>) {
    let bounds = container.bounds();
    let bluetooth_on = Cell::new(events::connection::peek_pebble_app_connection());
    let battery_state = Cell::new(events::battery_state::battery_service_peek_current_value());

    pin_init::stack_pin_init!(let container = container
                              .with_update_proc(|_layer, mut ctx| {
                                  let bluetooth_on = bluetooth_on.get();
                                  let battery_state = battery_state.get();

                                  if bluetooth_on {
                                      draw_bluetooth_icon(&mut ctx);
                                  }

                                  draw_battery(&mut ctx, &bounds, &battery_state);
                              })
    );

    pin_init::stack_pin_init!(let battery_events = events::battery_state::stream());
    pin_init::stack_pin_init!(let connection_events = events::connection::stream());

    loop {
        container.mark_dirty();

        match embassy_futures::select::select(battery_events.next(), connection_events.next()).await
        {
            embassy_futures::select::Either::First(Some(x)) => {
                battery_state.set(x);
            }
            embassy_futures::select::Either::Second(Some(x)) => {
                bluetooth_on.set(x);
            }
            _ => {}
        }
    }
}
