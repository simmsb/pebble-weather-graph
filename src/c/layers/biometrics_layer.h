#pragma once
#include <pebble.h>
#include <stdbool.h>

#define TL_BIOMETRICS_FONT_KEY FONT_KEY_GOTHIC_18
#define TL_BIOMETRICS_FONT_H 18
#define TL_BIOMETRICS_BORDER_PAD 0
#define TL_BIOMETRICS_BORDER_H_PAD 5
#define TL_BIOMETRICS_TEXT_Y_ADJ (-3)
#define TL_BIOMETRICS_H (TL_BIOMETRICS_FONT_H + 2 * TL_BIOMETRICS_BORDER_PAD + 1)
#define TL_BIOMETRICS_Y_PAD 10
#define TL_BIOMETRICS_LABEL_LEN 10

typedef struct {
  Layer *container;
  TextLayer *hr_layer;
  TextLayer *steps_layer;
  HealthValue heart_rate;
  HealthValue steps;
  GFont font;
  char hr_buf[TL_BIOMETRICS_LABEL_LEN];
  char steps_buf[TL_BIOMETRICS_LABEL_LEN];
} BiometricsLayer;

BiometricsLayer *biometrics_layer_create(GRect frame);
void biometrics_layer_destroy(BiometricsLayer *layer);
Layer *biometrics_layer_get_layer(BiometricsLayer *layer);
void biometrics_layer_update(BiometricsLayer *layer, HealthEventType typ);
