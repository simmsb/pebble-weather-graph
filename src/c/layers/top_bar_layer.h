#pragma once
#include <pebble.h>
#include <stdbool.h>

#define TL_TOP_BAR_H 14
#define TL_TOP_BAR_Y_TOP 5

typedef struct {
  Layer *container;
  bool bluetooth_on;
  BatteryChargeState battery_level;
} TopBarLayer;

TopBarLayer *top_bar_layer_create(GRect frame);
void top_bar_layer_destroy(TopBarLayer *layer);
Layer *top_bar_layer_get_layer(TopBarLayer *layer);
void top_bar_layer_set_bt(TopBarLayer *layer, bool on);
void top_bar_layer_set_battery(TopBarLayer *layer, BatteryChargeState level);
