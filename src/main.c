#include "emulator.h"

int main(int argc, char *argv[]) {
    if (argc != 2) {
        printf("usage: %s <filename>\n", argv[0]);
        exit(EXIT_SUCCESS);
    }

    app_t *app;
    uint8_t status = app_init(DEFAULT_MEM_SIZE, &app);
    if (status != E_OK) {
        printf("app init failed: %d\n", status);
        return EXIT_FAILURE;
    }

    char *filename = argv[1];
    status = app_from_file(filename, app);
    if (status == E_OK) {
        app_execute(app);
    } else {
        printf("app load failed: %d\n", status);
    }

    app_destroy(app);

    return EXIT_SUCCESS;
}
