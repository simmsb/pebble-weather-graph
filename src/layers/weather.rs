use core::ffi::CStr;

use gravel::graphics::draw::{self, Context};
use gravel::graphics::types::{GPoint, GRect};
use gravel::graphics::{
    BABY_BLUE_EYES, BLACK, CELESTE, CHROME_YELLOW, DARK_GRAY, ELECTRIC_ULTRAMARINE, GColor,
    LIGHT_GRAY, ORANGE, PASTEL_YELLOW, PICTON_BLUE, RED, SUNSET_ORANGE, VERY_LIGHT_BLUE, YELLOW,
};
use gravel::prelude::*;
use gravel::ui::layer::Layer;
use gravel_sys::graphics::fonts::FONT_KEY_GOTHIC_14;
use gravel_sys::graphics::gdraw::{GTextAlignment, GTextOverflowMode};
use gravel_sys::ui::layer::{Layer as SysLayer, LayerUpdateProc};
use heapless::CString;
use jiff::ToSpan as _;

use crate::modules::weather::{
    TL_WEATHER_SEGS_PER_HOUR, WEATHER_CITY_MAX_LEN, WEATHER_NUM_DAYS, Weather,
};
use crate::{APP_STATE, format_cstr};
use crate::{
    messages::{WeatherDaily, WeatherHourly, WeatherSegment},
    modules::clock::{local_now, utc_now},
};

pub const TL_WEATHER_HOURS_PER_PAGE: usize = 36;
pub const TL_WEATHER_SEGS_PER_PAGE: usize = TL_WEATHER_HOURS_PER_PAGE * TL_WEATHER_SEGS_PER_HOUR;
pub const TL_WEATHER_HOURLY_SEGS_PER_PAGE: usize = TL_WEATHER_HOURS_PER_PAGE;
pub const TL_WEATHER_CURRENT_HEIGHT: i16 = 20;
pub const TL_WEATHER_TEMP_BOTTOM_PAD: i16 = 22 + TL_WEATHER_CURRENT_HEIGHT;
pub const TL_WEATHER_WIDTH: usize = 200;

const MIN_TEMP: i8 = -10;
const MAX_TEMP: i8 = 35;
const SUN_LEVEL_MIN_TO_SHOW: u8 = 3;
const RAIN_CHANCE_MIN_TO_SHOW: u8 = 30;

pub struct WeatherLayer {
    container: Layer<()>,
}

fn colour_for_temperature(temp: i8) -> GColor {
    if temp >= 35 {
        RED
    } else if temp >= 30 {
        ORANGE
    } else if temp >= 25 {
        SUNSET_ORANGE
    } else if temp >= 22 {
        CHROME_YELLOW
    } else if temp >= 18 {
        PASTEL_YELLOW
    } else if temp >= 10 {
        CELESTE
    } else if temp >= 0 {
        PICTON_BLUE
    } else {
        ELECTRIC_ULTRAMARINE
    }
}

fn temp_line_height(bounds: &GRect, temp: i8) -> i16 {
    let range = (bounds.size.y - TL_WEATHER_TEMP_BOTTOM_PAD - 10) as i32;
    (((temp as i32).clamp(MIN_TEMP as i32, MAX_TEMP as i32) - MIN_TEMP as i32) * range
        / (MAX_TEMP - MIN_TEMP) as i32) as i16
}

fn draw_temp_line(ctx: &Context, bounds: &GRect, x: i16, temp: i8) {
    let height = temp_line_height(bounds, temp);
    let y0 = bounds.origin.y + (bounds.size.y - TL_WEATHER_TEMP_BOTTOM_PAD);
    let y1 = y0 - height;
    ctx.set_antialiased(false);
    ctx.set_stroke_color(colour_for_temperature(temp));
    ctx.set_stroke_width(1);
    ctx.draw_line(GPoint { x, y: y0 }, GPoint { x, y: y1 });
}

