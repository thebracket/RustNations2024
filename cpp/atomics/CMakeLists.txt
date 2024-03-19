cmake_minimum_required(VERSION 3.5)
project(TypeChanges)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

if (MSVC)
    add_compile_options(/W4 /WX)
else()
    add_compile_options(-Wall -Wextra -pedantic -Werror)
endif()

#set(CMAKE_EXE_LINKER_FLAGS "-static-libgcc -static-libstdc++ -static")
add_executable(atomics atomics.cpp)