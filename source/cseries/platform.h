#ifndef __PLATFORM_H__
#define __PLATFORM_H__

#ifndef TARGET
#error "TARGET is not defined"
#endif

#ifdef _WIN32
    #define PLATFORM_NAME_STRING "pc"
    #define EOL_STRING "\r\n"
#else
    #define PLATFORM_NAME_STRING "unknown"
    #define EOL_STRING "\n"
#endif

#endif
