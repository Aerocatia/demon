#ifndef __PLATFORM_H__
#define __PLATFORM_H__

#if defined(DEBUG)
#elif defined(NO_DEBUG)
#else
    #error "must have DEBUG or NO_DEBUG defined"
#endif

#ifndef TARGET
#error "TARGET is not defined"
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
