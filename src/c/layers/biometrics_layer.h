#pragma once
#include <pebble.h>
#include <stdbool.h>

#define TL_BIOMETRICS_FONT_KEY FONT_KEY_ROBOTO_CONDENSED_21
#define TL_BIOMETRICS_H (21 + 6)
#define TL_BIOMETRICS_Y_PAD 5
#define TL_BIOMETRICS_LABEL_LEN 10

typedef struct {
  Layer *container;
  Layer *inner;
  TextLayer *hr_layer;
  TextLayer *steps_layer;
  HealthValue heart_rate;
  HealthValue steps;
  char hr_buf[TL_BIOMETRICS_LABEL_LEN];
  char steps_buf[TL_BIOMETRICS_LABEL_LEN];
} BiometricsLayer;

BiometricsLayer *biometrics_layer_create(GRect frame);
void biometrics_layer_destroy(BiometricsLayer *layer);
Layer *biometrics_layer_get_layer(BiometricsLayer *layer);
void biometrics_layer_update(BiometricsLayer *layer, HealthEventType typ);
