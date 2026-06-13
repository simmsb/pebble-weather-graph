#include <pebble.h>
#include <stdint.h>

#include "c/modules/blue_noise.h"
#include "c/modules/weather.h"
#include "weather_layer.h"

static WeatherLayer s_weatherlayer;

#define GRAPH_CHART_TOP 8

static void prv_update_proc_weather(Layer *layer, GContext *ctx) {
  WeatherLayer *wl = *(WeatherLayer **)layer_get_data(layer);
  Weather *weather = wl->weather_data;

  APP_LOG(APP_LOG_LEVEL_INFO, "Drawing weather");

  GRect bounds = layer_get_bounds(layer);
  time_t now   = time(NULL);

  // --- Step 1: Find Min/Max Temp using Scalar Variables Only ---
  int8_t min_temp = 127;
  int8_t max_temp = -128;
  bool data_found = false;

  for (int x = 0; x < TL_WEATHER_WIDTH; x++) {
    time_t t   = now + (x * 36 * 3600) / TL_WEATHER_WIDTH;
    int idx_15 = (t - weather->reportStart) / 900;
    if (idx_15 >= 0 && idx_15 < WEATHER_NUM_SEGMENTS) {
      int8_t temp = weather->segments[idx_15].temperature;
      if (temp < min_temp)
        min_temp = temp;
      if (temp > max_temp)
        max_temp = temp;
      data_found = true;
    }
  }
  if (!data_found || min_temp == max_temp) {
    min_temp = 0;
    max_temp = 30;
  }

  // --- Step 2: Allocate Minimal Error-Diffusion Buffers (800 bytes total) ---
  int16_t err_row0[TL_WEATHER_WIDTH] = {0};
  int16_t err_row1[TL_WEATHER_WIDTH] = {0};
  int16_t *curr_err                  = err_row0;
  int16_t *next_err                  = err_row1;

  int graph_usable_h = TL_WEATHER_HEIGHT - GRAPH_CHART_TOP;

  // --- Step 3: Row-by-Row Rendering with Direct Indexing ---
  for (int y = 0; y < TL_WEATHER_HEIGHT; y++) {

    // Compute the baseline gradient shade for the current row
    int target_gray = 85;
    if (y >= GRAPH_CHART_TOP) {
      target_gray = 85 + ((y - GRAPH_CHART_TOP) * 85) / (graph_usable_h - 1);
    }

    for (int x = 0; x < TL_WEATHER_WIDTH; x++) {
      // Direct Time-to-Index calculations on the fly
      time_t t   = now + (x * 36 * 3600) / TL_WEATHER_WIDTH;
      int idx_15 = (t - weather->reportStart) / 900;
      int idx_1  = (t - weather->reportStart) / 3600;

      bool is_valid =
          (idx_15 >= 0 && idx_15 < WEATHER_NUM_SEGMENTS && idx_1 >= 0 && idx_1 < WEATHER_NUM_HOURS);

      GColor dither_color = GColorDarkGray;

      // A. Error Diffusion Field Generator (Unconditional calculation per pixel)
      if (y >= GRAPH_CHART_TOP) {
        int continuous_val = target_gray + curr_err[x];
        int chosen_val;

        if (continuous_val < 128) {
          dither_color = GColorDarkGray;
          chosen_val   = 85;
        } else {
          dither_color = GColorLightGray;
          chosen_val   = 170;
        }

        int err = continuous_val - chosen_val;

        // Spread error factors into adjacent columns/rows
        if (x + 1 < TL_WEATHER_WIDTH) {
          curr_err[x + 1] += (err * 7) / 16;
        }
        if (y + 1 < TL_WEATHER_HEIGHT) {
          if (x > 0)
            next_err[x - 1] += (err * 3) / 16;
          next_err[x] += (err * 5) / 16;
          if (x + 1 < TL_WEATHER_WIDTH)
            next_err[x + 1] += (err * 1) / 16;
        }
      }

      // B. Resolve Final Pixel Layout Color
      GColor pixel_color = GColorWhite;

      if (!is_valid) {
        pixel_color = GColorWhite;
      } else if (y < GRAPH_CHART_TOP) {
        // Cloud Cover (Top Layout Area)
        pixel_color =
            (y < (int)weather->segments[idx_15].cloud_cover) ? GColorLightGray : GColorWhite;
      } else {
        // Direct extraction from active structs
        WeatherSegment seg         = weather->segments[idx_15];
        WeatherHourlySegment h_seg = weather->hourly_segments[idx_1];

        // On-the-fly layout math expressions
        int y_temp = (TL_WEATHER_HEIGHT - 1) - 4 -
                     ((seg.temperature - min_temp) * (graph_usable_h - 8)) / (max_temp - min_temp);

        int seconds_into_hour = (t - weather->reportStart) % 3600;
        bool is_rain_bar_col  = (seconds_into_hour < 2400) && (h_seg.rain_amount > 0);

        int bar_height = h_seg.rain_amount * 4;
        int y_rain_top = (TL_WEATHER_HEIGHT - 1) - bar_height;
        if (y_rain_top < GRAPH_CHART_TOP)
          y_rain_top = GRAPH_CHART_TOP;

        // Composite layers
        if (is_rain_bar_col && y >= y_rain_top) {
          pixel_color = GColorCobaltBlue; // Layer 1: Rain Bar
        } else if (y == y_temp) {
          pixel_color = GColorBlack; // Layer 2: Temp Line
        } else if (y > y_temp) {
          pixel_color = dither_color; // Layer 3: Fill Below (Dither Field)
        } else {
          // Layer 4: Clear Skies / Procedural Weather Pattern Fills Above
          if (h_seg.rain_amount > 0 || seg.rain_chance > 20) {
            int mod_val = (seg.rain_chance > 75) ? 2 : ((seg.rain_chance > 45) ? 4 : 8);
            pixel_color = ((x + y * 2) % mod_val == 0) ? GColorBlue : GColorWhite;
          } else if (seg.is_day && seg.sun_level > 0) {
            int mod_val = (seg.sun_level >= 6) ? 2 : ((seg.sun_level >= 3) ? 4 : 6);
            pixel_color = ((x + y) % mod_val == 0) ? GColorYellow : GColorWhite;
          } else {
            pixel_color = GColorWhite;
          }
        }
      }

      graphics_context_set_stroke_color(ctx, pixel_color);
      graphics_draw_pixel(ctx, GPoint(bounds.origin.x + x, bounds.origin.y + y));
    }

    // C. Dynamic Buffer Row Swapping
    if (y >= GRAPH_CHART_TOP) {
      int16_t *temp_ptr = curr_err;
      curr_err          = next_err;
      next_err          = temp_ptr;
      memset(next_err, 0, sizeof(err_row0));
    }
  }
}

