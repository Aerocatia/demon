#ifndef DEMON_DRAW_STRING_H
#define DEMON_DRAW_STRING_H

#include "../cseries/cseries.h"

enum {
    _text_style_plain = NONE,
    _text_style_bold = 0,
    _text_style_italic,
    _text_style_condense,
    _text_style_underline,
    NUMBER_OF_TEXT_STYLES
};

enum {
    _text_justification_left,
    _text_justification_right,
    _text_justification_center,
    NUMBER_OF_TEXT_JUSTIFICATIONS
};

enum {
    _draw_text_wrap_horizontally_bit,
    _draw_text_wrap_vertically_bit,
    _draw_text_center_vertically_bit,
    _draw_text_bottom_justify_bit,
    NUMBER_OF_TEXT_FLAGS
};

enum {
    _text_string_checkmark,
    NUMBER_OF_TEXT_STRINGS
};

#endif
