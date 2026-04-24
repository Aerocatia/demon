#include "../cseries/cseries.h"

#include "data.h"

#define DATA_SIGNATURE 0x64407440 // 'd@t@'
#define DATA_ITERATOR_SIGNATURE 0x69746572 // 'iter'

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
