#ifndef DEMON_ERRORS_H
#define DEMON_ERRORS_H

enum {
    _error_immediate,
    _error_delayed,
    _error_silent,
    _error_log,
    NUMBER_OF_ERROR_MESSAGE_PRIORITIES
};

extern void (*error)(int16_t priority, const char *format, ...);

void write_to_error_file(char *string, bool date);

#endif
