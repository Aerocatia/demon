#ifndef DEMON_ERRORS_H
#define DEMON_ERRORS_H

enum {
    _error_immediate,
    _error_delayed,
    _error_silent,
    _error_log,
    NUMBER_OF_ERROR_MESSAGE_PRIORITIES
};

enum {
    _developer_mode_off,
    _developer_mode_errors_enabled,
    _developer_mode_logging_enabled,
    _developer_mode_metrics_enabled,
    _developer_mode_conprint_enabled,
    _developer_mode_full = 127
};

extern bool find_all_fucked_up_shit;
extern int32_t fucked_up_shit_count;

void errors_initialize(void);
void errors_dispose(void);

bool errors_handle(void);

void errors_output_to_debug_file(bool output_to_debug_file);
void errors_overflow_suppression_enable(bool overflow_suppression);
void errors_suppress_all_enable(bool suppress_all);

extern void (*error)(int16_t priority, const char *format, ...);

const char *error_get(void);
void errors_clear();

void write_to_error_file(char *string, bool date);

extern void (*stack_walk_initialize)(void);
extern void (*stack_walk_dispose)(void);

#endif
