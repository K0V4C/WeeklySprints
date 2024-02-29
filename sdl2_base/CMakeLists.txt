cmake_minimum_required(VERSION 3.28.3)

project(sdl2_base)

# set standard for c++
set(CMAKE_CXX_STANDARD 20)

# uncomment lines marked with * to and comment ones with ! to hand compile sdl2

# ensure sdl2 is installed and include it
# this could be changed to build sdl2 
find_package(SDL2 REQUIRED) # !
include_directories(${SDL2_INCLUDE_DIRS}) # ! 

# add_subdirectory(lib/sdl2) # *

# Adding source files
file(GLOB_RECURSE SOURCE_FILES CONFIGURE_DEPENDS "${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp")
add_executable(sdl2_base "${SOURCE_FILES}")

# include other  header files
target_include_directories(sdl2_base PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/header/")
target_link_libraries(sdl2_base ${SDL2_LIBRARIES}) # !
# target_link_libraries(sdl2_base PRIVATE sdl2) # *


# To run this in termiinal use
# cmake ../CMakeLists.txt -B . && make && ./sdl2_base



