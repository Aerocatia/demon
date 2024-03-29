#ifndef DEMON__IMPL_TYPES_TYPES_H
#define DEMON__IMPL_TYPES_TYPES_H

#include "../memory/table.h"

typedef uint32_t ColorARGBInt;

typedef union ScenarioScriptNodeValue {
    float f;
    TableID id;
    int8_t b;
    int16_t s;
    int32_t l;
    const char *string;
} ScenarioScriptNodeValue;
_Static_assert(sizeof(ScenarioScriptNodeValue) == 0x4);

typedef struct ColorARGB {
    float a;
    float r;
    float g;
    float b;
} ColorARGB;
_Static_assert(sizeof(ColorARGB) == 0x10);

typedef struct ColorRGB {
    float r;
    float g;
    float b;
} ColorRGB;
_Static_assert(sizeof(ColorRGB) == 0xC);

typedef struct String32 {
    char string[32];
} String32;
_Static_assert(sizeof(String32) == 32);

typedef struct Data {
    uint32_t size;
    uint32_t flags;
    uint32_t file_offset; // only applies to sounds
    void *pointer;
    char padding[4];
} Data;
_Static_assert(sizeof(Data) == 0x14);

typedef struct VectorXYZ {
    float x;
    float y;
    float z;
} VectorXYZ;
_Static_assert(sizeof(VectorXYZ) == 0xC);

typedef struct VectorXY {
    float x;
    float y;
} VectorXY;
_Static_assert(sizeof(VectorXY) == 0x8);

typedef struct VectorXYInt {
    int16_t x;
    int16_t y;
} VectorXYInt;
_Static_assert(sizeof(VectorXYInt) == 0x4);

typedef struct VectorIJK {
    float i;
    float j;
    float k;
} VectorIJK;
_Static_assert(sizeof(VectorIJK) == 0xC);

typedef struct Quaternion {
    float i;
    float j;
    float k;
    float l;
} Quaternion;
_Static_assert(sizeof(Quaternion) == 0x10);

typedef struct VectorPYR {
    float pitch;
    float yaw;
    float rotation;
} VectorPYR;
_Static_assert(sizeof(VectorPYR) == 0xC);

typedef struct VectorPY {
    float pitch;
    float yaw;
} VectorPY;
_Static_assert(sizeof(VectorPY) == 0x8);

typedef struct Rectangle2D {
    uint16_t top;
    uint16_t left;
    uint16_t bottom;
    uint16_t right;
} Rectangle2D;
_Static_assert(sizeof(Rectangle2D) == 0x8);

typedef struct Plane2D {
    float i;
    float j;
    float w;
} Plane2D;
_Static_assert(sizeof(Plane2D) == 0xC);

typedef struct Plane3D {
    float i;
    float j;
    float k;
    float w;
} Plane3D;
_Static_assert(sizeof(Plane3D) == 0x10);

typedef struct GenericReflexive {
    uint32_t count;
    void *pointer;
    uint8_t padding[4];
} GenericReflexive;
_Static_assert(sizeof(GenericReflexive) == 0xC);

/**
 * Decode an integer (X8)R8G8B8 value to floating point RGB.
 */
void decode_r8g8b8(ColorARGBInt rgb, ColorRGB *output);

/**
 * Decode an integer A8R8G8B8 value to floating point ARGB.
 */
void decode_a8r8g8b8(ColorARGBInt argb, ColorARGB *output);

#endif
