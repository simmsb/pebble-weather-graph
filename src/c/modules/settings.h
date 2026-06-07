#pragma once
#include <pebble.h>

typedef struct {
	GColor accent_color;
} Settings;

void settings_init(void);
Settings *settings_get(void);
void settings_save(void);
void settings_apply_from_message(DictionaryIterator *iter);
