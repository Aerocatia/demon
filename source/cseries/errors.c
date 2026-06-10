#include <stdint.h>
#include <stdio.h>
#include <time.h>
#include <string.h>

#include "cseries.h"
#include "build_number.h"
#include "platform.h"

#include "errors.h"

#define DEBUG_OUTPUT_FILENAME "debug.txt"

#ifdef REQUIRE_CACHE_FILE
#define CACHE_STRING "(CACHE)"
#else
#define CACHE_STRING ""
#endif

enum {
    ERROR_MESSAGE_BUFFER_MAXIMUM_SIZE = 4 * KILO
};

struct error_global_data {
    bool delayed;
    char debug_file_path[260]; // not in Gearbox Custom Edition release
    bool output_to_debug_file;
    bool display_state;
    bool recursion_lock;
    bool overflow_suppression;
    bool suppress_all;
    uint8_t developer_mode; // developer_mode enum
    uint8_t unused;
    int16_t message_buffer_size;
    char message_buffer[ERROR_MESSAGE_BUFFER_MAXIMUM_SIZE];
};
static_assert(sizeof(struct error_global_data) == 4366);

/* globals */

asm(".set _error_globals, 0x00B016C8"); // HACK demon.dll
extern struct error_global_data error_globals; // remove extern

bool find_all_fucked_up_shit = false;
int32_t fucked_up_shit_count = 0;

/* forward declarations */

static void reset_error_state(void);
char *(*errors_debug_file_path)(char *filename) = (void *)0x0075BB30;

/* public functions */

void errors_initialize(void) {
    error_globals.developer_mode = _developer_mode_full;
    error_globals.output_to_debug_file = true;
    error_globals.overflow_suppression = true;
    error_globals.suppress_all = false;
    error_globals.debug_file_path[0] = '\0';

    strcpy(error_globals.debug_file_path, errors_debug_file_path(DEBUG_OUTPUT_FILENAME));

    reset_error_state();
    stack_walk_initialize();
}

void errors_dispose(void) {
    stack_walk_dispose();
}

void (*error)(int16_t priority, const char *format, ...) = (void *)0x005512E0;

bool errors_handle(void) {
    bool handled = error_globals.delayed;
    reset_error_state();
    return handled;
}

void write_to_error_file(char *string, bool date) {
    static bool first_line = true;
    if(first_line) {
        char line[KILO];
        first_line = false;
        write_to_error_file(EOL_STRING EOL_STRING, false);
        write_to_error_file(TARGET " " PLATFORM_NAME_STRING " " BUILD_NUMBER CACHE_STRING
            " ----------------------------------------------" EOL_STRING, true);
        sprintf(line, "reference function: %s" EOL_STRING, "_write_to_error_file");
        write_to_error_file(line, true);
        sprintf(line, "reference address: %p" EOL_STRING, write_to_error_file);
        write_to_error_file(line, true);
    }

    if(!error_globals.output_to_debug_file) {
        return;
    }

    FILE *stream = fopen(DEBUG_OUTPUT_FILENAME, "a+b");
    if(!stream) {
        return;
    }

    if(date) {
        time_t time_value;
        struct tm *time_structure;
        time(&time_value);
        time_structure = localtime(&time_value);
        if(time_structure) {
            fprintf(stream, "%02d.%02d.%02d %02d:%02d:%02d  ",
                time_structure->tm_mon + 1,
                time_structure->tm_mday,
                time_structure->tm_year % 100,
                time_structure->tm_hour,
                time_structure->tm_min,
                time_structure->tm_sec);
        }
        else {
            fprintf(stream, "<TIME UNAVAILABLE>  ");
        }
    }

    fprintf(stream, "%s", string);
    fclose(stream);
}

/* private functions */

static void reset_error_state(void) {
    error_globals.delayed = false;
    error_globals.message_buffer_size = 0;
}
