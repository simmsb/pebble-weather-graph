#include "biometrics_layer.h"
#include "src/resource_ids.auto.h"
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static BiometricsLayer s_biometrics_layer;

static void prv_update_proc(Layer *layer, GContext *ctx) {
  BiometricsLayer *tl = *(BiometricsLayer **)layer_get_data(layer);
  GRect bounds        = layer_get_bounds(layer);

  GSize hr_size =
      graphics_text_layout_get_content_size(text_layer_get_text(tl->hr_layer), tl->font, bounds,
                                            GTextOverflowModeFill, GTextAlignmentLeft);
  GSize steps_size =
      graphics_text_layout_get_content_size(text_layer_get_text(tl->steps_layer), tl->font, bounds,
                                            GTextOverflowModeFill, GTextAlignmentLeft);

  int16_t icon_width = 0;
  int16_t num_icons  = 2;
  int16_t total_width =
      hr_size.w + steps_size.w + icon_width * num_icons + 2 * TL_BIOMETRICS_BORDER_H_PAD;

  GRect aligned = GRect(0, 0, total_width, bounds.size.h);
  grect_align(&aligned, &bounds, GAlignCenter, false);

  // printf("bio align: %d,%d %dx%d", aligned.origin.x, aligned.origin.y, aligned.size.w,
  //        aligned.size.h);
  // printf("bio frame: %d,%d %dx%d", bounds.origin.x, bounds.origin.y, bounds.size.w,
  // bounds.size.h); printf("steps_size: %dx%d", steps_size.w, steps_size.h);

  // GRect current_frame = layer_get_frame(text_layer_get_layer(tl->steps_layer));
  // printf("bio current: %d,%d %dx%d", current_frame.origin.x, current_frame.origin.y,
  // current_frame.size.w,
  //        current_frame.size.h);
  graphics_context_set_stroke_color(ctx, GColorBlack);
  graphics_context_set_antialiased(ctx, true);
  graphics_context_set_fill_color(ctx, GColorBlack);
  graphics_context_set_stroke_width(ctx, 0);
  graphics_fill_rect(ctx, GRect(aligned.origin.x, aligned.origin.y, aligned.size.w, aligned.size.h),
                     6, GCornersAll);
  graphics_context_set_antialiased(ctx, false);

  layer_set_frame(text_layer_get_layer(tl->hr_layer),
                  GRect(aligned.origin.x + icon_width + TL_BIOMETRICS_BORDER_H_PAD,
                        aligned.origin.y + TL_BIOMETRICS_TEXT_Y_ADJ, hr_size.w, hr_size.h));
  layer_set_frame(text_layer_get_layer(tl->steps_layer),
                  GRect(aligned.origin.x + hr_size.w + icon_width * 2 + TL_BIOMETRICS_BORDER_H_PAD,
                        aligned.origin.y + TL_BIOMETRICS_TEXT_Y_ADJ, steps_size.w, steps_size.h));
}

BiometricsLayer *biometrics_layer_create(GRect frame) {
  BiometricsLayer *tl = &s_biometrics_layer;

  tl->container = layer_create_with_data(frame, sizeof(BiometricsLayer *));

  int w                 = frame.size.w;
  GFont biometrics_font = fonts_get_system_font(TL_BIOMETRICS_FONT_KEY);
  // GFont biometrics_font = fonts_load_custom_font(resource_get_handle(RESOURCE_ID_LM_MONO_20));

  tl->font = biometrics_font;

  tl->hr_layer = text_layer_create(GRect(0, TL_BIOMETRICS_BORDER_PAD, w, TL_BIOMETRICS_FONT_H));
  text_layer_set_background_color(tl->hr_layer, GColorClear);
  text_layer_set_text_color(tl->hr_layer, GColorWhite);
  text_layer_set_font(tl->hr_layer, biometrics_font);
  layer_add_child(tl->container, text_layer_get_layer(tl->hr_layer));

  tl->steps_layer = text_layer_create(GRect(0, TL_BIOMETRICS_BORDER_PAD, w, TL_BIOMETRICS_FONT_H));
  text_layer_set_background_color(tl->steps_layer, GColorClear);
  text_layer_set_text_color(tl->steps_layer, GColorWhite);
  text_layer_set_font(tl->steps_layer, biometrics_font);
  layer_add_child(tl->container, text_layer_get_layer(tl->steps_layer));

  *(BiometricsLayer **)layer_get_data(tl->container) = tl;

  layer_set_update_proc(tl->container, prv_update_proc);

  return tl;
}

void biometrics_layer_destroy(BiometricsLayer *layer) {
  if (!layer)
    return;
  text_layer_destroy(layer->hr_layer);
  text_layer_destroy(layer->steps_layer);
  layer_destroy(layer->container);
}

Layer *biometrics_layer_get_layer(BiometricsLayer *layer) {
  return layer ? layer->container : NULL;
}

void biometrics_layer_update(BiometricsLayer *layer, HealthEventType typ) {

  snprintf(layer->hr_buf, sizeof(layer->hr_buf), "P %ld/", (long)87);
  text_layer_set_text(layer->hr_layer, layer->hr_buf);
  snprintf(layer->steps_buf, sizeof(layer->steps_buf), "S %ld", (long)1234);
  text_layer_set_text(layer->steps_layer, layer->steps_buf);

  /* switch (typ) { */
  /* case HealthEventHeartRateUpdate: { */
  /*   layer->heart_rate = health_service_peek_current_value(HealthMetricHeartRateBPM); */
  /*   snprintf(layer->hr_buf, sizeof(layer->hr_buf), "%ld", (long)layer->heart_rate); */
  /*   text_layer_set_text(layer->hr_layer, layer->hr_buf); */
  /*   break; */
  /* } */
  /* case HealthEventMovementUpdate: { */
  /*   layer->steps = health_service_peek_current_value(HealthMetricStepCount); */
  /*   snprintf(layer->steps_buf, sizeof(layer->steps_buf), "%ld", (long)layer->steps); */
  /*   text_layer_set_text(layer->steps_layer, layer->steps_buf); */
  /*   break; */
  /* } */
  /* default: return; */
  /* } */

  layer_mark_dirty(layer->container);
}
