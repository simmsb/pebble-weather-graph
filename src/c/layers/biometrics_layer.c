#include "biometrics_layer.h"
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static BiometricsLayer s_biometrics_layer;

static void prv_update_proc(Layer *layer, GContext *ctx) {
  BiometricsLayer *tl = *(BiometricsLayer **)layer_get_data(layer);
  GRect bounds        = layer_get_bounds(layer);

  graphics_context_set_stroke_color(ctx, GColorBlack);
  graphics_context_set_antialiased(ctx, true);
  graphics_context_set_fill_color(ctx, GColorBlack);
  graphics_context_set_stroke_width(ctx, 1);

  GSize hr_size    = text_layer_get_content_size(tl->hr_layer);
  GSize steps_size = text_layer_get_content_size(tl->steps_layer);

  int16_t icon_width  = 20;
  int16_t num_icons   = 3;
  int16_t total_width = hr_size.w + steps_size.w + icon_width * 3;

  GRect aligned = GRect(0, 0, total_width, bounds.size.h);
  grect_align(&aligned, &bounds, GAlignCenter, false);

  layer_set_frame(tl->inner, aligned);
  layer_set_frame(text_layer_get_layer(tl->hr_layer), GRect(aligned.origin.x, aligned.origin.y, hr_size.w, hr_size.h));
  layer_set_frame(text_layer_get_layer(tl->steps_layer),
                  GRect(aligned.origin.x + hr_size.w + icon_width, aligned.origin.y, steps_size.w, steps_size.h));
}

BiometricsLayer *biometrics_layer_create(GRect frame) {
  BiometricsLayer *tl = &s_biometrics_layer;

  tl->container = layer_create_with_data(frame, sizeof(BiometricsLayer *));
  tl->inner     = layer_create(frame);

  layer_add_child(tl->container, tl->inner);

  int w                 = frame.size.w;
  GFont biometrics_font = fonts_get_system_font(TL_BIOMETRICS_FONT_KEY);
  tl->hr_layer          = text_layer_create(GRect(0, 0, w, TL_BIOMETRICS_H));
  text_layer_set_background_color(tl->hr_layer, GColorClear);
  text_layer_set_text_color(tl->hr_layer, GColorWhite);
  text_layer_set_font(tl->hr_layer, biometrics_font);
  layer_add_child(tl->inner, text_layer_get_layer(tl->hr_layer));

  tl->steps_layer = text_layer_create(GRect(0, 0, w, TL_BIOMETRICS_H));
  text_layer_set_background_color(tl->hr_layer, GColorClear);
  text_layer_set_text_color(tl->hr_layer, GColorWhite);
  text_layer_set_font(tl->hr_layer, biometrics_font);
  layer_add_child(tl->inner, text_layer_get_layer(tl->steps_layer));

  *(BiometricsLayer **)layer_get_data(tl->container) = tl;

  layer_set_update_proc(tl->container, prv_update_proc);

  return tl;
}

void biometrics_layer_destroy(BiometricsLayer *layer) {
  if (!layer)
    return;
  text_layer_destroy(layer->hr_layer);
  text_layer_destroy(layer->steps_layer);
  layer_destroy(layer->inner);
  layer_destroy(layer->container);
  free(layer);
}

Layer *biometrics_layer_get_layer(BiometricsLayer *layer) { return layer ? layer->container : NULL; }

void biometrics_layer_update(BiometricsLayer *layer, HealthEventType typ) {
  switch (typ) {
  case HealthEventHeartRateUpdate: {
    layer->heart_rate = health_service_peek_current_value(HealthMetricHeartRateBPM);
    snprintf(layer->hr_buf, sizeof(layer->hr_buf), "%d", layer->heart_rate);
    text_layer_set_text(layer->hr_layer, layer->hr_buf);
    break;
  }
  case HealthEventMovementUpdate: {
    layer->steps = health_service_peek_current_value(HealthMetricStepCount);
    snprintf(layer->steps_buf, sizeof(layer->steps_buf), "%d", layer->steps);
    text_layer_set_text(layer->steps_layer, layer->steps_buf);
    break;
  }
  default: return;
  }

  layer_mark_dirty(layer->container);
}
