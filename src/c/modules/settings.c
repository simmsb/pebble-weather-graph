#include "settings.h"

#define STORAGE_KEY_SETTINGS 0

static Settings s_settings;

// Default settings values. New fields must be appended to the end of the
// struct to maintain compatibility with old persisted data.
static const Settings s_defaults = {
    .accent_color = {.argb = 0b11111111}, // GColorWhite
};

void settings_init(void) {
  s_settings = s_defaults;
  if (persist_exists(STORAGE_KEY_SETTINGS)) {
    int stored_size = persist_get_size(STORAGE_KEY_SETTINGS);
    // Read min(stored, current) bytes so field migration works in both
    // directions: upgrading (stored < current) keeps new field defaults;
    // downgrading (stored > current) discards unknown trailing fields but
    // preserves all fields the current version does know about.
    if (stored_size > 0) {
      int read_size = stored_size < (int)sizeof(s_settings) ? stored_size : (int)sizeof(s_settings);
      persist_read_data(STORAGE_KEY_SETTINGS, &s_settings, read_size);
    }
  }
}

Settings *settings_get(void) { return &s_settings; }

void settings_save(void) {
  persist_write_data(STORAGE_KEY_SETTINGS, &s_settings, sizeof(s_settings));
}

void settings_apply_from_message(DictionaryIterator *iter) {
  bool changed = false;
  Tuple *t;

  t = dict_find(iter, MESSAGE_KEY_SETTING_ACCENT_COLOR);
  if (t) {
    changed                 = true;
    s_settings.accent_color = GColorFromHEX(t->value->int32);
  }

  if (changed)
    settings_save();
}
