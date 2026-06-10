#pragma once
#include "../modules/settings.h"
#include <pebble.h>

#define TL_TIME_FONT_KEY RESOURCE_ID_LM_MONO_48
#define TL_TIME_H 49
#define TL_TIME_Y_TOP 0

typedef struct {
  Layer *container;
  TextLayer *time_label;
  GFont font;
  char time_buf[8];
} TimeLayer;

TimeLayer *time_layer_create(GRect frame);
void time_layer_destroy(TimeLayer *layer);
Layer *time_layer_get_layer(TimeLayer *layer);
void time_layer_update(TimeLayer *layer, struct tm *tick_time,
                       const Settings *settings);