fn draw_temp_top_line(ctx: &Context, bounds: &GRect, x0: i16, x1: i16, temp0: i8, temp1: i8) {
    let bottom = bounds.origin.y + (bounds.size.y - TL_WEATHER_TEMP_BOTTOM_PAD);
    let y0 = bottom - temp_line_height(bounds, temp0);
    let y1 = bottom - temp_line_height(bounds, temp1);
    ctx.set_antialiased(true);
    ctx.set_stroke_color(BLACK);
    ctx.set_stroke_width(1);
    ctx.draw_line(GPoint { x: x0, y: y0 }, GPoint { x: x1, y: y1 });
}

fn cloud_cover_y(bounds: &GRect, cloud_cover: u8) -> (i16, i16) {
    let max_cloud_radius: i16 = 4;
    let cloud_radius = (cloud_cover as i16 * max_cloud_radius) / 7;
    let y_mid = bounds.origin.y + max_cloud_radius;
    (y_mid - cloud_radius, y_mid + cloud_radius + 1)
}

fn draw_cloud_line(ctx: &Context, bounds: &GRect, x: i16, cloud_cover: u8) {
    if cloud_cover < 1 {
        return;
    }
    let (y0, y1) = cloud_cover_y(bounds, cloud_cover);
    ctx.set_antialiased(false);
    ctx.set_stroke_color(DARK_GRAY);
    ctx.set_stroke_width(1);
    ctx.draw_line(GPoint { x, y: y0 }, GPoint { x, y: y1 });
}

fn draw_sun_line(ctx: &Context, bounds: &GRect, x: i16, sun_level: u8, temp: i8, cloud_cover: u8) {
    if sun_level < SUN_LEVEL_MIN_TO_SHOW {
        return;
    }
    let (_, cloud_y) = cloud_cover_y(bounds, cloud_cover);
    let bottom = bounds.origin.y + (bounds.size.y - TL_WEATHER_TEMP_BOTTOM_PAD);
    let y0 = bottom - (temp_line_height(bounds, temp) + 4);
    let y1 = cloud_y + 2;
    let colour = PASTEL_YELLOW;
    ctx.set_antialiased(false);
    ctx.set_stroke_width(1);

    if sun_level >= 6 {
        let dot_colour = YELLOW;
        for y in y1..=y0 {
            let this_colour = if (y & 1) == (x & 1) {
                dot_colour
            } else {
                colour
            };
            ctx.set_stroke_color(this_colour);
            ctx.draw_pixel(GPoint { x, y });
        }
    } else {
        ctx.set_stroke_color(colour);
        ctx.draw_line(GPoint { x, y: y0 }, GPoint { x, y: y1 });
    }
}

#[derive(Copy, Clone)]
enum RainPattern {
    P30,
    P60,
    P90,
}

fn draw_rain_line(
    ctx: &Context,
    bounds: &GRect,
    x: i16,
    rain_chance: u8,
    temp: i8,
    cloud_cover: u8,
) {
    if rain_chance < RAIN_CHANCE_MIN_TO_SHOW {
        return;
    }
    let (_, cloud_y) = cloud_cover_y(bounds, cloud_cover);
    let bottom = bounds.origin.y + (bounds.size.y - TL_WEATHER_TEMP_BOTTOM_PAD);
    let y0 = bottom - (temp_line_height(bounds, temp) + 4);
    let y1 = cloud_y + 2;

    let (pattern, colour) = if rain_chance >= 90 {
        (RainPattern::P90, VERY_LIGHT_BLUE)
    } else if rain_chance >= 60 {
        (RainPattern::P60, BABY_BLUE_EYES)
    } else {
        (RainPattern::P30, BABY_BLUE_EYES)
    };

    ctx.set_antialiased(false);
    ctx.set_stroke_color(colour);
    ctx.set_stroke_width(1);

    for y in y1..=y0 {
        let draw = match pattern {
            RainPattern::P30 => (x & 1) == 0 && ((y + x / 2) & 1) != 0,
            RainPattern::P60 => (y & 1) != (x & 1),
            RainPattern::P90 => (x & 1) != 0,
        };
        if draw {
            ctx.draw_pixel(GPoint { x, y });
        }
    }
}

