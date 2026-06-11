#include <stdarg.h>
#include <stdint.h>
#include <stdio.h>
#include <time.h>
#include <string.h>

#include "cseries.h"
#include "build_number.h"
#include "platform.h"

#include "errors.h"

#include "../interface/terminal.h"

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

// halo_cache_symbols.exe
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

void error(short priority, const char *format, ...) {
    assert(priority >= 0 && priority < NUMBER_OF_ERROR_MESSAGE_PRIORITIES);

    // No.
    if(error_globals.suppress_all && priority == _error_silent) {
        return;
    }

    if(error_globals.overflow_suppression && priority == _error_silent) {
        static int32_t old_time = 0;
        static int32_t count = 0;
        int32_t current_time = system_milliseconds();
        int32_t timeout = TICKS_PER_SECOND * 30;
        int32_t max_count = 10;

        if(current_time > (timeout + old_time)) {
            count = 0;
        }

        old_time = current_time;
        if(count == max_count) {
            terminal_printf(global_real_argb_white, "too many errors, only printing to debug.txt");
        }

        count++;
        if(count >= max_count) {
            priority = _error_log;
        }
    }

    if(error_globals.recursion_lock) {
        return;
    }

    error_globals.recursion_lock = true;
    if(priority == _error_delayed) {
        error_globals.delayed = true;
    }

    if(format) {
        char buffer[ERROR_MESSAGE_BUFFER_MAXIMUM_SIZE + 4];

        va_list arglist;
        va_start(arglist, format);
        vsnprintf(buffer, ERROR_MESSAGE_BUFFER_MAXIMUM_SIZE, format, arglist);
        va_end(arglist);

        strcat(buffer, EOL_STRING);

#ifdef SHELL_CONSOLE
        fprintf(stderr, buffer);
#endif

//FIXME not in release builds
//#ifdef SYMBOLS
        display_debug_string(buffer);
//#endif
        if(priority != _error_log) {
            terminal_printf(global_real_argb_white, "%s", buffer);
        }

        write_to_error_file(buffer, true);

        int32_t new_size = strlen(buffer);
        if(error_globals.message_buffer_size + new_size >= ERROR_MESSAGE_BUFFER_MAXIMUM_SIZE) {
            const char *prefix = "[...too many errors to print...]" EOL_STRING;
            int32_t prefix_size = strlen(prefix);

            char *start_ptr = error_globals.message_buffer +
                PIN(ERROR_MESSAGE_BUFFER_MAXIMUM_SIZE / 2 + prefix_size + new_size, 0, error_globals.message_buffer_size - 1);
            char *copy_ptr = strchr(start_ptr, '\n');

            int32_t copy_index;
            if(copy_ptr == nullptr) {
                copy_index = error_globals.message_buffer_size;
            }
            else {
                copy_index = (copy_ptr - error_globals.message_buffer) + 1;
            }

            int32_t copy_size = error_globals.message_buffer_size - copy_index;
            assert(prefix_size + copy_size + new_size < ERROR_MESSAGE_BUFFER_MAXIMUM_SIZE);

            strncpy(error_globals.message_buffer, prefix, prefix_size);
            if(copy_size > 0) {
                memmove(error_globals.message_buffer + prefix_size, copy_ptr, copy_size);
            }

            error_globals.message_buffer[prefix_size + copy_size] = '\0';
            error_globals.message_buffer_size = prefix_size + copy_size;
        }

        if(error_globals.message_buffer_size + new_size < ERROR_MESSAGE_BUFFER_MAXIMUM_SIZE) {
            strcpy(error_globals.message_buffer + error_globals.message_buffer_size, buffer);
            error_globals.message_buffer_size += new_size;
        }
    }

    if(priority == _error_immediate) {
        system_exit(-4998);
    }

    error_globals.recursion_lock = false;
}

bool errors_handle(void) {
    bool handled = error_globals.delayed;
    reset_error_state();
    return handled;
}

void errors_output_to_debug_file(bool output_to_debug_file) {
    error_globals.output_to_debug_file = output_to_debug_file;
}

void errors_overflow_suppression_enable(bool overflow_suppression) {
    error_globals.overflow_suppression = overflow_suppression;
}

void errors_suppress_all_enable(bool suppress_all) {
    error_globals.suppress_all = suppress_all;
}

const char *error_get(void) {
    return error_globals.message_buffer;
}

void errors_clear(void) {
    reset_error_state();
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
