#include "weather.h"
#include "message_keys.auto.h"
#include "utils.h"
#include <assert.h>
#include <stdint.h>

static Weather s_weather;

#define STORAGE_KEY_WEATHER 1

void weather_init(void) {
  for (size_t i = 0; i < WEATHER_NUM_SEGMENTS; i++) {
    s_weather.segments[i] = (WeatherSegment){.temperature = 0,
                                             .rain_chance = 0,
                                             .cloud_cover = 0,
                                             .sun_level   = 0,
                                             .is_day      = 0,
                                             .uv_index    = 0};
  }

  for (size_t i = 0; i < WEATHER_NUM_DAYS; i++) {
    s_weather.daily[i] =
        (WeatherDaily){.day = SUNDAY, .temp_high = 0, .temp_low = 0};
  }

  s_weather.reportStart = 0;
  strncpy(s_weather.locationName, "", WEATHER_CITY_MAX_LEN);

  if (persist_exists(STORAGE_KEY_WEATHER)) {
    int stored_size = persist_get_size(STORAGE_KEY_WEATHER);

    if (stored_size == sizeof(s_weather)) {
      persist_read_data(STORAGE_KEY_WEATHER, &s_weather, sizeof(s_weather));
    }
  }
}

Weather *weather_get(void) { return &s_weather; }

void weather_save(void) {
  persist_write_data(STORAGE_KEY_WEATHER, &s_weather, sizeof(s_weather));
}

void weather_update_seg_from_message(DictionaryIterator *iter) {
  Tuple *t;

  t = dict_find(iter, MESSAGE_KEY_WEATHER_SEG_UPDATE_IDX);
  if (!t) {
    APP_LOG(APP_LOG_LEVEL_WARNING,
            "Told to update weather segment but had no idx");
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
