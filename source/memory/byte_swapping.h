#ifndef DEMON_BYTE_SWAPPING_H
#define DEMON_BYTE_SWAPPING_H

#include <stdint.h>

#include "../cseries/platform.h"

#define SWAP2(q) ((((uint16_t)(q))>>8) | ((((uint16_t)(q))<<8)))
#define SWAP4(q) (((((uint32_t) (q)))>>24) | ((((uint32_t) (q))>>8)&0xff00) | ((((uint32_t) (q))<<8)&0xff0000) | ((((uint32_t) (q))<<24)))
#define SWAP8(q) (((uint64_t)(q)>>56) | (((uint64_t)(q)>>40)&0xff00) | (((uint64_t)(q)>>24)&0xff0000) | (((uint64_t)(q)>>8)&0xff000000) | \
(((uint64_t)(q)<<8)&0xff00000000) | (((uint64_t)(q)<<24)&0xff0000000000) | (((uint64_t)(q)<<40)&0xff000000000000) | ((uint64_t)(q)<<56))

#if defined(big_endian)
    #define SWAP2_BE(q) q
    #define SWAP4_BE(q) q
    #define SWAP8_BE(q) q
    #define SWAP2_LE(q) SWAP2(q)
    #define SWAP4_LE(q) SWAP4(q)
    #define SWAP8_LE(q) SWAP8(q)
#elif defined(little_endian)
    #define SWAP2_BE(q) SWAP2(q)
    #define SWAP4_BE(q) SWAP4(q)
    #define SWAP8_BE(q) SWAP8(q)
    #define SWAP2_LE(q) q
    #define SWAP4_LE(q) q
    #define SWAP8_LE(q) q
#else
    #error "must be big or little endian"
#endif

#endif
