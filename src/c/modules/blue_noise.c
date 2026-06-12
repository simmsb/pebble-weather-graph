#include <pebble.h>
#include <stddef.h>
#include <stdint.h>

#include "blue_noise_data.h"

bool noise_get_at(int x, int y) {
  size_t idx = y * (BLUE_NOISE_W / 8) + x;
  uint8_t b  = blue_noise[idx % (sizeof(blue_noise) / sizeof(blue_noise[0]))];
  return (bool)((b >> (y % 8)) & 1);
}
