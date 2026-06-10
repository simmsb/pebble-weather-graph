#pragma once
#include <pebble.h>

#define TL_WEATHER_HEIGHT 140

typedef struct {

} WeatherLayer;


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
