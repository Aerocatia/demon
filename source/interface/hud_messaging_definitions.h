#ifndef DEMON_HUD_MESSAGING_DEFINITIONS_H
#define DEMON_HUD_MESSAGING_DEFINITIONS_H

#include "../cseries/cseries.h"
#include "../tag_files/tag_groups.h"

#include "../input/input.h"
#include "../game/players.h"

enum {
    HUD_MESSAGE_TEXT_DEFINITION_TAG = 0x686D7420, // 'hmt '
    HUD_MESSAGE_TEXT_VERSION = 1,
};

enum {
    MAX_TEXT_MESSAGES = 4,
    MAX_MESSAGE_LENGTH = 63,
    MAX_STATE_TEXT_PANELS = 8,
    MAX_STATE_MESSAGES = KIB,
    MAXIMUM_MESSAGE_STRING_DATA_SIZE = 64 * KIB,
};

enum {
    _hud_message_type_text,
    _hud_message_type_icon,
    NUMBER_OF_HUD_MESSAGE_TYPES
};

enum {
    _hud_icon_specific_button_start,
    _hud_icon_specific_button_end = NUMBER_OF_GAMEPAD_BUTTONS + NUMBER_OF_GAMEPAD_STICKS - 1,
    _hud_icon_remapped_button_start,
    _hud_icon_remapped_button_end = _hud_icon_remapped_button_start + NUMBER_OF_PLAYER_BUTTONS - 1,
    _hud_icon_misc_start,
    _hud_icon_misc_end = _hud_icon_misc_start + INT8_BITS - 1,
    NUMBER_OF_HUD_ICON_TYPES,
    NUMBER_OF_HUD_BUTTON_ICONS = _hud_icon_misc_start,
    NUMBER_OF_HUD_CUSTOM_ICONS = _hud_icon_misc_end - _hud_icon_misc_start + 1,
};

// these can't ever change
static_assert(_hud_icon_remapped_button_start == 18);
static_assert(_hud_icon_remapped_button_end == 31);
static_assert(NUMBER_OF_HUD_ICON_TYPES == 40);
static_assert(NUMBER_OF_HUD_BUTTON_ICONS == 32);
static_assert(NUMBER_OF_HUD_CUSTOM_ICONS == 8);

struct hud_state_message_element {
    uint8_t type;
    uint8_t data;
};
static_assert(sizeof(struct hud_state_message_element) == 2);

struct hud_state_message_definition {
    char name[TAG_STRING_LENGTH + 1];
    uint16_t text_start_index;
    uint16_t element_start_index;
    uint8_t element_count;
    uint8_t pad[3];
    int32_t unused[6];
};
static_assert(sizeof(struct hud_state_message_definition) == 64);

struct hud_state_messages {
    struct tag_data text_data;
    struct tag_block elements;
    struct tag_block messages;
    int32_t unused[21];
};
static_assert(sizeof(struct hud_state_messages) == 128);

/* hud state messaging functions */

static inline struct hud_state_messages *hud_state_messages_get(int32_t index) {
    return tag_get(HUD_MESSAGE_TEXT_DEFINITION_TAG, index);
}

static inline struct hud_state_message_element *hud_state_messages_get_element(struct hud_state_messages *message_group, int32_t index) {
    return tag_block_get_element_with_size(&message_group->elements, index, sizeof(struct hud_state_message_element));
}

static inline struct hud_state_message_definition *hud_state_messages_get_message(struct hud_state_messages *message_group, int32_t index) {
    return tag_block_get_element_with_size(&message_group->messages, index, sizeof(struct hud_state_message_definition));
}

#endif
