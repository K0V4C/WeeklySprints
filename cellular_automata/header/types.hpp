#ifndef TYPES
#define TYPES

#include <cstdint>
#include <string>

// Define cell types
enum class Cell_Type {
        None,
        Dead,
        Alive,
        Elementary_Cell,
        Langton_White,
        Langton_Black,
        Conductor, 
        Electron_Head,
        Electron_Tail,
        Temp
};

enum class Color {
   White,
   Black,
};

// rgba struct
struct RGBA {
    
    uint8_t red;
    uint8_t green;
    uint8_t blue;
    uint8_t alpha;

    RGBA(uint8_t r, uint8_t g, uint8_t b, uint8_t a) :
        red(r), green(g), blue(b), alpha(a){}

    RGBA() : red(0), green(0), blue(0), alpha(0){}
};

// define coresponding colors for cell types
const RGBA cell_colors[] = {
    RGBA(0, 0, 0, 255), // None
    
    // Conway
    RGBA(0, 0, 0, 255), // Dead
    RGBA(255, 255, 255, 255), // Alive

    // 1D
    RGBA(255, 255, 255, 255), // Elementary_Cell

    // Langton Ant
    RGBA(255, 255, 255, 255), // Langton_White
    RGBA(0, 0, 0, 255), // Langton_Black

    // WireWorld
    RGBA(255,255,0,255), // Yellow
    RGBA(0,0,255,255), // Blue 
    RGBA(255,0,0,255), // Red 
    // None 

    // Temp
    RGBA(255, 255, 0, 255) // Temp
};

// color palete used for drawing
const RGBA color_palete[] = {
    {255,255,255,255}, // White
    {0,0,0,255}, // Black
};


const std::string simulations[] = {
    "Elementary Automata -> RuleZZZ",
    "Conways_Game",
    "Langtons_Ant",
    "WireWorld",
};

#endif 
