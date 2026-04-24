#ifndef __DATA_H__
#define __DATA_H__

#define DATUM_HEADER \
    int16_t identifier

struct datum_header {
    DATUM_HEADER;
};

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
static_assert(sizeof(struct data_array) == 0x38);

#define DATUM_IS_USED(datum) ((datum)->identifier)
#define DATUM_IS_FREE(datum) (!DATUM_IS_USED(datum))
#define MARK_DATUM_AS_FREE(datum) ((datum)->identifier = 0)

#define BUILD_DATUM_INDEX(identifier, absolute_index) \
    ((uint32_t)(absolute_index) | ((uint32_t)(identifier)<<SHORT_BITS))
#define DATUM_INDEX_TO_IDENTIFIER(index) ((index)>>SHORT_BITS)
#define DATUM_INDEX_TO_ABSOLUTE_INDEX(index) ((index)&((1<<SHORT_BITS)-1))

#define DATUM_TRY_AND_GET(data_array, index, structure) ((structure *)datum_try_and_get((data_array), (index)))

#ifdef DEBUG
    #define DATUM_GET(data_array, index, structure) ((structure *)datum_get((data_array), (index)))
    #define DATUM_GET_BY_SIZE(data_array, index, structure) ((structure *)datum_get((data_array), (index)))
    void data_verify(struct data_array *data);
#else
    #define DATUM_GET(data_array, index, structure) ((structure *)(((data_array)->data))+DATUM_INDEX_TO_ABSOLUTE_INDEX(index))
    #define DATUM_GET_BY_SIZE(data_array, index, structure) ((structure *)(((data_array)->data))+(data_array)->size * DATUM_INDEX_TO_ABSOLUTE_INDEX(index))
    #define data_verify(data) ((void)0)
#endif

#endif
