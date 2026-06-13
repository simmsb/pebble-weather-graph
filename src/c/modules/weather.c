#include "weather.h"
#include "message_keys.auto.h"
#include "utils.h"
#include <stdint.h>

static Weather s_weather;

#define STORAGE_KEY_WEATHER 1

void weather_init(void) { s_weather = (Weather){0}; }

Weather *weather_get(void) { return &s_weather; }

void weather_save(void) { persist_write_data(STORAGE_KEY_WEATHER, &s_weather, sizeof(s_weather)); }

void weather_update_seg_from_message(DictionaryIterator *iter) {
  Tuple *t;

  t = dict_find(iter, MESSAGE_KEY_WEATHER_SEG_UPDATE_IDX);
  if (!t) {
    return;
  }

  int32_t tmp_idx = tuple_read_int32_t_or_panic(t);
  size_t idx      = (size_t)tmp_idx;

  if (idx >= WEATHER_NUM_SEGMENTS) return;
  // ASSERT(idx < WEATHER_NUM_SEGMENTS, "idx: %d", idx);

  dict_reset(iter);
  WeatherSegment *seg = &weather_get()->segments[idx];

  while ((t = dict_take(iter))) {
    if (t->key == MESSAGE_KEY_WEATHER_SEG_TEMPERATURE) {
      seg->temperature = tuple_read_int32_t_or_panic(t);
    } else if (t->key == MESSAGE_KEY_WEATHER_SEG_SUN_LEVEL) {
      seg->sun_level = tuple_read_int32_t_or_panic(t);
    } else if (t->key == MESSAGE_KEY_WEATHER_SEG_IS_DAY) {
      seg->is_day = tuple_read_int32_t_or_panic(t);
    }
  }
}

void weather_update_hour_from_message(DictionaryIterator *iter) {
  Tuple *t;

  t = dict_find(iter, MESSAGE_KEY_WEATHER_HOUR_UPDATE_IDX);
  if (!t) {
    return;
  }

  int32_t tmp_idx = tuple_read_int32_t_or_panic(t);
  size_t idx      = (size_t)tmp_idx;

  if (idx >= WEATHER_NUM_HOURS) return;
  // ASSERT(idx < WEATHER_NUM_HOURS, "idx: %d", idx);

  dict_reset(iter);
  WeatherHourlySegment *seg = &weather_get()->hourly_segments[idx];

  while ((t = dict_take(iter))) {
    if (t->key == MESSAGE_KEY_WEATHER_HOUR_RAIN_AMOUNT) {
      seg->rain_amount = tuple_read_int32_t_or_panic(t);
    } else if (t->key == MESSAGE_KEY_WEATHER_HOUR_RAIN_CHANCE) {
      seg->rain_chance = tuple_read_int32_t_or_panic(t);
    } else if (t->key == MESSAGE_KEY_WEATHER_HOUR_CLOUD_COVER) {
      seg->cloud_cover = tuple_read_int32_t_or_panic(t);
    }
  }
}

void weather_update_day_from_message(DictionaryIterator *iter) {
  Tuple *t;

  t = dict_find(iter, MESSAGE_KEY_WEATHER_DAY_UPDATE_IDX);
  if (!t) {
    return;
  }

  int32_t tmp_idx = tuple_read_int32_t_or_panic(t);
  size_t idx      = (size_t)tmp_idx;

  if (idx >= WEATHER_NUM_DAYS) return;
  // ASSERT(idx < WEATHER_NUM_DAYS, "idx: %d", idx);

  dict_reset(iter);
  WeatherDaily *day = &weather_get()->daily[idx];

  while ((t = dict_take(iter))) {
    if (t->key == MESSAGE_KEY_WEATHER_DAY_TEMP_HIGH) {
      day->temp_high = tuple_read_int32_t_or_panic(t);
    } else if (t->key == MESSAGE_KEY_WEATHER_DAY_TEMP_LOW) {
      day->temp_low = tuple_read_int32_t_or_panic(t);
    } else if (t->key == MESSAGE_KEY_WEATHER_DAY_NAME) {
      day->day = tuple_read_int32_t_or_panic(t);
    }
  }
}

bool weather_update_from_message(DictionaryIterator *iter) {
  Tuple *t;

  Weather *w = weather_get();

  dict_reset(iter);

  bool updated = false;

  while ((t = dict_take(iter))) {
    if (t->key == MESSAGE_KEY_WEATHER_REPORT_START) {
      // fix in 2049
      w->reportStart = (time_t)tuple_read_int32_t_or_panic(t);
      updated = true;
    } else if (t->key == MESSAGE_KEY_WEATHER_LOCATION_NAME) {
      tuple_read_cstring_or_panic(t, w->locationName, WEATHER_CITY_MAX_LEN);
      updated = true;
    } else if (t->key == MESSAGE_KEY_WEATHER_CURRENT_TEMP) {
      w->currentTemperature = tuple_read_int32_t_or_panic(t);
      updated = true;
    }
  }

  return updated;
}
