#pragma once
#include <pebble.h>

#define CROAK(fmt, args...) { APP_LOG(APP_LOG_LEVEL_ERROR, fmt, ## args); }

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
