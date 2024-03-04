#ifndef CANVAS
#define CANVAS

// Class used to draw to the screen, couse automatons are 1d od 2d grid like only.
// Class for drawing them is called Canvas 

#include <SDL2/SDL_render.h>
#include <cstdint>

class Canvas {
public:
    
    Canvas(SDL_Renderer*);
    
    // Maybe need to change this
    Canvas(const Canvas&) = delete;
    Canvas(Canvas&&) = delete;
    auto operator=(const Canvas&) = delete;
    auto operator=(Canvas&&) = delete; 
    
    auto draw_background() -> void;
    auto draw_grid_lines() -> void;
    auto draw_cells() -> void;
    auto present() -> void;
 
    ~Canvas();

private:
    
    SDL_Renderer* renderer;

    int32_t width = 0;
    int32_t heigth = 0;

    int32_t resolution = 0;
    int32_t square_side = 0;

    int32_t horiontal_squares = 0; // this is number of square width wise 
    int32_t vertical_squares = 0;  // this is number of squares vertical
};

#endif
