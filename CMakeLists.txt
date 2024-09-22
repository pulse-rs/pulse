cmake_minimum_required(VERSION 3.29)
project(pulse)

set(CMAKE_CXX_STANDARD 14)

add_executable(pulse
    cli/lib/env.cpp
    cli/lib/io.cpp
    cli/lib/lib.cpp
    cli/lib/math.cpp
    playground/build/source/main.cpp
    playground/build/std/env.cpp
    playground/build/std/io.cpp
    playground/build/std/lib.cpp
    playground/build/std/math.cpp
        cli/lib/error.cpp)
