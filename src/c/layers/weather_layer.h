#pragma once

#include "c/modules/utils.h"
#include "c/modules/weather.h"
#include <pebble.h>

#define TL_WEATHER_Y_PAD 10
#define TL_WEATHER_HOURS_PER_PAGE 36
#define TL_WEATHER_SEGS_PER_PAGE (TL_WEATHER_HOURS_PER_PAGE * TL_WEATHER_SEGS_PER_HOUR)
#define TL_WEATHER_HOURLY_SEGS_PER_PAGE TL_WEATHER_HOURS_PER_PAGE
#define TL_WEATHER_CURRENT_HEIGHT 20
#define TL_WEATHER_TEMP_BOTTOM_PAD (22 + TL_WEATHER_CURRENT_HEIGHT)
#define TL_WEATHER_WIDTH 200

typedef struct {
  Layer *container;
  Weather *weather_data;
} WeatherLayer;

WeatherLayer *weather_layer_create(GRect frame, Weather *weather_data);
void weather_layer_destroy(WeatherLayer *layer);
Layer *weather_layer_get_layer(WeatherLayer *layer);
void weather_layer_update(WeatherLayer *layer);

/*
 * Weather needs to render:
 *
 * - Vertical day dividers
 * - Cloud cover percentage
 * - Rain stripes vertically
 * - Temperature curve w/ dithered fill
 * - rain amount lines from bottom - these blot out curve
 * - day/night
 */
