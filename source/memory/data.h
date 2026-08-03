#ifndef DEMON_DATA_H
#define DEMON_DATA_H

#include "../cseries/cseries.h"

struct datum_header {
    int16_t identifier;
};
static_assert(sizeof(struct datum_header) == 2);

struct data_array {
    char name[32];
    int16_t maximum_count;
    int16_t size;
    bool valid;
    bool identifier_zero_invalid;
    tag signature;
    int16_t first_free_absolute_index;
    int16_t count;
    int16_t actual_count;
    int16_t next_identifier;
    void *data;
};
static_assert(sizeof(struct data_array) == 56);

struct data_iterator {
    struct data_array *data;
    int16_t absolute_index;
    int32_t index;
    tag signature;
};
static_assert(sizeof(struct data_iterator) == 16);

#ifdef DEBUG_BUILD
void data_verify(struct data_array *data);
#else
#define data_verify(data) ((void)0)
#endif

struct data_array *data_new(const char *name, int16_t maximum_count, int16_t size);
void data_destroy(struct data_array *data);
void data_dispose(struct data_array *data);

int32_t data_allocation_size(int16_t maximum_count, int16_t size);
void data_initialize(struct data_array *data, const char *name, int16_t maximum_count, int16_t size);

void data_make_valid(struct data_array *data);
void data_make_invalid(struct data_array *data);

int32_t datum_new(struct data_array *data);
int32_t datum_new_at_index(struct data_array *data, int32_t index);
int32_t datum_new_at_plain_index_hack_for_player_data(struct data_array *data, int32_t index);
void datum_delete(struct data_array *data, int32_t index);
void *datum_get(struct data_array *data, int32_t index);
void *datum_try_and_get(struct data_array *data, int32_t index);

void data_delete_all(struct data_array *data);
void data_compact(struct data_array *data);

void data_iterator_new(struct data_iterator *iterator, struct data_array *data);
void *data_iterator_next(struct data_iterator *iterator);

int32_t data_next_index(struct data_array *data, int32_t index);
int32_t data_prev_index(struct data_array *data, int32_t index);
int32_t data_last_index(struct data_array *data);

/* inline functions */

static inline bool datum_is_used(struct datum_header *header) {
    return header->identifier != 0;
}

static inline bool datum_is_free(struct datum_header *header) {
    return header->identifier == 0;
}

static inline void mark_datum_as_free(struct datum_header *header) {
    header->identifier = 0;
}

static inline int32_t build_datum_index(int16_t identifier, int16_t absolute_index) {
    return (uint32_t)absolute_index | ((uint32_t)identifier << INT16_BITS);
}

static inline int16_t datum_index_to_identifier(int32_t datum_index) {
    return datum_index >> INT16_BITS;
}

static inline int16_t datum_index_to_absolute_index(int32_t datum_index) {
    return datum_index & ((1 << INT16_BITS) -1);
}

#endif
