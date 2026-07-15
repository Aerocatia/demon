#ifndef DEMON_INPUT_H
#define DEMON_INPUT_H

enum {
    FIRST_GAMEPAD_ANALOG_BUTTON,
    _gamepad_analog_button_a = FIRST_GAMEPAD_ANALOG_BUTTON,
    _gamepad_analog_button_b,
    _gamepad_analog_button_x,
    _gamepad_analog_button_y,
    _gamepad_analog_button_black,
    _gamepad_analog_button_white,
    _gamepad_analog_button_left_trigger,
    _gamepad_analog_button_right_trigger,
    NUMBER_OF_GAMEPAD_ANALOG_BUTTONS,
    FIRST_GAMEPAD_BINARY_BUTTON = NUMBER_OF_GAMEPAD_ANALOG_BUTTONS,
    _gamepad_binary_button_dpad_up = FIRST_GAMEPAD_BINARY_BUTTON,
    _gamepad_binary_button_dpad_down,
    _gamepad_binary_button_dpad_left,
    _gamepad_binary_button_dpad_right,
    _gamepad_binary_button_start,
    _gamepad_binary_button_back,
    _gamepad_binary_button_left_thumb,
    _gamepad_binary_button_right_thumb,
    NUMBER_OF_GAMEPAD_BUTTONS,
    NUMBER_OF_GAMEPAD_BINARY_BUTTONS = NUMBER_OF_GAMEPAD_BUTTONS - NUMBER_OF_GAMEPAD_ANALOG_BUTTONS
};
static_assert(NUMBER_OF_GAMEPAD_BUTTONS == 16);
static_assert(NUMBER_OF_GAMEPAD_BINARY_BUTTONS == 8);

enum {
    _gamepad_stick_left,
    _gamepad_stick_right,
    NUMBER_OF_GAMEPAD_STICKS
};

#endif
