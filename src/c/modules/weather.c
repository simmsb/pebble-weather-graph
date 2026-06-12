#include "weather.h"
#include "message_keys.auto.h"
#include "utils.h"
#include <assert.h>
#include <math.h>
#include <stdint.h>

static Weather s_weather;

#define STORAGE_KEY_WEATHER 1

static void simulate_weather_data(Weather *);

void weather_init(void) {
  /* for (size_t i = 0; i < WEATHER_NUM_SEGMENTS; i++) { */
  /*   s_weather.segments[i] = (WeatherSegment){.temperature = 0, */
  /*                                            .rain_chance = 0, */
  /*                                            .cloud_cover = 0, */
  /*                                            .sun_level   = 0, */
  /*                                            .is_day      = 0, */
  /*                                            .uv_index    = 0}; */
  /* } */

  /* for (size_t i = 0; i < WEATHER_NUM_DAYS; i++) { */
  /*   s_weather.daily[i] = */
  /*       (WeatherDaily){.day = SUNDAY, .temp_high = 0, .temp_low = 0}; */
  /* } */

  /* s_weather.reportStart = 0; */
  /* strncpy(s_weather.locationName, "", WEATHER_CITY_MAX_LEN); */

  /* if (persist_exists(STORAGE_KEY_WEATHER)) { */
  /*   int stored_size = persist_get_size(STORAGE_KEY_WEATHER); */

  /*   if (stored_size == sizeof(s_weather)) { */
  /*     persist_read_data(STORAGE_KEY_WEATHER, &s_weather, sizeof(s_weather)); */
  /*   } */
  /* } */

  simulate_weather_data(&s_weather);
}

Weather *weather_get(void) { return &s_weather; }

void weather_save(void) { persist_write_data(STORAGE_KEY_WEATHER, &s_weather, sizeof(s_weather)); }

void weather_update_seg_from_message(DictionaryIterator *iter) {
  Tuple *t;

  t = dict_find(iter, MESSAGE_KEY_WEATHER_SEG_UPDATE_IDX);
  if (!t) {
    APP_LOG(APP_LOG_LEVEL_WARNING, "Told to update weather segment but had no idx");
    return;
  }

  assert(WEATHER_NUM_SEGMENTS < UINT8_MAX);
  uint8_t tmp_idx = tuple_read_uint8_t_or_panic(t);
  size_t idx      = (size_t)tmp_idx;

  dict_read_first(iter);
  WeatherSegment *seg = &weather_get()->segments[idx];

  while ((t = dict_read_next(iter))) {
    if (t->key == MESSAGE_KEY_WEATHER_SEG_TEMPERATURE) {
      seg->temperature = tuple_read_int16_t_or_panic(t);
    } else if (t->key == MESSAGE_KEY_WEATHER_SEG_RAIN_CHANCE) {
      seg->rain_chance = tuple_read_uint8_t_or_panic(t);
    } else if (t->key == MESSAGE_KEY_WEATHER_SEG_CLOUD_COVER) {
      seg->cloud_cover = tuple_read_uint8_t_or_panic(t);
    } else if (t->key == MESSAGE_KEY_WEATHER_SEG_SUN_LEVEL) {
      seg->sun_level = tuple_read_uint8_t_or_panic(t);
    } else if (t->key == MESSAGE_KEY_WEATHER_SEG_IS_DAY) {
      seg->is_day = tuple_read_uint8_t_or_panic(t);
    } else if (t->key == MESSAGE_KEY_WEATHER_SEG_UV_INDEX) {
      seg->rain_chance = tuple_read_uint8_t_or_panic(t);
    }
  }
}

void weather_update_day_from_message(DictionaryIterator *iter) {
  Tuple *t;

  t = dict_find(iter, MESSAGE_KEY_WEATHER_DAY_UPDATE_IDX);
  if (!t) {
    APP_LOG(APP_LOG_LEVEL_WARNING, "Told to update weather day but had no idx");
    return;
  }

  assert(WEATHER_NUM_DAYS < UINT8_MAX);
  uint8_t tmp_idx = tuple_read_uint8_t_or_panic(t);
  size_t idx      = (size_t)tmp_idx;

  dict_read_first(iter);
  WeatherDaily *day = &weather_get()->daily[idx];

  while ((t = dict_read_next(iter))) {
    if (t->key == MESSAGE_KEY_WEATHER_DAY_TEMP_HIGH) {
      day->temp_high = tuple_read_int16_t_or_panic(t);
    } else if (t->key == MESSAGE_KEY_WEATHER_DAY_TEMP_LOW) {
      day->temp_low = tuple_read_int16_t_or_panic(t);
    } else if (t->key == MESSAGE_KEY_WEATHER_DAY_NAME) {
      day->day = tuple_read_uint8_t_or_panic(t);
    }
  }
}

void weather_update_from_message(DictionaryIterator *iter) {
  Tuple *t;

  Weather *w = weather_get();

  while ((t = dict_read_next(iter))) {
    if (t->key == MESSAGE_KEY_WEATHER_REPORT_START) {
      // fix in 2106
      w->reportStart = (time_t)tuple_read_uint32_t_or_panic(t);
    } else if (t->key == MESSAGE_KEY_WEATHER_LOCATION_NAME) {
      tuple_read_cstring_or_panic(t, w->locationName, WEATHER_CITY_MAX_LEN);
    }
  }
}

