#include "top_bar_layer.h"
#include <stddef.h>
#include <stdlib.h>
#include <string.h>

static TopBarLayer s_top_bar_layer;

static void prv_update_proc(Layer *layer, GContext *ctx) {
  TopBarLayer *tl = *(TopBarLayer **)layer_get_data(layer);
  GRect bounds    = layer_get_bounds(layer);

  graphics_context_set_stroke_color(ctx, GColorBlack);
  graphics_context_set_stroke_width(ctx, 1);

  if (tl->bluetooth_on) {
    int x            = 5;
    int y            = 1;
    int w            = 6;
    int h            = 10;
    int cross_offset = 2;

    // Draw bluetooth icon, starting from top left. Each command is the next line segment.
    graphics_context_set_antialiased(ctx, false);
    graphics_draw_line(ctx, GPoint(x, y + cross_offset), GPoint(x + w, y + h - cross_offset));
    // End the line at y + h + 1 as opposed to y + h as the line drawing
    // algorithm stays one y-pixel too low at that slope.
    graphics_draw_line(ctx, GPoint(x + w, y + h - cross_offset), GPoint(x + w / 2, y + h + 1));
    graphics_draw_line(ctx, GPoint(x + w / 2, y + h), GPoint(x + w / 2, y));
    graphics_draw_line(ctx, GPoint(x + w / 2, y - 1), GPoint(x + w, y + cross_offset));
    graphics_draw_line(ctx, GPoint(x + w, y + cross_offset), GPoint(x, y + h - cross_offset));
  }

  // draw battery icon
  int w         = 12;
  int h         = 7;
  int x         = bounds.size.w - (w + 7);
  int y         = 0;
  int bar_h     = 3;
  int bar_inset = (h - bar_h) / 2;

  int num_bars    = (w - 2) / 2;
  int pct_per_bar = 100 / (num_bars + 2);

  graphics_context_set_antialiased(ctx, false);

  if (tl->battery_level.is_charging) {
    graphics_context_set_stroke_color(ctx, GColorGreen);
  }

  // border
  graphics_draw_line(ctx, GPoint(x, y), GPoint(x, y + h));
  graphics_draw_line(ctx, GPoint(x, y + h), GPoint(x + w, y + h));
  graphics_draw_line(ctx, GPoint(x + w, y + h), GPoint(x + w, y));
  graphics_draw_line(ctx, GPoint(x + w, y), GPoint(x, y));
  graphics_draw_line(ctx, GPoint(x + w + 1, y + bar_inset), GPoint(x + w + 1, y + h - bar_inset));

  uint8_t pc = tl->battery_level.charge_percent;

  for (int i = 0; i < num_bars; i++) {
    if (pc >= (pct_per_bar * (i + 1))) {
      graphics_draw_line(ctx, GPoint(x + 2 + i * 2, y + bar_inset), GPoint(x + 2 + i * 2, y + h - bar_inset));
    }
  }
}

TopBarLayer *top_bar_layer_create(GRect frame) {
  TopBarLayer *tl = &s_top_bar_layer;

  tl->container = layer_create_with_data(frame, sizeof(TopBarLayer *));

  *(TopBarLayer **)layer_get_data(tl->container) = tl;

  layer_set_update_proc(tl->container, prv_update_proc);

  return tl;
}

void top_bar_layer_destroy(TopBarLayer *layer) {
  if (!layer)
    return;
  layer_destroy(layer->container);
}

Layer *top_bar_layer_get_layer(TopBarLayer *layer) { return layer ? layer->container : NULL; }

void top_bar_layer_set_bt(TopBarLayer *layer, bool on) {
  layer->bluetooth_on = on;
  layer_mark_dirty(layer->container);
}

void top_bar_layer_set_battery(TopBarLayer *layer, BatteryChargeState level) {
  layer->battery_level = level;
  layer_mark_dirty(layer->container);
}
