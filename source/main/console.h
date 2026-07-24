#ifndef DEMON_CONSOLE_H
#define DEMON_CONSOLE_H

// FIXME_EXE_FUNCTION_POINTER
extern void (*console_printf)(bool clear, const char *format, ...);
extern void (*console_warning)(const char *format, ...);

#endif
