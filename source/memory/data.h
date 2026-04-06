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

#define DATUM_IS_USED(datum) ((datum)->identifier)
#define DATUM_IS_FREE(datum) (!DATUM_IS_USED(datum))
#define MARK_DATUM_AS_FREE(datum) ((datum)->identifier = 0)

#define BUILD_DATUM_INDEX(identifier, absolute_index) \
    ((uint32_t)(absolute_index) | ((uint32_t)(identifier)<<SHORT_BITS))
#define DATUM_INDEX_TO_IDENTIFIER(index) ((index)>>SHORT_BITS)
#define DATUM_INDEX_TO_ABSOLUTE_INDEX(index) ((index)&((1<<SHORT_BITS)-1))

#endif
