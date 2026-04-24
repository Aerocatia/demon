#include <string.h>

#include "../cseries/cseries.h"
#include "data.h"

#define DATA_SIGNATURE 0x64407440 // 'd@t@'
#define DATA_ITERATOR_SIGNATURE 0x69746572 // 'iter'

void data_initialize(struct data_array *data, const char *name, int16_t maximum_count, int16_t size) {
    assert(maximum_count > 0);
    assert(size > 0);
    assert(name);
    assert(data);

    memset(data, 0, sizeof(struct data_array));
    strncpy(data->name, name, sizeof(data->name) -1);
    data->maximum_count = maximum_count;
    data->size = size;
    data->signature = DATA_SIGNATURE;
    data->data = data + 1;
    data->valid = false;
}

void data_make_valid(struct data_array *data) {
    data_verify(data);

    data->valid = true;
    data_delete_all(data);
}

void data_make_invalid(struct data_array *data) {
    data_verify(data);

    data->valid = false;
}

void data_delete_all(struct data_array *data) {
    data_verify(data);
    assert(data->valid);

    data->count = 0;
    data->actual_count = 0;
    data->first_free_absolute_index = 0;
    strncpy((char *)&data->next_identifier, data->name, sizeof(data->next_identifier));
    data->next_identifier |= (int16_t)0x8000;

    for(int16_t index = 0; index < data->maximum_count; ++index) {
        struct datum_header *header = (struct datum_header *)((uint8_t *)data->data + index * data->size);
        MARK_DATUM_AS_FREE(header);
    }
}

#ifdef DEBUG
void data_verify(struct data_array *data) {
    assert(data);

    if(data->data &&
        data->signature == DATA_SIGNATURE &&
        data->maximum_count >= 0 &&
        data->count >= 0 && data->count <= data->maximum_count &&
        data->first_free_absolute_index >= 0 && data->first_free_absolute_index <= data->maximum_count &&
        data->actual_count >= 0 && data->actual_count <= data->count
    ) {
        return;
    }

    vhalt(csprintf(temporary, "%s data array @%p is bad or not allocated", data->name, data));
}
#endif
