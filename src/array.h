#pragma once
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#include "error.h"

typedef struct array {
    size_t index;
    size_t length;
    size_t *data;
} array_t;

uint8_t array_new(size_t size, array_t **array);
uint8_t array_push(array_t *array, size_t item);
uint8_t array_pop(array_t *array, size_t *value);
void array_free(array_t *array);
