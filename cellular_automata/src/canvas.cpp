#include "../header/canvas.hpp"
#include "../header/objects.hpp"

#include <SDL2/SDL_mutex.h>
#include <SDL2/SDL_render.h>
#include <cstdint>
#include <mutex>


Canvas::Canvas(SDL_Renderer* renderer) : renderer(renderer){

    this->width = G_Objects::game_window->get_width();
    this->heigth = G_Objects::game_window->get_height();
    this->resolution = G_Objects::game->get_resolution();
    this->square_side = G_Objects::game->get_square_side();
    this->horiontal_squares = G_Objects::game->get_horizontal_squares();
    this->vertical_squares = G_Objects::game->get_vertical_squares();
}

auto Canvas::draw_background() -> void {

    RGBA colors = color_palete[0]; // White
    SDL_SetRenderDrawColor(renderer, colors.red, colors.green, colors.blue, colors.alpha);
    SDL_RenderClear(renderer);

}


auto Canvas::draw_cells() -> void {

    // Lock over cells 
    std::scoped_lock<std::mutex> lock(*G_Objects::output_cell_mutex);
    for(auto& cell : G_Objects::output_cells) {
        
        RGBA colors = cell->color;

        SDL_SetRenderDrawColor(renderer,
                            colors.red,
                            colors.green,
                            colors.blue,
                            colors.alpha);

        SDL_RenderFillRect(renderer, cell->rect);
    }

}

auto Canvas::draw_grid_lines() -> void {

    RGBA colors = color_palete[1]; // Black 
    SDL_SetRenderDrawColor(renderer, colors.red, colors.green, colors.blue, colors.alpha);
    
    // Draw horizontal lines
    for(int32_t i = 0; i < vertical_squares; i++)
       SDL_RenderDrawLine(renderer, 0, i * square_side, width, i * square_side);
    
    // Draw vertical lines
    for(int32_t i = 0; i < horiontal_squares; i++)
        SDL_RenderDrawLine(renderer, i * square_side, 0, i * square_side, heigth);

}

auto Canvas::present() -> void {
    SDL_RenderPresent(renderer);
}

Canvas::~Canvas() {}
