#include <stdint.h>
#include <stdio.h>
#include <time.h>

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

void (*error)(int16_t priority, const char *format, ...) = (void *)0x005512E0;

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

    // FIXME: use error_globals
    bool output_to_debug_file = *(bool *)0x00B017CD;
    if(!output_to_debug_file /* error_globals.output_to_debug_file */) {
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