// typedef struct {
//   int16_t error;
//   Rng rng;
// } Dither;

// static Dither dither_new(uint8_t seed) { return (Dither){.error = 0, .rng = rng_new(seed)}; }

// static uint8_t add_u8_signed_saturating(uint8_t a, int16_t b, int16_t *err) {
//   int16_t sum = (int16_t)a + b;

//   if (sum > 255) {
//     *err = (int16_t)(sum - 255);
//     return 255;
//   } else if (sum < 0) {
//     *err = (int16_t)sum;
//     return 0;
//   } else {
//     *err = 0;
//     return (uint16_t)sum;
//   }
// }

// shade, error, chosen
//     0,     0,      0
//     1,     1,      0  // because we chose 0 but had shade of 1
//     2,     3,      0
//     3,     6,      0
//     4,     10,     0
//     5,     15,     0
//     6,     21,     0
//     7,     28,     0
//     8,     36,     0
//     9,     45,     0
//     10,    55,     0
//     11,    66,     0
//     12,    14,     1

// static GColor next_color_target_shade(Dither *dither, uint8_t shade) {
//   // int8_t random_offset = 8 - (int8_t)(rng_next(&dither->rng) & 0xf);
//   // uint8_t shade_       = add_u8_signed_saturating(shade, &random_offset);
//   int16_t target = (int16_t)shade + dither->error;