fn draw_is_day(ctx: &Context, bounds: &GRect, x: i16, is_day: bool) {
    if !is_day {
        return;
    }
    let y = bounds.origin.y + (bounds.size.y - TL_WEATHER_TEMP_BOTTOM_PAD);
    ctx.set_antialiased(false);
    ctx.set_stroke_color(LIGHT_GRAY);
    ctx.set_stroke_width(1);
    ctx.draw_line(GPoint { x, y: y + 2 }, GPoint { x, y: y + 4 });
}

fn interpolate_segment(a: &WeatherSegment, b: &WeatherSegment, weight: i32) -> WeatherSegment {
    let w = TL_WEATHER_WIDTH as i32;
    WeatherSegment {
        temperature: (a.temperature as i32
            + (b.temperature as i32 - a.temperature as i32) * weight / w)
            as i8,
        sun_level: (a.sun_level as i32 + (b.sun_level as i32 - a.sun_level as i32) * weight / w)
            as u8,
        is_day: a.is_day,
    }
}

fn interpolate_hourly(a: &WeatherHourly, b: &WeatherHourly, weight: i32) -> WeatherHourly {
    let w = TL_WEATHER_WIDTH as i32;
    WeatherHourly {
        rain_amount: (a.rain_amount as i32
            + (b.rain_amount as i32 - a.rain_amount as i32) * weight / w)
            as u8,
        rain_chance: (a.rain_chance as i32
            + (b.rain_chance as i32 - a.rain_chance as i32) * weight / w)
            as u8,
        cloud_cover: (a.cloud_cover as i32
            + (b.cloud_cover as i32 - a.cloud_cover as i32) * weight / w)
            as u8,
    }
}

fn day_name(day: u8) -> &'static str {
    match day {
        0 | 7 => "SU",
        1 => "MO",
        2 => "TU",
        3 => "WE",
        4 => "TH",
        5 => "FR",
        6 => "SA",
        _ => "UNK",
    }
}

fn draw_midnight_marker_and_dates(
    ctx: &Context,
    bounds: &GRect,
    x: i16,
    temp: i8,
    day_left: &WeatherDaily,
    day_right: &WeatherDaily,
    is_first: bool,
) {
    let bottom = bounds.origin.y + (bounds.size.y - TL_WEATHER_TEMP_BOTTOM_PAD);
    let y0 = bounds.origin.y;
    let y1 = bottom - (temp_line_height(bounds, temp) + 8);
    let y2 = bounds.origin.y + (bounds.size.y - TL_WEATHER_TEMP_BOTTOM_PAD) + 2;
    let y3 = bounds.origin.y + (bounds.size.y - TL_WEATHER_CURRENT_HEIGHT);

    ctx.set_antialiased(false);
    ctx.set_stroke_color(LIGHT_GRAY);
    ctx.set_stroke_width(1);

    let mut y = y0;
    while y < y1 {
        ctx.draw_pixel(GPoint { x, y });
        y += 2;
    }
    y = y2;
    while y < y3 {
        ctx.draw_pixel(GPoint { x, y });
        y += 2;
    }

    if !is_first {
        return;
    }

    let width: i16 = 100;
    let text_pad: i16 = 5;
    let left_x = x - (width + text_pad - 1);
    let right_x = x + text_pad;
    let y = y2 + 2;

    let font = unsafe {
        gravel_sys::graphics::fonts::fonts_get_system_font(FONT_KEY_GOTHIC_14.as_ptr().cast())
    };

    let mut buf = CString::<32>::new();

    ctx.set_text_color(BLACK);
    ctx.draw_text(
        format_cstr!(
            &mut buf,
            "{} {}/{}°",
            day_name(day_left.day),
            day_left.temp_low,
            day_left.temp_high
        ),
        font,
        GRect {
            origin: GPoint { x: left_x, y },
            size: gravel::graphics::types::GSize { x: width, y: 16 },
        },
        GTextOverflowMode::Fill,
        GTextAlignment::Right,
    );
    ctx.draw_text(
        format_cstr!(
            &mut buf,
            "{} {}/{}°",
            day_name(day_right.day),
            day_right.temp_low,
            day_right.temp_high
        ),
        font,
        GRect {
            origin: GPoint { x: right_x, y: y },
            size: gravel::graphics::types::GSize { x: width, y: 16 },
        },
        GTextOverflowMode::Fill,
        GTextAlignment::Left,
    );
}

