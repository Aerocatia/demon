#ifndef DEMON_CSERIES_H
#define DEMON_CSERIES_H

#include <stdint.h>
#include <stdlib.h>

/* constants */

#define NONE -1

#define KIB 1024
#define MIB (KIB * KIB)
#define GIB (KIB * MIB)

#define INT32_BITS 32
#define INT32_BITS_BITS 5
#define INT16_BITS 16
#define INT16_BITS_BITS 4
#define INT8_BITS 8
#define INT8_BITS_BITS 3

enum {
    MILLISECONDS_PER_SECOND = 1000,
    SECONDS_PER_MINUTE = 60,
    MINUTES_PER_HOUR = 60,
    HOURS_PER_DAY = 24,
    VBLANKS_PER_SECOND = 60,
    TICKS_PER_SECOND = 30,
    TICKS_PER_MINUTE = TICKS_PER_SECOND * SECONDS_PER_MINUTE,
    TICKS_PER_HOUR = TICKS_PER_MINUTE * MINUTES_PER_HOUR,
    TICKS_PER_DAY = TICKS_PER_HOUR * HOURS_PER_DAY
};

#define TICKS_TO_SECONDS(t) ((t) * (1.0f / TICKS_PER_SECOND))
#define SECONDS_TO_TICKS(s) ((s) * TICKS_PER_SECOND)

enum {
    _x,
    _y,
    _z,
    NUMBER_OF_RECTANGLE2D_COMPONENTS = 4,
    NUMBER_OF_RECTANGLE3D_COMPONENTS = 6,
    NUMBER_OF_VERTICES_PER_LINE = 2,
    NUMBER_OF_VERTICES_PER_TRIANGLE = 3,
    NUMBER_OF_VERTICES_PER_QUADRILATERAL = 4,
    NUMBER_OF_VERTICES_PER_HEXAGON = 6,
    NUMBER_OF_VERTICES_PER_PYRAMID = 5,
    NUMBER_OF_VERTICES_PER_CUBE = 8,
    NUMBER_OF_TRIANGLES_PER_QUADRILATERAL = 2,
    NUMBER_OF_EDGES_PER_TRIANGLE = 3,
    NUMBER_OF_EDGES_PER_QUADRILATERAL = 4,
    NUMBER_OF_EDGES_PER_HEXAGON = 6,
    NUMBER_OF_FACES_PER_CUBE = 6,
    _rectangle_top_left = 0,
    _rectangle_bottom_left,
    _rectangle_top_right,
    _rectangle_bottom_right,
    NUMBER_OF_POINTS_PER_RECTANGLE
};

/* macros */

#define FLOOR(n,floor) ((n)<(floor)?(floor):(n))
#define CEILING(n,ceiling) ((n)>(ceiling)?(ceiling):(n))
#define PIN(n,floor,ceiling) ((n)<(floor) ? (floor) : CEILING(n,ceiling))

#define FLAG(b) (1<<(b))

#define TEST_FLAG(f, b) (((f)&FLAG(b))!=0)
#define SWAP_FLAG(f, b) ((f)^=FLAG(b))
#define SET_FLAG(f, b, v) ((v) ? ((f)|=FLAG(b)) : ((f)&=~FLAG(b)))

/* types */

typedef uint32_t tag;
typedef float real;

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

/* asserts */

#ifdef assert
#undef assert
#endif

#ifdef DEBUG_BUILD
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

#if DEBUG_BUILD
extern char temporary[256];
#endif

/* global colors */

extern const real_argb_color *global_real_argb_white;
extern const real_argb_color *global_real_argb_grey;
extern const real_argb_color *global_real_argb_black;
extern const real_argb_color *global_real_argb_red;
extern const real_argb_color *global_real_argb_green;
extern const real_argb_color *global_real_argb_blue;
extern const real_argb_color *global_real_argb_yellow;
extern const real_argb_color *global_real_argb_cyan;
extern const real_argb_color *global_real_argb_magenta;
extern const real_argb_color *global_real_argb_pink;
extern const real_argb_color *global_real_argb_lightblue;
extern const real_argb_color *global_real_argb_orange;
extern const real_argb_color *global_real_argb_purple;
extern const real_argb_color *global_real_argb_aqua;
extern const real_argb_color *global_real_argb_darkgreen;
extern const real_argb_color *global_real_argb_salmon;
extern const real_argb_color *global_real_argb_violet;

extern const real_rgb_color *global_real_rgb_white;
extern const real_rgb_color *global_real_rgb_grey;
extern const real_rgb_color *global_real_rgb_black;
extern const real_rgb_color *global_real_rgb_red;
extern const real_rgb_color *global_real_rgb_green;
extern const real_rgb_color *global_real_rgb_blue;
extern const real_rgb_color *global_real_rgb_yellow;
extern const real_rgb_color *global_real_rgb_cyan;
extern const real_rgb_color *global_real_rgb_magenta;
extern const real_rgb_color *global_real_rgb_pink;
extern const real_rgb_color *global_real_rgb_lightblue;
extern const real_rgb_color *global_real_rgb_orange;
extern const real_rgb_color *global_real_rgb_purple;
extern const real_rgb_color *global_real_rgb_aqua;
extern const real_rgb_color *global_real_rgb_darkgreen;
extern const real_rgb_color *global_real_rgb_salmon;
extern const real_rgb_color *global_real_rgb_violet;

/* functions */

tag string_to_tag(const char *s);
char *tag_to_string(tag t, char *s);

void system_exit(int code);

/* debug memory */

#ifdef DEBUG_BUILD

void *debug_malloc(size_t size, bool clear, const char *source_file, int32_t source_line);
void debug_free(void *pointer, const char *source_file, int32_t source_line);
void *debug_realloc(void *pointer, size_t size, const char *source_file, int32_t source_line);

#undef malloc
#undef calloc
#undef free
#undef realloc

#define calloc(num, size) (debug_malloc((num) * (size), true, __FILE__, __LINE__))
#define malloc(size) (debug_malloc((size), false, __FILE__, __LINE__))
#define free(pointer) (debug_free((pointer), __FILE__, __LINE__))
#define realloc(pointer, size) (debug_realloc(pointer, size, __FILE__, __LINE__))

#endif

uint32_t system_milliseconds(void);

#endif
