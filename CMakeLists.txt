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

set_target_properties(demon PROPERTIES PREFIX "")
target_compile_options(demon PRIVATE -Wall -Wextra)
