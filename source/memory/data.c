#include <string.h>

#include "../cseries/cseries.h"
#include "data.h"

#define DATA_SIGNATURE 0x64407440 // 'd@t@'
#define DATA_ITERATOR_SIGNATURE 0x69746572 // 'iter'

static void datum_initialize(struct data_array *data, struct datum_header *header);

struct data_array *data_new(const char *name, int16_t maximum_count, int16_t size) {
    struct data_array *data = malloc(data_allocation_size(maximum_count, size));

    if(data) {
        data_initialize(data, name, maximum_count, size);
    }

    return data;
}

int32_t data_allocation_size(int16_t maximum_count, int16_t size) {
    return sizeof(struct data_array) + maximum_count * size;
}

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

// Name is a guess. Split from data_dispose in the 2020 Custom Edition debug EXE.
void data_destroy(struct data_array *data) {
    data_verify(data);

    memset(data, 0, sizeof(struct data_array));
}

void data_dispose(struct data_array *data) {
    data_destroy(data);
    free(data);
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

int32_t datum_new_at_index(struct data_array *data, int32_t index) {
    data_verify(data);
    assert(data->valid);

    int16_t absolute_index = DATUM_INDEX_TO_ABSOLUTE_INDEX(index);
    int16_t identifier = DATUM_INDEX_TO_IDENTIFIER(index);
    if(absolute_index < 0 && absolute_index > data->maximum_count && !identifier) {
        return NONE;
    }

    struct datum_header *header = (struct datum_header *)((uint8_t *)data->data + absolute_index * data->size);
    if(!DATUM_IS_FREE(header)) {
        return NONE;
    }

    data->actual_count += 1;
    if(absolute_index >= data->count) {
        data->count = absolute_index + 1;
    }

    datum_initialize(data, header);
    header->identifier = identifier;

    return index;
}

int32_t datum_new(struct data_array *data) {
    data_verify(data);
    assert(data->valid);

    int16_t absolute_index = data->first_free_absolute_index;
    void *cursor = data->data + absolute_index * data->size;
    struct datum_header *header;
    while(true) {
        if(absolute_index >= data->maximum_count) {
            return NONE;
        }

        header = cursor;
        if(DATUM_IS_FREE(header)) {
            break;
        }

        absolute_index++;
        cursor += data->size;
    }

    datum_initialize(data, header);
    data->actual_count += 1;
    data->first_free_absolute_index = absolute_index + 1;
    if(absolute_index >= data->count) {
        data->count = absolute_index + 1;
    }

    return BUILD_DATUM_INDEX(header->identifier, absolute_index);
}

void datum_delete(struct data_array *data, int32_t index) {
    void *cursor = datum_get(data, index);
    struct datum_header *header = cursor;
    MARK_DATUM_AS_FREE(header);

    int16_t absolute_index = DATUM_INDEX_TO_ABSOLUTE_INDEX(index);
    if(absolute_index < data->first_free_absolute_index) {
        data->first_free_absolute_index = absolute_index;
    }

    if(absolute_index + 1 == data->count) {
        do {
            header = cursor -= data->size;
            data->count -= 1;
        }
        while(data->count > 0 && DATUM_IS_FREE(header));
    }

    data->actual_count -= 1;
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

void *datum_get(struct data_array *data, int32_t index) {
    assert(data->valid);

    int16_t identifier = DATUM_INDEX_TO_IDENTIFIER(index);
    assert(identifier || !data->identifier_zero_invalid);

    int16_t absolute_index = DATUM_INDEX_TO_ABSOLUTE_INDEX(index);
    if(absolute_index >= 0 && absolute_index < data->count) {
        struct datum_header *header = (struct datum_header *)((uint8_t *)data->data + absolute_index * data->size);
        if(DATUM_IS_USED(header) && (!identifier || identifier == header->identifier)) {
            return header;
        }
    }

    vhalt(csprintf(temporary, "%s index #%d (0x%x) is unused or changed",
        data->name, DATUM_INDEX_TO_ABSOLUTE_INDEX(index), index));

    return nullptr;
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

static void datum_initialize(struct data_array *data, struct datum_header *header) {
    memset(header, 0, data->size);
    header->identifier = data->next_identifier++;
    if(!data->next_identifier) {
        data->next_identifier = SHORT_MIN;
    }
}
