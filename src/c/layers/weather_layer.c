#include <pebble.h>
#include <stdint.h>
#include <stdio.h>

#include "c/modules/weather.h"
#include "weather_layer.h"

static WeatherLayer s_weatherlayer;

#define GRAPH_CHART_TOP 8

static GColor colour_for_temperature(int8_t temp) {
  if (temp >= 35) {
    return GColorRed;
  } else if (temp >= 30) {
    return GColorOrange;
  } else if (temp >= 25) {
    return GColorSunsetOrange;
  } else if (temp >= 22) {
    return GColorChromeYellow;
  } else if (temp >= 18) {
    return GColorPastelYellow;
  } else if (temp >= 10) {
    return GColorCeleste;
  } else if (temp >= 0) {
    return GColorPictonBlue;
  } else {
    return GColorElectricUltramarine;
  }
}

#define MIN_TEMP -10
#define MAX_TEMP 35

static int8_t temp_line_height_from_temp(GRect *bounds, int8_t temp) {
  return ((clamp(temp, MIN_TEMP, MAX_TEMP) - MIN_TEMP) *
          ((bounds->size.h - TL_WEATHER_TEMP_BOTTOM_PAD) - 10)) /
         (MAX_TEMP - MIN_TEMP);
}

static void draw_temp_line(GRect *bounds, GContext *ctx, int x, int8_t temp) {
  int height = temp_line_height_from_temp(bounds, temp);

  uint16_t y0 = bounds->origin.y + (bounds->size.h - TL_WEATHER_TEMP_BOTTOM_PAD);
  uint16_t y1 = y0 - height;

  GColor colour = colour_for_temperature(temp);

  graphics_context_set_antialiased(ctx, false);
  graphics_context_set_stroke_color(ctx, colour);
  graphics_context_set_stroke_width(ctx, 1);
  graphics_draw_line(ctx, GPoint(x, y0), GPoint(x, y1));
}

static void draw_temp_top_line(GRect *bounds, GContext *ctx, int x0, int x1, int8_t temp0,
                               int8_t temp1) {
  int height0 = temp_line_height_from_temp(bounds, temp0);
  int height1 = temp_line_height_from_temp(bounds, temp1);

  uint16_t bottom = bounds->origin.y + (bounds->size.h - TL_WEATHER_TEMP_BOTTOM_PAD);
  uint16_t y0     = bottom - height0;
  uint16_t y1     = bottom - height1;

  graphics_context_set_antialiased(ctx, true);
  graphics_context_set_stroke_color(ctx, GColorBlack);
  graphics_context_set_stroke_width(ctx, 1);
  graphics_draw_line(ctx, GPoint(x0, y0), GPoint(x1, y1));
}

static void cloud_cover_calculate_y_pos(GRect *bounds, uint8_t cloud_cover, uint16_t *y0_out,
                                        uint16_t *y1_out) {
  uint16_t max_cloud_radius = 4;
  uint16_t cloud_radius     = (cloud_cover * max_cloud_radius) / ((1 << 3) - 1);

  uint16_t y_mid = bounds->origin.y + max_cloud_radius;

  *y0_out = y_mid - cloud_radius;
  *y1_out = y_mid + cloud_radius + 1;
}

static void draw_cloud_line(GRect *bounds, GContext *ctx, int x, uint8_t cloud_cover) {
  if (cloud_cover < 1)
    return;

  // could cover is 3-bit integer
  uint16_t y0, y1;
  cloud_cover_calculate_y_pos(bounds, cloud_cover, &y0, &y1);

  graphics_context_set_antialiased(ctx, false);
  graphics_context_set_stroke_color(ctx, GColorDarkGray);
  graphics_context_set_stroke_width(ctx, 1);
  graphics_draw_line(ctx, GPoint(x, y0), GPoint(x, y1));
}

#define SUN_LEVEL_MIN_TO_SHOW 3

