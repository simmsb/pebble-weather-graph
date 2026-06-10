#include "time_layer.h"
#include <stddef.h>
#include <stdlib.h>
#include <string.h>

static TimeLayer s_timelayer;

TimeLayer *time_layer_create(GRect frame) {
  TimeLayer *tl = &s_timelayer;

  tl->time_buf[0] = '\0';

  tl->container = layer_create(frame);
  int w         = frame.size.w;

  GFont time_font = fonts_load_custom_font(resource_get_handle(TL_TIME_FONT_KEY));
  tl->font = time_font;
  tl->time_label  = text_layer_create(GRect(0, 0, w, TL_TIME_H));
  text_layer_set_background_color(tl->time_label, GColorClear);
  text_layer_set_text_color(tl->time_label, GColorBlack);
  text_layer_set_font(tl->time_label, time_font);
  text_layer_set_text_alignment(tl->time_label, GTextAlignmentCenter);
  text_layer_set_text(tl->time_label, tl->time_buf);
  layer_add_child(tl->container, text_layer_get_layer(tl->time_label));

  return tl;
}

void time_layer_destroy(TimeLayer *layer) {
  if (!layer)
    return;
  text_layer_destroy(layer->time_label);
  fonts_unload_custom_font(layer->font);
  layer_destroy(layer->container);
}

Layer *time_layer_get_layer(TimeLayer *layer) {
  return layer ? layer->container : NULL;
}

void time_layer_update(TimeLayer *layer, struct tm *tick_time,
                       const Settings *settings) {
  if (!layer || !tick_time || !settings)
    return;

  strftime(layer->time_buf, sizeof(layer->time_buf), "%H:%M", tick_time);
  text_layer_set_text(layer->time_label, layer->time_buf);
}
