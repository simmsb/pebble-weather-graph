#include "layers/biometrics_layer.h"
#include "layers/time_layer.h"
#include "layers/top_bar_layer.h"
#include "modules/settings.h"
#include "modules/weather.h"
#include <pebble.h>
#include <stddef.h>

// GRAPH_LAYERS_H is the combined height of daylight+cloud+precip+event — also
// used for the icon bar overlay and the temp layer so all three match. Must be
// tall enough to fit the icon slots: >= 228 uses 22px icons (need 66px+),
// middle tier uses 18px icons (56px gives zone_h=18), small uses 14px icons.
#if PBL_DISPLAY_HEIGHT >= 228
#define DAYLIGHT_H 11
#define CLOUD_H 17
#define PRECIP_H 26
#define EVENT_H 12
// Sums to 66
#elif PBL_DISPLAY_HEIGHT <= 168
#define DAYLIGHT_H 9
#define CLOUD_H 13
#define PRECIP_H 11
#define EVENT_H 12
// Sums to 45
#else
#define DAYLIGHT_H 11
#define CLOUD_H 16
#define PRECIP_H 17
#define EVENT_H 12
// Sums to 56
#endif
#define GRAPH_LAYERS_H (DAYLIGHT_H + CLOUD_H + PRECIP_H + EVENT_H)

static Window *s_main_window;
static TimeLayer *s_time_layer;
static TopBarLayer *s_top_bar_layer;
static BiometricsLayer *s_biometrics_layer;

static void prv_request_weather(void);

static void prv_tick_handler(struct tm *tick_time, TimeUnits units_changed) {
  time_layer_update(s_time_layer, tick_time, settings_get());

  // Request fresh weather every hour and re-push cached data so that
  // current_hour advances in all graph layers regardless of whether a new
  // fetch succeeds.
  if (units_changed & HOUR_UNIT) {
    prv_request_weather();
  }
}

/**
 * Handle incoming AppMessage; parse weather fields and push to layers if
 * complete.
 *
 * @param iter     Incoming message dictionary iterator.
 * @param context  Unused callback context.
 */
static void prv_inbox_received(DictionaryIterator *iter, void *context) {
  DictionaryIterator orig = *iter;
  settings_apply_from_message(iter);
  *iter = orig;
  weather_update_from_message(iter);
  *iter = orig;
  weather_update_seg_from_message(iter);
  *iter = orig;
  weather_update_day_from_message(iter);
}

static void prv_inbox_dropped(AppMessageResult reason, void *context) {
  APP_LOG(APP_LOG_LEVEL_WARNING, "Inbox dropped: %d", (int)reason);
}

static void prv_request_weather(void) {
  DictionaryIterator *iter;
  AppMessageResult result = app_message_outbox_begin(&iter);
  if (result == APP_MSG_OK) {
    dict_write_uint8(iter, MESSAGE_KEY_WEATHER_REQUEST, 1);
    app_message_outbox_send();
  }
}

static void prv_battery_handler(BatteryChargeState state) { top_bar_layer_set_battery(s_top_bar_layer, state); }

static void prv_bt_handler(bool connected) { top_bar_layer_set_bt(s_top_bar_layer, connected); }

static void prv_health_handler(HealthEventType typ, void *context) {
  (void)context;
  biometrics_layer_update(s_biometrics_layer, typ);
}

static void prv_window_load(Window *window) {
  Layer *root  = window_get_root_layer(window);
  GRect bounds = layer_get_bounds(root);
  int w        = bounds.size.w;

  APP_LOG(APP_LOG_LEVEL_INFO, "display top left: %d,%d", bounds.origin.x, bounds.origin.y);

  int y = 0;
  s_time_layer = time_layer_create(GRect(0, TL_TIME_Y_TOP, w, TL_TIME_H));
  layer_add_child(root, time_layer_get_layer(s_time_layer));
  y += TL_TIME_Y_TOP + TL_TIME_H;

  // top bar overlays time
  s_top_bar_layer = top_bar_layer_create(GRect(0, TL_TOP_BAR_Y_TOP, w, TL_TOP_BAR_H));
  layer_add_child(root, top_bar_layer_get_layer(s_top_bar_layer));

  y += TL_BIOMETRICS_Y_PAD;
  s_biometrics_layer = biometrics_layer_create(GRect(0, y, w, TL_BIOMETRICS_H));
  layer_add_child(root, biometrics_layer_get_layer(s_biometrics_layer));
  y += TL_BIOMETRICS_H;

  // Seed time display immediately
  time_t now_t   = time(NULL);
  struct tm *now = localtime(&now_t);
  if (now)
    time_layer_update(s_time_layer, now, settings_get());
}

static void prv_window_unload(Window *window) {
  time_layer_destroy(s_time_layer);
  top_bar_layer_destroy(s_top_bar_layer);
}

//
// App lifecycle
//

static void init(void) {
  settings_init();
  weather_init();

  s_main_window = window_create();
  window_set_background_color(s_main_window, GColorWhite);
  window_set_window_handlers(s_main_window, (WindowHandlers){
                                                .load   = prv_window_load,
                                                .unload = prv_window_unload,
                                            });
  window_stack_push(s_main_window, true);

  tick_timer_service_subscribe(MINUTE_UNIT, prv_tick_handler);
  battery_state_service_subscribe(prv_battery_handler);
  connection_service_subscribe((ConnectionHandlers){
      .pebble_app_connection_handler = prv_bt_handler,
  });
  health_service_events_subscribe(prv_health_handler, NULL);

  // AppMessage: register callbacks BEFORE opening
  app_message_register_inbox_received(prv_inbox_received);
  app_message_register_inbox_dropped(prv_inbox_dropped);
  app_message_open(1536, 64);

  // Trigger initial weather fetch
  prv_request_weather();
}

static void deinit(void) {
  tick_timer_service_unsubscribe();
  battery_state_service_unsubscribe();
  connection_service_unsubscribe();
  window_destroy(s_main_window);
}

int main(void) {
  init();
  app_event_loop();
  deinit();
  return 0;
}
