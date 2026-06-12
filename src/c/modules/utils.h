#pragma once
#include <pebble.h>

#define CROAK(fmt, ...) { APP_LOG(APP_LOG_LEVEL_ERROR, fmt, ## __VA_ARGS__); }

typedef enum { FAIL, OK } Success;

Success tuple_read_data(Tuple *t, uint8_t *dest, const uint16_t max_size);
Success tuple_read_cstring(Tuple *t, char *dest, const uint16_t max_size);
Success tuple_read_uint8_t(Tuple *t, uint8_t *dest);
Success tuple_read_uint16_t(Tuple *t, uint16_t *dest);
Success tuple_read_uint32_t(Tuple *t, uint32_t *dest);
Success tuple_read_int8_t(Tuple *t, int8_t *dest);
Success tuple_read_int16_t(Tuple *t, int16_t *dest);
Success tuple_read_int32_t(Tuple *t, int32_t *dest);

void tuple_read_data_or_panic(Tuple *t, uint8_t *dest, const uint16_t max_size);
void tuple_read_cstring_or_panic(Tuple *t, char *dest, const uint16_t max_size);
uint8_t tuple_read_uint8_t_or_panic(Tuple *t);
uint16_t tuple_read_uint16_t_or_panic(Tuple *t);
uint32_t tuple_read_uint32_t_or_panic(Tuple *t);
int8_t tuple_read_int8_t_or_panic(Tuple *t);
int16_t tuple_read_int16_t_or_panic(Tuple *t);
int32_t tuple_read_int32_t_or_panic(Tuple *t);

typedef struct {
  float x, y;
} vec2;

typedef struct {
  uint32_t v;
} Rng;

Rng rng_new(uint32_t seed);

uint32_t rng_next(Rng *rng);

void generate_blue_noise(Rng *rng, size_t w, size_t h, vec2 buffer[w][h]);
