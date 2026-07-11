#ifndef DEMON_PLATFORM_H
#define DEMON_PLATFORM_H

// We need 1-byte bools
static_assert(sizeof(bool) == 1);

#ifndef TARGET_STRING
#error "TARGET_STRING is not defined"
#endif

#if defined(__BYTE_ORDER__) && (__BYTE_ORDER__ == __ORDER_BIG_ENDIAN__)
    #define big_endian
#elif defined(__BYTE_ORDER__) && (__BYTE_ORDER__ == __ORDER_LITTLE_ENDIAN__)
    #define little_endian
#else
    #error "unknown target architecture"
#endif

#ifdef _WIN32
    #define PLATFORM_NAME_STRING "pc"
    #define EOL_STRING "\r\n"
#else
    #define PLATFORM_NAME_STRING "unknown"
    #define EOL_STRING "\n"
#endif

#endif