//   printf("shade %d, target %d, error: %d", shade, target, dither->error);

//   // dither->error = err;

//   // // if we switched colour, adjust
//   // if (dither->error > 64) {
//   //   dither->error -= 64;
//   // } else if (dither->error < -64) {
//   //   dither->error += 64;
//   // }

//   // dither->error += err;

//   if (target >= 192) {
//     dither->error += target - 192;
//     return GColorWhite;
//   } else if (target >= 128) {
//     dither->error += target - 128;
//     return GColorLightGray;
//   } else if (target >= 64) {
//     dither->error += target - 64;
//     return GColorDarkGray;
//   } else {
//     dither->error += target - 0;
//     return GColorBlack;
//   }
// }

uint16_t rand_n_bits(int x, int y, uint8_t bits) {
  int o = 0;

  uint16_t random_value = 0;

  for (int i = 0; i < bits; i++) {
    random_value = (random_value << 1) | noise_get_at(y, x + o++);
  }

  return random_value;
}

static GColor color_dithered(int x, int y, uint8_t shade) {
  int16_t offset = 64 - (int16_t)rand_n_bits(x, y, 7);
  int16_t target = shade + offset;

  if (target >= 192) {
    return GColorWhite;
  } else if (target >= 128) {
    return GColorLightGray;
  } else if (target >= 64) {
    return GColorDarkGray;
  } else {
    return GColorBlack;
  }
}

static void draw_temp_to_height(GRect *bounds, GContext *ctx, int x, int height) {
  uint16_t y     = bounds->origin.y + bounds->size.h;
  uint16_t y_max = y - height;

  // Dither dither = dither_new(x);

  while (y > y_max) {
    uint8_t shade = ((uint16_t)y * 255) / TL_WEATHER_HEIGHT;
    GColor color  = color_dithered(x, y, shade);

    // GColor color = next_color_target_shade(&dither, shade);
    // printf("height %d shade %d, colour: %d", y, shade, color.r);

    graphics_context_set_stroke_color(ctx, color);
    graphics_draw_pixel(ctx, GPoint(x, y));

    y--;
  }
}

static int clamp(int a, int min, int max) {
  if (a < min) {
    return min;
  } else if (a > max) {
    return max;
  } else {
    return a;
  }
}

static void prv_update_proc_weather_2(Layer *layer, GContext *ctx) {

  WeatherLayer *wl = *(WeatherLayer **)layer_get_data(layer);
  Weather *weather = wl->weather_data;

  APP_LOG(APP_LOG_LEVEL_INFO, "Drawing weather");

  GRect bounds = layer_get_bounds(layer);
  time_t now   = time(NULL);

  time_t offset = (now - wl->weather_data->reportStart) / (15 * 60);

  int min_temp = -10;
  int max_temp = 35;

  // first, draw temperature shadow

  for (size_t x = 0; x < TL_WEATHER_WIDTH; x++) {
    size_t segment_idx = offset + (x * TL_WEATHER_SEGS_PER_PAGE) / TL_WEATHER_WIDTH;
    size_t next_segment_idx = segment_idx + 1;

    WeatherSegment *slot = &weather->segments[segment_idx];

    int y = ((clamp(slot->temperature, min_temp, max_temp) - min_temp) * TL_WEATHER_TEMP_HEIGHT) /
            (max_temp - min_temp);

    // printf("Draw height at %d to %d (idx %d)", x, y, segment_idx);
    draw_temp_to_height(&bounds, ctx, x, y);
  }
}

WeatherLayer *weather_layer_create(GRect frame, Weather *weather_data) {
  WeatherLayer *wl = &s_weatherlayer;

  wl->weather_data = weather_data;
  wl->container    = layer_create_with_data(frame, sizeof(WeatherLayer *));
  *(WeatherLayer **)layer_get_data(wl->container) = wl;

  layer_set_update_proc(wl->container, prv_update_proc_weather_2);

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
