#ifndef __CONSOLE_H__
#define __CONSOLE_H__

void (*console_printf)(bool clear, const char *format, ...) = (void *)0x006ACDF0;

#endif
