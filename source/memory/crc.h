#ifndef DEMON_CRC_H
#define DEMON_CRC_H

#include <stdint.h>

void crc_new(uint32_t *crc_reference);
void crc_checksum_buffer(uint32_t *crc_reference, const void *buffer, int32_t buffer_size);

#endif
