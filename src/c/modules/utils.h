#pragma once
#include <pebble.h>
#include <stdnoreturn.h>

#define xstr(a) str(a)
#define str(a) #a

#define CROAK(fmt, ...)                                                                            \
  {                                                                                                \
    APP_LOG(APP_LOG_LEVEL_ERROR, fmt, ##__VA_ARGS__);                                              \
    panic();                                                                                       \
  }
#define ASSERT(cond, fmt, ...)                                                                     \
  {                                                                                                \
    if (!(cond)) {                                                                                 \
      APP_LOG(APP_LOG_LEVEL_ERROR, "Assertion failed: " xstr(cond) ": " fmt, ##__VA_ARGS__);       \
      panic();                                                                                     \
    }                                                                                              \
  }

noreturn void panic();

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

void dict_reset(DictionaryIterator *iter);

/* Like dict_read_next but returns the cursor as it was before moving  */
Tuple *dict_take(DictionaryIterator *iter);

typedef struct {
  float x, y;
} vec2;

typedef struct {
  uint32_t v;
} Rng;

Rng rng_new(uint32_t seed);

uint32_t rng_next(Rng *rng);

int clamp(int a, int min, int max);

#define MAX(a, b) ((a > b) ? a : b)
