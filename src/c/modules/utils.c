#include "utils.h"
#include <math.h>

Success tuple_read_data(Tuple *t, uint8_t *dest, const uint16_t max_size) {
  if (t->type != TUPLE_BYTE_ARRAY) {
    return FAIL;
  }
  if (t->length > max_size) {
    return FAIL;
  };

  memcpy(dest, t->value->data, t->length);

  return OK;
}

Success tuple_read_cstring(Tuple *t, char *dest, const uint16_t max_size) {
  if (t->type != TUPLE_CSTRING) {
    return FAIL;
  }
  if (t->length > max_size) {
    return FAIL;
  };

  strncpy(dest, t->value->cstring, t->length);

  return OK;
}

Success tuple_read_uint8_t(Tuple *t, uint8_t *dest) {
  if (t->type != TUPLE_UINT) {
    return FAIL;
  }
  if (t->length != 1) {
    return FAIL;
  };

  *dest = t->value->uint8;

  return OK;
}

Success tuple_read_uint16_t(Tuple *t, uint16_t *dest) {
  if (t->type != TUPLE_UINT) {
    return FAIL;
  }
  if (t->length != 2) {
    return FAIL;
  };

  *dest = t->value->uint16;

  return OK;
}

Success tuple_read_uint32_t(Tuple *t, uint32_t *dest) {
  if (t->type != TUPLE_UINT) {
    return FAIL;
  }
  if (t->length != 4) {
    return FAIL;
  };

  *dest = t->value->uint32;

  return OK;
}

Success tuple_read_int8_t(Tuple *t, int8_t *dest) {
  if (t->type != TUPLE_INT) {
    return FAIL;
  }
  if (t->length != 1) {
    return FAIL;
  };

  *dest = t->value->int8;

  return OK;
}

Success tuple_read_int16_t(Tuple *t, int16_t *dest) {
  if (t->type != TUPLE_INT) {
    return FAIL;
  }
  if (t->length != 2) {
    return FAIL;
  };

  *dest = t->value->int16;

  return OK;
}

Success tuple_read_int32_t(Tuple *t, int32_t *dest) {
  if (t->type != TUPLE_INT) {
    return FAIL;
  }
  if (t->length != 4) {
    return FAIL;
  };

  *dest = t->value->int32;

  return OK;
}

void tuple_read_data_or_panic(Tuple *t, uint8_t *dest, const uint16_t max_size) {
  if (!tuple_read_data(t, dest, max_size)) {
    CROAK("tried to read tuple of incorrect type");
  }
}

void tuple_read_cstring_or_panic(Tuple *t, char *dest, const uint16_t max_size) {
  if (!tuple_read_cstring(t, dest, max_size)) {
    CROAK("tried to read tuple of incorrect type");
  }
}

uint8_t tuple_read_uint8_t_or_panic(Tuple *t) {
  uint8_t dst = 0;
  if (!tuple_read_uint8_t(t, &dst)) {
    CROAK("tried to read tuple of incorrect type");
  }
  return dst;
}
uint16_t tuple_read_uint16_t_or_panic(Tuple *t) {
  uint16_t dst = 0;
  if (!tuple_read_uint16_t(t, &dst)) {
    CROAK("tried to read tuple of incorrect type");
  }
  return dst;
}
uint32_t tuple_read_uint32_t_or_panic(Tuple *t) {
  uint32_t dst = 0;
  if (!tuple_read_uint32_t(t, &dst)) {
    CROAK("tried to read tuple of incorrect type");
  }
  return dst;
}
int8_t tuple_read_int8_t_or_panic(Tuple *t) {
  int8_t dst = 0;
  if (!tuple_read_int8_t(t, &dst)) {
    CROAK("tried to read tuple of incorrect type");
  }
  return dst;
}
int16_t tuple_read_int16_t_or_panic(Tuple *t) {
  int16_t dst = 0;
  if (!tuple_read_int16_t(t, &dst)) {
    CROAK("tried to read tuple of incorrect type");
  }
  return dst;
}
int32_t tuple_read_int32_t_or_panic(Tuple *t) {
  int32_t dst = 0;
  if (!tuple_read_int32_t(t, &dst)) {
    CROAK("tried to read tuple of incorrect type");
  }
  return dst;
}

Rng rng_new(uint32_t seed) { return (Rng){.v = seed * 0x9E3779B9}; }

uint32_t rng_next(Rng *rng) {
  uint32_t x = rng->v;
  x ^= x << 13;
  x ^= x >> 17;
  x ^= x << 5;
  return rng->v = x;
}

static float vec2_length2(vec2 v) { return v.x * v.x + v.y * v.y; }

static float vec2_distance(vec2 a, vec2 b) {
  float dx = a.x - b.x;
  float dy = a.y - b.y;
  return sqrtf(dx * dx + dy * dy);
}

static float energy(size_t w, size_t h, const vec2 buffer[w][h], size_t x, size_t y) {
  // Loop over 9x9 region around target pixel.
  float totalEnergy = 0.0f;
  for (int dy = -4; dy <= 4; dy++) {
    for (int dx = -4; dx <= 4; dx++) {
      if (dx == 0 && dy == 0)
        continue;

      size_t x2 = (x + dx + w) % w;
      size_t y2 = (y + dy + h) % h;

      float dist = vec2_distance(buffer[y][x], buffer[y2][x2]);
      float len  = vec2_length2((vec2){.x = dx, .y = dy});

      totalEnergy += expf(-len / (2.1f * 2.1f) - dist);
    }
  }

  return totalEnergy;
}

static float int_to_float(uint32_t random) {
  union {
    uint32_t u32;
    float f;
  } u = {.u32 = random >> 9 | 0x3f800000};

  return u.f - 1.0;
}

static void minimize_energy(Rng *rng, size_t w, size_t h, vec2 buffer[w][h]) {
  // Try to swap pixels for many iterations.
  for (int i = 0; i < 1000; i++) {
    size_t x0 = rng_next(rng) % w;
    size_t y0 = rng_next(rng) % h;
    size_t x1 = rng_next(rng) % w;
    size_t y1 = rng_next(rng) % h;

    if (x0 == x1 && y0 == y1)
      continue;

    // Find energy before and after swapping pixels.
    float startEnergy = energy(w, h, buffer, x0, y0) + energy(w, h, buffer, x1, y1);
    vec2 tmp          = buffer[y0][x0];
    buffer[y0][x0]    = buffer[y1][x1];
    buffer[y1][x1]    = tmp;

    float endEnergy = energy(w, h, buffer, x0, y0) + energy(w, h, buffer, x1, y1);

    // If the energy was lower before the swap, then swap back.
    if (startEnergy < endEnergy) {
      vec2 tmp       = buffer[y0][x0];
      buffer[y0][x0] = buffer[y1][x1];
      buffer[y1][x1] = tmp;
    }
  }
}

void generate_blue_noise(Rng *rng, size_t w, size_t h, vec2 buffer[w][h]) {
  for (size_t y = 0; y < h; y++) {
    for (size_t x = 0; x < w; x++) {
      buffer[y][x] = (vec2){.x = int_to_float(rng_next(rng)), .y = int_to_float(rng_next(rng))};
    }
  }

  minimize_energy(rng, w, h, buffer);
}
