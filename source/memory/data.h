#ifndef __DATA_H__
#define __DATA_H__

#define DATUM_HEADER \
    int16_t identifier

struct datum_header {
    DATUM_HEADER;
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

#define DATUM_IS_USED(datum) ((datum)->identifier)
#define DATUM_IS_FREE(datum) (!DATUM_IS_USED(datum))
#define MARK_DATUM_AS_FREE(datum) ((datum)->identifier = 0)

#define BUILD_DATUM_INDEX(identifier, absolute_index) \
    ((uint32_t)(absolute_index) | ((uint32_t)(identifier) << SHORT_BITS))
#define DATUM_INDEX_TO_IDENTIFIER(index) ((index) >> SHORT_BITS)
#define DATUM_INDEX_TO_ABSOLUTE_INDEX(index) ((index) & ((1 << SHORT_BITS) -1))

#define DATUM_TRY_AND_GET(data_array, index, structure) ((structure *)datum_try_and_get((data_array), (index)))

#ifdef DEBUG
    #define DATUM_GET(data_array, index, structure) ((structure *)datum_get((data_array), (index)))
    #define DATUM_GET_BY_SIZE(data_array, index, structure) ((structure *)datum_get((data_array), (index)))
    void data_verify(struct data_array *data);
#else
    #define DATUM_GET(data_array, index, structure) ((structure *)(((data_array)->data)) + DATUM_INDEX_TO_ABSOLUTE_INDEX(index))
    #define DATUM_GET_BY_SIZE(data_array, index, structure) ((structure *)(((data_array)->data)) + (data_array)->size * DATUM_INDEX_TO_ABSOLUTE_INDEX(index))
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
void datum_delete(struct data_array *data, int32_t index);
void *datum_get(struct data_array *data, int32_t index);

void data_delete_all(struct data_array *data);

void data_iterator_new(struct data_iterator *iterator, struct data_array *data);
void *data_iterator_next(struct data_iterator *iterator);

int32_t data_next_index(struct data_array *data, int32_t index);
int32_t data_prev_index(struct data_array *data, int32_t index);

#endif