static void simulate_weather_data(Weather *weather) {
  if (!weather)
    return;

  // 1. Initialize metadata
  weather->reportStart = time(NULL);
  strncpy(weather->locationName, "Pebble Valley", WEATHER_CITY_MAX_LEN);

  // Seed standard random generator using current time
  srand((unsigned int)weather->reportStart);

  // Get current hour component to sync the diurnal cycle to real-world time
  struct tm *time_info = localtime(&weather->reportStart);
  int start_hour       = time_info->tm_hour;

  // 2. Generate 15-Minute Segments (Core Data Matrix)
  for (int i = 0; i < WEATHER_NUM_SEGMENTS; i++) {
    float hours_elapsed = i / 4.0f;
    int current_hour    = (start_hour + (int)hours_elapsed) % 24;

    // --- A. Diurnal Temperature Model ---
    // Earth's temperature roughly mimics a cosine wave peaking around 15:00 (3 PM)
    float rad           = (current_hour - 15) * 0.2618f; // 2*PI / 24 hours = 0.2618 radians/hour
    float baseline_temp = 18.0f;                         // Warm day baseline (Celsius)

    // Introduce a weather front: Temperature drops by 5 degrees during a storm (Hours 12 to 24)
    if (hours_elapsed >= 12.0f && hours_elapsed <= 24.0f) {
      baseline_temp -= 5.0f;
    }

    // Calculate final temperature with a small amount of random variance (+/- 1 degree)
    int8_t simulated_temp = (int8_t)(baseline_temp + (7.0f * cosf(rad)) + (rand() % 3 - 1));
    weather->segments[i].temperature = simulated_temp;

    // --- B. Day / Night Tracking ---
    uint8_t is_day              = (current_hour >= 6 && current_hour <= 19) ? 1 : 0;
    weather->segments[i].is_day = is_day;

    // --- C. Moving Storm Front Model ---
    // We simulate a localized storm tracking smoothly across hours 10 through 26
    float storm_intensity = 0.0f;
    if (hours_elapsed >= 10.0f && hours_elapsed <= 26.0f) {
      // Linear ramp up and down peaking perfectly at hour 18
      storm_intensity = 1.0f - (fabsf(hours_elapsed - 18.0f) / 8.0f);
    }

    // --- D. Correlated Weather Attributes ---
    // Cloud Cover (Bitfield max 7: Clamp values strictly between 0 and 7)
    int clouds = (int)(storm_intensity * 7.0f) + (rand() % 2);
    if (!is_day && clouds < 2)
      clouds += 1; // Nights have baseline cloud noise
    if (clouds > 7)
      clouds = 7;
    if (clouds < 0)
      clouds = 0;
    weather->segments[i].cloud_cover = clouds;

    // Rain Chance (0% to 100%)
    int rain_chance = (int)(storm_intensity * 95.0f);
    if (rain_chance > 20)
      rain_chance += (rand() % 10 - 5); // Add variance
    if (rain_chance > 100)
      rain_chance = 100;
    if (rain_chance < 0)
      rain_chance = 0;
    weather->segments[i].rain_chance = rain_chance;

    // Solar Level & UV Index (Active only during day, heavily suppressed by clouds)
    if (is_day) {
      int sun                        = 7 - clouds - (rand() % 2);
      weather->segments[i].sun_level = (sun < 0) ? 0 : sun;

      // UV peaks at solar noon (12:00)
      float solar_noon_factor = 1.0f - (abs(current_hour - 12) / 6.0f);
      if (solar_noon_factor < 0)
        solar_noon_factor = 0;

      int uv                        = (int)(solar_noon_factor * (9.0f - clouds));
      weather->segments[i].uv_index = (uv < 0) ? 0 : uv;
    } else {
      weather->segments[i].sun_level = 0;
      weather->segments[i].uv_index  = 0;
    }
  }

  // 3. Generate Hourly Segments (Rain Accumulation Overlay)
  for (int h = 0; h < WEATHER_NUM_HOURS; h++) {
    // Sample the rain chance from the middle 15-minute segment of this hour
    uint8_t mid_hour_chance = weather->segments[h * 4 + 2].rain_chance;

    if (mid_hour_chance > 45) {
      // Rain volume in mm (scaled up to ~5mm depending on severity)
      int8_t volume = (int8_t)((mid_hour_chance - 40) / 12 + (rand() % 2));
      weather->hourly_segments[h].rain_amount = (volume <= 0) ? 1 : volume;
    } else {
      weather->hourly_segments[h].rain_amount = 0;
    }
  }

  // 4. Generate Daily Highs / Lows (Extracted directly from segment math)
  for (int d = 0; d < WEATHER_NUM_DAYS; d++) {
    int start_seg = d * 96; // 24 hours * 4 segments/hour
    int end_seg   = start_seg + 96;
    if (end_seg > WEATHER_NUM_SEGMENTS)
      end_seg = WEATHER_NUM_SEGMENTS;

    int8_t high = -128;
    int8_t low  = 127;

    for (int s = start_seg; s < end_seg; s++) {
      if (weather->segments[s].temperature > high)
        high = weather->segments[s].temperature;
      if (weather->segments[s].temperature < low)
        low = weather->segments[s].temperature;
    }

    weather->daily[d].temp_high = high;
    weather->daily[d].temp_low  = low;
    weather->daily[d].day       = (uint8_t)((time_info->tm_mday + d) % 31 + 1);
  }
}
