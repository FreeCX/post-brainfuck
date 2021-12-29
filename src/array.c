#include "array.h"

uint8_t array_new(size_t size, array_t **array) {
    *array = (array_t *) malloc(sizeof(array_t));
    if (*array == NULL) {
        return E_MEMORY;
    }
    (*array)->data = (size_t *) malloc(size * sizeof(size_t));
    if ((*array)->data == NULL) {
        return E_MEMORY;
    }
    (*array)->length = size;
    (*array)->index = 0;
    return E_OK;
}

uint8_t array_push(array_t *array, size_t item) {
    if (array->index == array->length) {
        array->length *= 2;
        size_t *new_ptr = (size_t *) realloc(array->data, sizeof(size_t) * array->length);
        if (new_ptr == NULL) {
            return E_MEMORY;
        }
        array->data = new_ptr;
    }
    array->data[array->index] = item;
    array->index += 1;
    return E_OK;
}

uint8_t array_pop(array_t *array, size_t *value) {
    if (array->index == 0) {
        return E_EMPTY;
    }
    array->index -= 1;
    *value = array->data[array->index];
    return E_OK;
}

void array_free(array_t *array) {
    if (array) {
        if (array->data) {
            free(array->data);
        }
        free(array);
    }
}
