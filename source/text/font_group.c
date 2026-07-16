#include <stdint.h>

#include "../cseries/cseries.h"

#include "font_group.h"

struct font_character *font_get_character_by_ascii_code(struct font_header *header, uint16_t character) {
    auto tables_entry = font_get_character_tables_entry(header, HIGH_BYTE(character));
    if(tables_entry->table.count > 0) {
        auto table_entry = font_get_character_table_entry(tables_entry, LOW_BYTE(character));
        auto character_index = table_entry ? table_entry->character_index : NONE; // original does not check this
        if(character_index != NONE) {
            return font_get_character(header, character_index);
        }
    }

    return nullptr;
}
