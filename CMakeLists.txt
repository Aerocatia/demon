cmake_minimum_required(VERSION 4.0)

project(Demon
    VERSION 1.0.0
    DESCRIPTION "Here we go again"
    LANGUAGES C
)

add_library(demon SHARED
    source/main/demon.c
    source/main/main.c
)

option(DEMON_COUNT_THUNKS "Count thunks on DLL load" OFF)
if(${DEMON_COUNT_THUNKS})
    add_definitions(-DDEMON_COUNT_THUNKS)
endif()

set_target_properties(demon PROPERTIES PREFIX "")
target_compile_options(demon PRIVATE -Wall -Wextra -Werror)
