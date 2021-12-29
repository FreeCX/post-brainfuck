#include "emulator.h"

int brainfuck_command(const char s) {
    return (s == '>') || (s == '<') || (s == '+') || (s == '-') ||
           (s == '.') || (s == ',') || (s == '[') || (s == ']');
}

uint8_t app_init(const int tape_size, app_t **app) {
    *app = malloc(sizeof(app_t));
    if (*app == NULL) {
        return E_MEMORY;
    }
    (*app)->mem = (char *) calloc(tape_size, sizeof(char));
    if ((*app)->mem == NULL) {
        return E_MEMORY;
    }

    (*app)->ip = 0;
    (*app)->mp = 0;
    (*app)->app_size = 0;
    (*app)->mem_size = tape_size;

    return E_OK;
}

void app_parse_token(char token, app_t *app) {
    if (brainfuck_command(token)) {
        app->app[app->app_size] = token;
        app->app_size += 1;
    }
}

bool app_fill_jump_table(app_t *app) {
    app->jumps = (size_t *) calloc(app->app_size, sizeof(size_t));
    if (app->jumps == NULL) {
        return E_MEMORY;
    }

    array_t *array;
    uint8_t status = array_new(10, &array);
    if (status != E_OK) {
        return status;
    }

    for (size_t index = 0; index < app->app_size; index++) {
        if (app->app[index] == '[') {
            status = array_push(array, index);
            if (status != E_OK) {
                return status;
            }
        }
        if (app->app[index] == ']') {
            size_t last = 0;
            status = array_pop(array, &last);
            if (status != E_OK) {
                return status;
            }
            app->jumps[last] = index;
            app->jumps[index] = last;
        }
    }

    array_free(array);

    return E_OK;
}

uint8_t app_from_file(const char *filename, app_t *app) {
    size_t app_buffer = DEFAULT_APP_BUFFER;

    FILE *fp = fopen(filename, "r");
    if (fp == NULL) {
        return E_FILE;
    }

    app->app = (char *) calloc(app_buffer, sizeof(char));
    if (app->app == NULL) {
        return E_MEMORY;
    }

    int token = 0;
    while ((token = fgetc(fp)) != EOF) {
        if (app->app_size == app_buffer) {
            app_buffer *= 2;
            char * buffer = (char *) realloc(app->app, app_buffer);
            if (buffer == NULL) {
                return E_MEMORY;
            }
            app->app = buffer;
        }
        app_parse_token(token, app);
    }

    fclose(fp);

    return app_fill_jump_table(app);
}

void app_step(app_t *app) {
    char cmd = app->app[app->ip];
    switch (cmd) {
        case '>': {
            app->mp += 1;
            break;
        }
        case '<': {
            app->mp -= 1;
            break;
        }
        case '+': {
            app->mem[app->mp] += 1;
            break;
        }
        case '-': {
            app->mem[app->mp] -= 1;
            break;
        }
        case '.': {
            putchar(app->mem[app->mp]);
            break;
        }
        case ',': {
            int c = getchar();
            if (c == EOF) {
                return;
            }
            app->mem[app->mp] = (char)c;
            break;
        }
        case '[': {
            if (app->mem[app->mp] == 0) {
                app->ip = app->jumps[app->ip];
            }
            break;
        }
        case ']': {
            if (app->mem[app->mp] != 0) {
                app->ip = app->jumps[app->ip];
            }
            break;
        }
        default: { break; }
    }
    app->ip += 1;
}

void app_execute(app_t *app) {
    while (app->ip < app->app_size) {
        app_step(app);
    }
}

void app_destroy(app_t *app) {
    if (app) {
        if (app->mem) {
            free(app->mem);
        }
        if (app->app) {
            free(app->app);
        }
        if (app->jumps) {
            free(app->jumps);
        }
        free(app);
    }
}
