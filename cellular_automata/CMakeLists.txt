
cmake_minimum_required(VERSION 3.28.3)

project(cellular_automata_engine)

# set standard for c++
set(CMAKE_CXX_STANDARD 20)

# ensure sdl2 is installed and include it
# this could be changed to build sdl2 
find_package(SDL2 REQUIRED)
include_directories(${SDL2_INCLUDE_DIRS})

# Adding source files
file(GLOB_RECURSE SOURCE_FILES CONFIGURE_DEPENDS 
    "${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/src/automaton/*.cpp")
add_executable(cellular_automata_engine "${SOURCE_FILES}")

# include other  header files
target_include_directories(cellular_automata_engine PUBLIC 
    "${CMAKE_CURRENT_SOURCE_DIR}/header/"
    "${CMAKE_CURRENT_SOURCE_DIR}/header/automaton/")
target_link_libraries(cellular_automata_engine ${SDL2_LIBRARIES})
