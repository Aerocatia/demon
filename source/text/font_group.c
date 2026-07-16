#include <stdint.h>

#include "../cseries/cseries.h"

#include "font_group.h"

struct font_character *font_get_character_by_ascii_code(struct font_header *header, uint16_t character) {
    struct font_character_tables_entry *character_tables_entry = font_get_character_tables_entry(header, HIGH_BYTE(character));
    if(character_tables_entry->table.count > 0) {
        struct font_character_table_entry *entry = font_get_character_table_entry(character_tables_entry, LOW_BYTE(character));
        int16_t character_index = entry ? entry->character_index : NONE; // original does not check this
        if(character_index != NONE) {
            return font_get_character(header, character_index);
        }
    }

    return nullptr;
}