fn draw_current_infos(ctx: &Context, bounds: &GRect, weather: &Weather) {
    let line_y = bounds.origin.y + (bounds.size.y - TL_WEATHER_CURRENT_HEIGHT);
    let text_y = line_y + 1;

    ctx.set_stroke_width(1);
    ctx.set_stroke_color(LIGHT_GRAY);

    let mut x: i16 = 0;
    while (x as usize) < TL_WEATHER_WIDTH {
        ctx.draw_pixel(GPoint { x, y: line_y });
        x += 2;
    }

    let x_mid = bounds.origin.x + bounds.size.x / 2;
    let mut y = line_y;
    while y < bounds.origin.y + bounds.size.y {
        ctx.draw_pixel(GPoint { x: x_mid, y });
        y += 2;
    }

    let font = unsafe {
        gravel_sys::graphics::fonts::fonts_get_system_font(FONT_KEY_GOTHIC_14.as_ptr().cast())
    };

    let mut buf = CString::<{ WEATHER_CITY_MAX_LEN + 10 }>::new();
    let location = weather.location_name.as_c_str().to_str().unwrap_or("");
    ctx.set_text_color(BLACK);
    ctx.draw_text(
        format_cstr!(
            &mut buf,
            "{}° {}",
            weather.data.current_temperature,
            location
        ),
        font,
        GRect {
            origin: GPoint {
                x: bounds.origin.x,
                y: text_y,
            },
            size: gravel::graphics::types::GSize {
                x: bounds.size.x / 2,
                y: 16,
            },
        },
        GTextOverflowMode::Fill,
        GTextAlignment::Center,
    );

    let current_date = local_now().date();

    ctx.draw_text(
        format_cstr!(&mut buf, "{}", current_date.strftime("%a %b %d")),
        font,
        GRect {
            origin: GPoint {
                x: x_mid,
                y: text_y,
            },
            size: gravel::graphics::types::GSize {
                x: bounds.size.x / 2,
                y: 16,
            },
        },
        GTextOverflowMode::Fill,
        GTextAlignment::Center,
    );
}

fn line_is_on_midnight(now: jiff::civil::DateTime, segment_idx: usize) -> bool {
    let dt = now + 15.minutes() * segment_idx as i64;

    dt.hour() == 0 && dt.minute() == 0
}

