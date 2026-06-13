#include "utils.h"

noreturn void panic() {
  APP_LOG(APP_LOG_LEVEL_ERROR, "App panicked, oops!");

  __builtin_trap();
}

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

void dict_reset(DictionaryIterator *iter) { dict_read_first(iter); }

Tuple *dict_take(DictionaryIterator *iter) {
  Tuple *ret = iter->cursor;

  if ((void *)ret >= iter->end)
    return NULL;

  Tuple *next = dict_read_next(iter);

  if (ret == next)
    return NULL;

  return ret;
}

Rng rng_new(uint32_t seed) { return (Rng){.v = seed * 0x9E3779B9}; }

uint32_t rng_next(Rng *rng) {
  uint32_t x = rng->v;
  x ^= x << 13;
  x ^= x >> 17;
  x ^= x << 5;
  return rng->v = x;
}

int clamp(int a, int min, int max) {
  if (a < min) {
    return min;
  } else if (a > max) {
    return max;
  } else {
    return a;
  }
}
