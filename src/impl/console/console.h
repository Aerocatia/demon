#ifndef DEMON__IMPL_CONSOLE_H
#define DEMON__IMPL_CONSOLE_H

#include <stdbool.h>

struct ColorARGB;

/**
 * Set console prompt display parameters. This is run on startup.
 */
void set_console_prompt_display_params(void);

/**
 * Print a formatted line of text to the console.
 *
 * @param color (optional)
 * @param fmt   format
 */
void console_printf(const struct ColorARGB *color, const char *fmt, ...);

/**
 * Print a red formatted line of text to the console.
 *
 * The syntax is the same as console_printf except no color is passed.
 */
#define console_printf_debug_err(...) { ColorARGB dbg_err_color_print = { 1.0, 1.0, 0.1, 0.1 }; console_printf(&dbg_err_color_print, __VA_ARGS__); }

/**
 * Check if a command is allowed.
 *
 * TODO: This is not fully understood yet.
 *
 * @return true if allowed, false if not
 */
bool command_is_allowed(uint8_t a);

struct GenericReflexive;

/**
 * Load names from the reflexive.
 *
 * @param reflexive    reflexive to search
 * @param name_offset  offset to the name in an entry
 * @param element_size size of each entry
 */
void load_names_from_reflexive(const struct GenericReflexive *reflexive, uint32_t name_offset, uint32_t element_size);

#endif
