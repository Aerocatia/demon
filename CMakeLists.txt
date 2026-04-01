cmake_minimum_required(VERSION 4.0)

project(Demon
    VERSION 1.0.0
    DESCRIPTION "Here we go again"
    LANGUAGES C
)

add_library(demon SHARED
    source/cache/cache_files.c
    source/cseries/cseries.c
    source/cseries/errors.c
    source/main/console.c
    source/main/demon.c
    source/main/main.c
    source/math/matrix_math.c
    source/math/real_math.c
    source/math/real_math_debug_inlines.c
    source/memory/crc.c
    source/text/unicode.c
)

target_compile_definitions(demon PUBLIC
    DEBUG
    NO_DEBUG_TRAP
    TARGET="halo"
    REQUIRE_CACHE_FILE
)

option(DEMON_COUNT_THUNKS "Count thunks on DLL load" OFF)
if(${DEMON_COUNT_THUNKS})
    target_compile_definitions(demon PUBLIC DEMON_COUNT_THUNKS)
endif()

option(DEMON_MATH_ASSERTS "Enable math assertion checks" OFF)
if(${DEMON_MATH_ASSERTS})
    target_compile_definitions(demon PUBLIC MATH_ASSERTS)
endif()

option(DEMON_NO_DEBUG_TRAP "Immediately exit on an assertion error rather than emitting the debug trap instruction" ON)
if(${DEMON_NO_DEBUG_TRAP})
    target_compile_definitions(demon PUBLIC NO_DEBUG_TRAP)
endif()

set_target_properties(demon PROPERTIES PREFIX "")
target_compile_options(demon PRIVATE -Wall -Wextra -mfpmath=sse)