static void draw_sun_line(GRect *bounds, GContext *ctx, int x, uint8_t sun_level, int8_t temp,
                          uint8_t cloud_cover) {
  // sun level is a 3-bit integer
  if (sun_level < SUN_LEVEL_MIN_TO_SHOW)
    return;

  uint16_t cloud_y_ignored, cloud_y;
  cloud_cover_calculate_y_pos(bounds, cloud_cover, &cloud_y_ignored, &cloud_y);

  int temp_height = temp_line_height_from_temp(bounds, temp);
  uint16_t bottom = bounds->origin.y + (bounds->size.h - TL_WEATHER_TEMP_BOTTOM_PAD);
  uint16_t y0     = bottom - (temp_height + 4);
  uint16_t y1     = cloud_y + 2;

  GColor colour = GColorPastelYellow;

  graphics_context_set_antialiased(ctx, false);
  graphics_context_set_stroke_color(ctx, colour);
  graphics_context_set_stroke_width(ctx, 1);

  if (sun_level >= 6) {
    GColor dot_colour = GColorYellow;

    for (int y = y1; y <= y0; y++) {
      GColor this_colour = ((y & 1) == (x & 1)) ? dot_colour : colour;

      graphics_context_set_stroke_color(ctx, this_colour);
      graphics_draw_pixel(ctx, GPoint(x, y));
    }
  } else {
    graphics_draw_line(ctx, GPoint(x, y0), GPoint(x, y1));
  }
}

#define RAIN_CHANCE_MIN_TO_SHOW 30

/*

rain patterns

30    60    90

x     x     x x
  x    x    x x
x     x     x x
  x    x    x x
x     x     x x
  x    x    x x
x     x     x x
  x    x    x x

 */

typedef enum {
  RainPattern30,
  RainPattern60,
  RainPattern90,
} RainPattern;

static void draw_rain_line(GRect *bounds, GContext *ctx, int x, uint8_t rain_chance, int8_t temp,
                           uint8_t cloud_cover) {
  if (rain_chance < RAIN_CHANCE_MIN_TO_SHOW)
    return;

  uint16_t cloud_y_ignored, cloud_y;
  cloud_cover_calculate_y_pos(bounds, cloud_cover, &cloud_y_ignored, &cloud_y);

  int temp_height = temp_line_height_from_temp(bounds, temp);
  uint16_t bottom = bounds->origin.y + (bounds->size.h - TL_WEATHER_TEMP_BOTTOM_PAD);
  uint16_t y0     = bottom - (temp_height + 4);
  uint16_t y1     = cloud_y + 2;

  GColor colour = GColorBabyBlueEyes;

  RainPattern pattern = RainPattern30;
  if (rain_chance >= 90) {
    pattern = RainPattern90;
    colour  = GColorVeryLightBlue;
  } else if (rain_chance >= 60)
    pattern = RainPattern60;

  graphics_context_set_antialiased(ctx, false);
  graphics_context_set_stroke_color(ctx, colour);
  graphics_context_set_stroke_width(ctx, 1);

  for (int y = y1; y <= y0; y++) {
    switch (pattern) {
    case RainPattern30: {
      if (x & 1)
        continue;
      if ((y + (x / 2)) & 1)
        continue;
      break;
    }
    case RainPattern60: {
      if ((y & 1) == (x & 1))
        continue;
      break;
    }
    case RainPattern90: {
      if (!(x & 1))
        continue;
    }
    }

    graphics_draw_pixel(ctx, GPoint(x, y));
  }
}

static void draw_is_day(GRect *bounds, GContext *ctx, int x, bool is_day) {
  if (!is_day)
    return;

  uint16_t y = bounds->origin.y + (bounds->size.h - TL_WEATHER_TEMP_BOTTOM_PAD);

  graphics_context_set_antialiased(ctx, false);
  graphics_context_set_stroke_color(ctx, GColorLightGray);
  graphics_context_set_stroke_width(ctx, 1);
  graphics_draw_line(ctx, GPoint(x, y + 2), GPoint(x, y + 4));
}