extern "C" fn update_proc(layer: *mut SysLayer, ctx: *mut gravel_sys::graphics::types::GContext) {
    let app_state_ref = APP_STATE.get();
    let app_state = app_state_ref.as_ref().unwrap();

    let weather = &app_state.weather;
    let weather_data = &weather.data;
    let bounds = unsafe { gravel_sys::ui::layer::layer_get_bounds(layer) };
    let ctx: Context<'_> = Context::from_raw(ctx);

    // round to first 15 minute interval
    let now = utc_now()
        .round(
            jiff::TimestampRound::new()
                .smallest(jiff::Unit::Minute)
                .increment(15)
                .mode(jiff::RoundMode::Floor),
        )
        .unwrap();

    let now_dt = local_now()
        .round(
            jiff::civil::DateTimeRound::new()
                .smallest(jiff::Unit::Minute)
                .increment(15)
                .mode(jiff::RoundMode::Floor),
        )
        .unwrap();

    let now_ts = now.as_second() as u32;
    let offset_15 = ((now_ts - weather_data.report_start) / (15 * 60)) as usize;
    let offset_hour = ((now_ts - weather_data.report_start) / (60 * 60)) as usize;

    let mut prev = WeatherSegment::default();
    let mut prev_hr = WeatherHourly::default();
    let mut prev_x: i16 = 0;

    crate::info!(
        "Drawing weather, offset: {}, report_start: {}",
        offset_15,
        weather_data.report_start
    );

    // crate::debug!(
    //     "Weather data sizes: segs: {}, hours: {}, days: {}",
    //     weather_data.segments.len(),
    //     weather_data.hourly_segments.len(),
    //     weather_data.daily.len(),
    // );

    let mut drawn_midnight = false;

    for x in 0..TL_WEATHER_WIDTH {
        let x_i16 = x as i16;
        let scaled_15 = x * TL_WEATHER_SEGS_PER_PAGE;
        let scaled_hr = x * TL_WEATHER_HOURS_PER_PAGE;

        let segment_idx = offset_15 + scaled_15 / TL_WEATHER_WIDTH;
        let naive_next_idx = offset_15 + (scaled_15 + TL_WEATHER_SEGS_PER_PAGE) / TL_WEATHER_WIDTH;
        let mut next_segment_idx = segment_idx + 1;

        let hourly_segment_idx = offset_hour + scaled_hr / TL_WEATHER_WIDTH;
        let mut hourly_next_segment_idx = hourly_segment_idx + 1;

        let is_doubled = naive_next_idx == segment_idx;
        if next_segment_idx >= weather_data.segments.len() {
            next_segment_idx = segment_idx;
        }

        if hourly_next_segment_idx >= weather_data.hourly_segments.len() {
            hourly_next_segment_idx = hourly_segment_idx;
        }

        if segment_idx >= weather_data.segments.len()
            || hourly_segment_idx >= weather_data.hourly_segments.len()
        {
            // crate::debug!("Skipping seg {} with idx {}, hourly: {}", x, segment_idx, hourly_segment_idx);
            continue;
        }

        let slot = &weather_data.segments[segment_idx];
        let next_slot = &weather_data.segments[next_segment_idx];
        let slot_hr = &weather_data.hourly_segments[hourly_segment_idx];
        let next_slot_hr = &weather_data.hourly_segments[hourly_next_segment_idx];

        let weight = (scaled_15 % TL_WEATHER_WIDTH) as i32;
        let weight_hr = (scaled_hr % TL_WEATHER_WIDTH) as i32;

        let interp = interpolate_segment(slot, next_slot, weight);
        let interp_hr = interpolate_hourly(slot_hr, next_slot_hr, weight_hr);

        let max_temp = prev
            .temperature
            .max(slot.temperature)
            .max(next_slot.temperature);
        let max_cloud = prev_hr
            .cloud_cover
            .max(slot_hr.cloud_cover)
            .max(next_slot_hr.cloud_cover);

        draw_sun_line(&ctx, &bounds, x_i16, interp.sun_level, max_temp, max_cloud);
        draw_rain_line(
            &ctx,
            &bounds,
            x_i16,
            interp_hr.rain_chance,
            max_temp,
            max_cloud,
        );

        if x & 1 != 0 {
            draw_cloud_line(&ctx, &bounds, x_i16, interp_hr.cloud_cover);
        }

        if !is_doubled && line_is_on_midnight(now_dt, segment_idx) {
            let day_idx = ((now_ts - weather_data.report_start) / (60 * 60 * 24)) as usize;
            let next_day_idx = if day_idx < WEATHER_NUM_DAYS {
                day_idx + 1
            } else {
                day_idx
            };
            if day_idx < weather_data.daily.len() && next_day_idx < weather_data.daily.len() {
                draw_midnight_marker_and_dates(
                    &ctx,
                    &bounds,
                    x_i16,
                    interp.temperature,
                    &weather_data.daily[day_idx],
                    &weather_data.daily[next_day_idx],
                    !drawn_midnight,
                );
                drawn_midnight = true;
            }
        } else {
            draw_is_day(&ctx, &bounds, x_i16, interp.is_day);
        }

        if x & 1 != 0 {
            draw_temp_line(&ctx, &bounds, x_i16, interp.temperature);
        }

        if x > 0 {
            draw_temp_top_line(
                &ctx,
                &bounds,
                prev_x,
                x_i16,
                prev.temperature,
                interp.temperature,
            );
        }

        prev_x = x_i16;
        prev = interp;
        prev_hr = interp_hr;
    }

    draw_current_infos(&ctx, &bounds, weather);
}

impl WeatherLayer {
    pub fn new(frame: GRect) -> Self {
        let container = Layer::new(frame);
        container.set_update_proc(update_proc as LayerUpdateProc);
        Self { container }
    }

    pub fn layer(&self) -> &Layer<()> {
        &self.container
    }

    pub fn update(&self) {
        self.container.mark_dirty();
    }
}
