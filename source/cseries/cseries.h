#ifndef __CSERIES_H__
#define __CSERIES_H__

#include <stdint.h>
#include <stdlib.h>

#define NONE -1

#define KILO 1024
#define MEG (KILO * KILO)
#define GIG (KILO * MEG)

typedef uint32_t tag;

#ifdef assert
#undef assert
#endif

#ifdef DEBUG
    void display_assert(char *string, char *file, int32_t line, bool fatal);
    void display_debug_string(char *string);
    char *csprintf(char *buffer, char *format, ...);

    #ifndef NO_DEBUG_TRAP
        #ifdef _MSC_VER
            #define enter_debugger() __debugbreak()
        #elif defined(__GNUC__)
            #define enter_debugger() __builtin_trap()
        #else
            #define enter_debugger() ((void)0)
        #endif
    #else
        #define enter_debugger() ((void)0)
    #endif

    #define halt() { display_assert((char *)nullptr, __FILE__, __LINE__, true); enter_debugger(); system_exit(-1); }
    #define vhalt(diag) { display_assert(diag, __FILE__, __LINE__, true); enter_debugger(); system_exit(-1); }
    #define assert(expr) if(!(expr)) { display_assert(#expr, __FILE__, __LINE__, true); enter_debugger(); system_exit(-1); }
    #define vassert(expr,diag) if(!(expr)) { display_assert(diag, __FILE__, __LINE__, true); enter_debugger(); system_exit(-1); }
    #define pause() { display_assert((char *)nullptr, __FILE__, __LINE__, false); enter_debugger(); }
    #define vpause(diag) { display_assert(diag, __FILE__, __LINE__, false); enter_debugger(); }
    #define warn(expr) ((expr) ? true : (display_assert(#expr, __FILE__, __LINE__, false), false))
    #define vwarn(expr,diag) ((expr) ? true : (display_assert(diag, __FILE__, __LINE__, false), false))
    #define vwarn_trace(expr,diag) ((expr) ? true : (display_assert(diag, __FILE__, __LINE__, true), false))
    #undef unreachable
    #define unreachable() do { assert(!"unreachable"); } while(0)
#else
    #define display_debug_string(string) ((void)0)
    #define halt() ((void)0)
    #define vhalt(diag) ((void)0)
    #define assert(expr) ((void)0)
    #define vassert(expr,diag) ((void)0)
    #define pause() ((void)0)
    #define vpause(diag) ((void)0)
    #define warn(expr) (expr) ((void)0)
    #define vwarn(expr,diag) (expr) ((void)0)
    #define vwarn_trace(expr,diag) (expr) ((void)0)
    // #define unreachable() ((void)0) - Use C23 macro
#endif

#if DEBUG
extern char temporary[256];
#endif

void system_exit(int code);

#endif
