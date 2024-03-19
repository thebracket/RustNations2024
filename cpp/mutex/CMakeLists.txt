cmake_minimum_required(VERSION 3.5)
project(TypeChanges)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

#set(CMAKE_EXE_LINKER_FLAGS "-static-libgcc -static-libstdc++ -static")
add_executable(mutex mutex.cpp)