static WeatherSegment interpolate_segment(WeatherSegment *a, WeatherSegment *b, int32_t weight) {
  int8_t temperature =
      (int16_t)a->temperature +
      (((int16_t)b->temperature - (int16_t)a->temperature) * weight) / TL_WEATHER_WIDTH;

  uint8_t sun_level = (int16_t)a->sun_level +
                      (((int16_t)b->sun_level - (int16_t)a->sun_level) * weight) / TL_WEATHER_WIDTH;
  uint8_t is_day    = a->is_day;

  return (WeatherSegment){.temperature = temperature, .sun_level = sun_level, .is_day = is_day};
}

static WeatherHourlySegment interpolate_hourly_segment(WeatherHourlySegment *a,
                                                       WeatherHourlySegment *b, int32_t weight) {
  uint8_t rain_amount =
      (int16_t)a->rain_amount +
      (((int16_t)b->rain_amount - (int16_t)a->rain_amount) * weight) / TL_WEATHER_WIDTH;
  uint8_t rain_chance =
      (int16_t)a->rain_chance +
      (((int16_t)b->rain_chance - (int16_t)a->rain_chance) * weight) / TL_WEATHER_WIDTH;
  uint8_t cloud_cover =
      (int16_t)a->cloud_cover +
      (((int16_t)b->cloud_cover - (int16_t)a->cloud_cover) * weight) / TL_WEATHER_WIDTH;

  return (WeatherHourlySegment){
      .rain_amount = rain_amount, .rain_chance = rain_chance, .cloud_cover = cloud_cover};
}

static char *day_name(int day) {
  switch (day) {
  case 0: return "SU";
  case 1: return "MO";
  case 2: return "TU";
  case 3: return "WE";
  case 4: return "TH";
  case 5: return "FR";
  case 6: return "SA";
  case 7: return "SU";
  default: return "UNK";
  }
}

static void draw_midnight_marker_and_dates(GRect *bounds, GContext *ctx, int x, int8_t temp,
                                           WeatherDaily *day_left, WeatherDaily *day_right) {
  int temp_height = temp_line_height_from_temp(bounds, temp);
  uint16_t bottom = bounds->origin.y + (bounds->size.h - TL_WEATHER_TEMP_BOTTOM_PAD);
  uint16_t y0     = bounds->origin.y;
  uint16_t y1     = bottom - (temp_height + 8);
  uint16_t y2     = bounds->origin.y + (bounds->size.h - TL_WEATHER_TEMP_BOTTOM_PAD) + 2;
  uint16_t y3     = bounds->origin.y + (bounds->size.h - TL_WEATHER_CURRENT_HEIGHT);

  graphics_context_set_antialiased(ctx, false);
  graphics_context_set_stroke_color(ctx, GColorLightGray);
  graphics_context_set_stroke_width(ctx, 1);

  for (int y = y0; y < y1; y += 2) {
    graphics_draw_pixel(ctx, GPoint(x, y));
  }

  for (int y = y2; y < y3; y += 2) {
    graphics_draw_pixel(ctx, GPoint(x, y));
  }

  // a bit sneaky, but also draw the days and min/max temps here, as this function is only called
  // once

  int day_width  = 15;
  int temp_width = 60;
  int text_pad   = 5;

  int left_day_x   = x - (day_width + text_pad - 1);
  int left_temp_x  = left_day_x - (temp_width + text_pad);
  int right_day_x  = x + text_pad;
  int right_temp_x = right_day_x + day_width + text_pad;
  int day_y        = y2 + 2;

  graphics_context_set_text_color(ctx, GColorBlack);
  GFont day_font = fonts_get_system_font(FONT_KEY_GOTHIC_14);
  graphics_draw_text(ctx, day_name(day_left->day), day_font,
                     GRect(left_day_x, day_y, day_width, 16), GTextOverflowModeFill,
                     GTextAlignmentRight, NULL);

  graphics_draw_text(ctx, day_name(day_right->day), day_font,
                     GRect(right_day_x, day_y, day_width, 16), GTextOverflowModeFill,
                     GTextAlignmentLeft, NULL);

  char temp_range_buf[16];
  snprintf(temp_range_buf, sizeof(temp_range_buf), "%d/%d°", day_left->temp_low,
           day_left->temp_high);
  graphics_draw_text(ctx, temp_range_buf, day_font, GRect(left_temp_x, day_y, temp_width, 16),
                     GTextOverflowModeFill, GTextAlignmentRight, NULL);

  snprintf(temp_range_buf, sizeof(temp_range_buf), "%d/%d°", day_right->temp_low,
           day_right->temp_high);
  graphics_draw_text(ctx, temp_range_buf, day_font, GRect(right_temp_x, day_y, temp_width, 16),
                     GTextOverflowModeFill, GTextAlignmentLeft, NULL);
}

