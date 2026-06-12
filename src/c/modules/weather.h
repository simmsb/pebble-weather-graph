#pragma once
#include <pebble.h>
#include <stdint.h>

#define WEATHER_NUM_HOURS 48
#define WEATHER_NUM_DAYS 2
#define TL_WEATHER_SEGS_PER_HOUR 4
#define WEATHER_NUM_SEGMENTS (WEATHER_NUM_HOURS * TL_WEATHER_SEGS_PER_HOUR)
#define WEATHER_CITY_MAX_LEN 24

typedef struct {
  int8_t temperature;
  uint8_t rain_chance;
  uint8_t cloud_cover : 3;
  uint8_t sun_level : 3;
  uint8_t is_day : 1;
  uint8_t uv_index;
} WeatherSegment;

typedef struct {
  int8_t rain_amount;
} WeatherHourlySegment;

typedef struct {
  int8_t temp_high;
  int8_t temp_low;
  uint8_t day;
} WeatherDaily;

typedef struct {
  WeatherSegment segments[WEATHER_NUM_SEGMENTS];
  WeatherHourlySegment hourly_segments[WEATHER_NUM_HOURS];
  WeatherDaily daily[WEATHER_NUM_DAYS];

  time_t reportStart; // when this was fetched
  char locationName[WEATHER_CITY_MAX_LEN];
} Weather;

void weather_init(void);
Weather *weather_get(void);
void weather_save(void);
void weather_update_seg_from_message(DictionaryIterator *iter);
void weather_update_day_from_message(DictionaryIterator *iter);
void weather_update_from_message(DictionaryIterator *iter);
