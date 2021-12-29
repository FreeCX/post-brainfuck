#pragma once
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

#include "array.h"
#include "const.h"
#include "error.h"

typedef struct app {
    char *mem;
    char *app;
    size_t *jumps;
    size_t mem_size;
    size_t app_size;
    size_t ip;
    size_t mp;
} app_t;

uint8_t app_init(const int tape_size, app_t **app);
void app_parse_token(char token, app_t *app);
uint8_t app_from_buffer(const char *buffer);
uint8_t app_from_file(const char *filename, app_t *app);
void app_step(app_t *app);
void app_execute(app_t *app);
void app_destroy(app_t *app);