static void draw_current_infos(GRect *bounds, GContext *ctx, int8_t current_temp,
                               char *location_name, time_t now) {
  uint16_t line_y = bounds->origin.y + (bounds->size.h - TL_WEATHER_CURRENT_HEIGHT);
  uint16_t text_y = line_y + 1;

  graphics_context_set_stroke_width(ctx, 1);
  graphics_context_set_stroke_color(ctx, GColorLightGray);
  graphics_context_set_stroke_width(ctx, 1);

  for (int x = 0; x < TL_WEATHER_WIDTH; x += 2) {
    graphics_draw_pixel(ctx, GPoint(x, line_y));
  }

  int x_mid = bounds->origin.x + bounds->size.w / 2;
  for (int y = line_y; y < (bounds->origin.y + bounds->size.h); y += 2) {
    graphics_draw_pixel(ctx, GPoint(x_mid, y));
  }

  GFont font = fonts_get_system_font(FONT_KEY_GOTHIC_14);

  char buf[WEATHER_CITY_MAX_LEN + 10];

  snprintf(buf, sizeof(buf), "%d° %s", current_temp, location_name);
  graphics_draw_text(ctx, buf, font, GRect(bounds->origin.x, text_y, bounds->size.w / 2, 16),
                     GTextOverflowModeFill, GTextAlignmentCenter, NULL);

  struct tm *t = gmtime(&now);
  strftime(buf, sizeof(buf), "%a %b %d", t);
  graphics_draw_text(ctx, buf, font, GRect(x_mid, text_y, bounds->size.w / 2, 16),
                     GTextOverflowModeFill, GTextAlignmentCenter, NULL);
}

static bool line_is_on_midnight(uint32_t start_15, size_t x) {
  time_t t_for_this_pos = (start_15 + x) * 15 * 60;
  struct tm *t          = gmtime(&t_for_this_pos);

  return t->tm_hour == 0 && t->tm_min == 0;
}

