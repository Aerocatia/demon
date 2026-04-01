#ifndef __CSERIES_H__
#define __CSERIES_H__

#include <stdint.h>
#include <stdlib.h>

/* ---------- constants */

#define NONE -1

#define KILO 1024
#define MEG (KILO * KILO)
#define GIG (KILO * MEG)

/* ---------- macros */

/* ---------- types */

typedef uint32_t tag;
typedef float real;

#undef LONG_MAX
#undef LONG_MIN
#undef CHAR_MAX
#undef CHAR_MIN

enum {
    UNSIGNED_LONG_MAX = 4294967295,
    LONG_MAX = 2147483647L,
    LONG_MIN = -LONG_MAX - 1L,
    LONG_BITS = 32,
    LONG_BITS_BITS = 5,

    UNSIGNED_SHORT_MAX = 65535,
    SHORT_MAX = 32767,
    SHORT_MIN = -SHORT_MAX - 1,
    SHORT_BITS = 16,
    SHORT_BITS_BITS = 4,

    UNSIGNED_CHAR_MAX = 255,
    CHAR_MAX = 127,
    CHAR_MIN = -CHAR_MAX - 1,
    CHAR_BITS = 8,
    CHAR_BITS_BITS = 3
};

/* ---------- structures */

typedef union {
    uint8_t n[6];
    struct {
        uint8_t x0, x1;
        uint8_t y0, y1;
        uint8_t z0, z1;
    };
} byte_rectangle3d;

typedef union {
    int16_t n[2];
    struct {
        int16_t x, y;
    };
} point2d;

typedef union {
    int16_t n[4];
    struct {
        int16_t y0, x0, y1, x1;
    };
} rectangle2d;

typedef union {
    uint16_t n[3];
    struct {
        uint16_t red, green, blue;
    };
} rgb_color;

typedef union {
    uint16_t n[4];
    struct {
        uint16_t alpha;
        union {
            rgb_color rgb;
            struct {
                uint16_t red, green, blue;
            };
        };
    };
} argb_color;

typedef union {
    uint16_t n[3];
    struct {
        uint16_t hue, saturation, value;
    };
} hsv_color;

typedef union {
    uint16_t n[4];
    struct {
        uint16_t alpha;
        union {
            hsv_color hsv;
            struct {
                uint16_t hue, saturation, value;
            };
        };
    };
} ahsv_color;

typedef union {
    real n[3];
    struct {
        real red, green, blue;
    };
} real_rgb_color;

typedef union {
    real n[4];
    struct {
        real alpha;
        union {
            real_rgb_color rgb;
            struct {
                real red, green, blue;
            };
        };
    };
} real_argb_color;

typedef union {
    real n[3];
    struct {
        real hue, saturation, value;
    };
} real_hsv_color;

typedef union {
    real n[4];
    struct {
        real alpha;
        union {
            real_hsv_color hsv;
            struct {
                real hue, saturation, value;
            };
        };
    };
} real_ahsv_color;

typedef union {
    real n[2];
    struct {
        real x, y;
    };
    struct {
        real u, v;
    };
} real_point2d;

typedef union {
    real n[3];
    struct {
        real x, y, z;
    };
    struct {
        real u, v, w;
    };
} real_point3d;

typedef union {
    real n[2];
    struct {
        real i, j;
    };
} real_vector2d;

typedef union {
    real n[3];
    struct {
        real i, j, k;
    };
} real_vector3d;

typedef union {
    real n[4];
    struct {
        real i, j, k, l;
    };
} real_vector4d;

typedef struct {
    real_vector2d n;
    real d;
} real_plane2d;

typedef struct {
    real_vector3d n;
    real d;
} real_plane3d;

typedef union {
    real n[4];
    struct {
        real x0, x1;
        real y0, y1;
    };
} real_rectangle2d;

typedef union {
    real n[6];
    struct {
        real x0, x1;
        real y0, y1;
        real z0, z1;
    };
} real_rectangle3d;

typedef struct {
    real_vector3d v;
    real w;
} real_quaternion;

typedef struct {
    real_quaternion rotation;
    real_point3d translation;
    real scale;
} real_orientation;

typedef union {
    real n[2];
    struct {
        real yaw, pitch;
    };
} real_euler_angles2d;

typedef union {
    real n[3];
    struct {
        real yaw, pitch, roll;
    };
} real_euler_angles3d;

typedef struct {
    union {
        real n[3][3];
        struct {
            real_vector3d forward;
            real_vector3d left;
            real_vector3d up;
        };
    };
} real_matrix3x3;

typedef struct {
    real scale;
    union {
        real n[4][3];
        struct {
            real_vector3d forward;
            real_vector3d left;
            real_vector3d up;
            real_point3d position;
        };
    };
} real_matrix4x3;

/* ---------- asserts */

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

/* ---------- globals */

#if DEBUG
extern char temporary[256];
#endif

/* ---------- prototypes */

tag string_to_tag(const char *s);
char *tag_to_string(tag t, char *s);

void system_exit(int code);

#endif