static void prv_update_weather_proc(Layer *layer, GContext *ctx) {
  WeatherLayer *wl = *(WeatherLayer **)layer_get_data(layer);
  Weather *weather = wl->weather_data;

  APP_LOG(APP_LOG_LEVEL_INFO, "Drawing weather");

  GRect bounds = layer_get_bounds(layer);
  time_t now   = time(NULL);

  size_t offset     = (now - wl->weather_data->reportStart) / (15 * 60);
  uint32_t start_15 = now / (15 * 60);

  WeatherSegment prev          = {0};
  WeatherHourlySegment prev_hr = {0};
  size_t prev_x                = 0;

  for (size_t x = 0; x < TL_WEATHER_WIDTH; x++) {
    size_t scaled_15 = x * TL_WEATHER_SEGS_PER_PAGE;
    size_t scaled_hr = x * TL_WEATHER_HOURLY_SEGS_PER_PAGE;

    size_t segment_idx      = offset + scaled_15 / TL_WEATHER_WIDTH;
    size_t naive_next_idx   = offset + (scaled_15 + TL_WEATHER_SEGS_PER_PAGE) / TL_WEATHER_WIDTH;
    size_t next_segment_idx = segment_idx + 1;

    size_t hourly_segment_idx      = offset + scaled_hr / TL_WEATHER_WIDTH;
    size_t hourly_next_segment_idx = hourly_segment_idx + 1;

    bool is_doubled = naive_next_idx == segment_idx;

    if (next_segment_idx >= TL_WEATHER_SEGS_PER_PAGE) {
      next_segment_idx = segment_idx;
    }

    WeatherSegment *slot      = &weather->segments[segment_idx];
    WeatherSegment *next_slot = &weather->segments[next_segment_idx];

    WeatherHourlySegment *slot_hr      = &weather->hourly_segments[hourly_segment_idx];
    WeatherHourlySegment *next_slot_hr = &weather->hourly_segments[hourly_next_segment_idx];

    int32_t weight    = scaled_15 % TL_WEATHER_WIDTH;
    int32_t weight_hr = scaled_hr % TL_WEATHER_WIDTH;

    WeatherSegment interp          = interpolate_segment(slot, next_slot, weight);
    WeatherHourlySegment interp_hr = interpolate_hourly_segment(slot_hr, next_slot_hr, weight_hr);

    draw_sun_line(&bounds, ctx, x, interp.sun_level,
                  MAX(prev.temperature, MAX(slot->temperature, next_slot->temperature)),
                  MAX(prev_hr.cloud_cover, MAX(slot_hr->cloud_cover, next_slot_hr->cloud_cover)));
    draw_rain_line(&bounds, ctx, x, interp_hr.rain_chance,
                   MAX(prev.temperature, MAX(slot->temperature, next_slot->temperature)),
                   MAX(prev_hr.cloud_cover, MAX(slot_hr->cloud_cover, next_slot_hr->cloud_cover)));

    if (x & 1) {
      draw_cloud_line(&bounds, ctx, x, interp_hr.cloud_cover);
    }

    if (!is_doubled && line_is_on_midnight(start_15, segment_idx)) {
      size_t day_idx      = (now - wl->weather_data->reportStart) / (60 * 60 * 24);
      size_t next_day_idx = (day_idx < WEATHER_NUM_DAYS) ? day_idx + 1 : day_idx;

      draw_midnight_marker_and_dates(&bounds, ctx, x, interp.temperature, &weather->daily[day_idx],
                                     &weather->daily[next_day_idx]);
    } else {
      draw_is_day(&bounds, ctx, x, interp.is_day);
    }

    // only draw temperature lines on odd x positions
    if (x & 1) {
      draw_temp_line(&bounds, ctx, x, interp.temperature);
    }

    if (x > 0) {
      draw_temp_top_line(&bounds, ctx, prev_x, x, prev.temperature, interp.temperature);
    }

    prev_x = x;
    prev   = interp;
  }

  draw_current_infos(&bounds, ctx, weather->currentTemperature, weather->locationName, now);

  APP_LOG(APP_LOG_LEVEL_INFO, "Done drawing weather");
}

WeatherLayer *weather_layer_create(GRect frame, Weather *weather_data) {
  WeatherLayer *wl = &s_weatherlayer;

  printf("Weather layer create");

  wl->weather_data = weather_data;
  wl->container    = layer_create_with_data(frame, sizeof(WeatherLayer *));
  *(WeatherLayer **)layer_get_data(wl->container) = wl;

  layer_set_update_proc(wl->container, prv_update_weather_proc);

  return wl;
}

void weather_layer_destroy(WeatherLayer *layer) {
  if (!layer)
    return;

  layer_destroy(layer->container);
}

Layer *weather_layer_get_layer(WeatherLayer *layer) { return layer->container; }

void weather_layer_update(WeatherLayer *layer) {
  printf("Weather layer update");
  layer_mark_dirty(layer->container);
}
