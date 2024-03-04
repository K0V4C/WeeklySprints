#include "../header/objects.hpp"
#include <SDL2/SDL_mutex.h>
#include <memory>

std::unique_ptr<U_Window> G_Objects::game_window = nullptr;
std::shared_ptr<Game> G_Objects::game = nullptr;
std::shared_ptr<Canvas> G_Objects::g_canvas = nullptr;

std::shared_ptr<std::mutex> G_Objects::output_cell_mutex = nullptr;

// Cells are created inside Game class
Grid<Cell> G_Objects::input_cells =  Grid<Cell>();
Grid<Cell> G_Objects::output_cells =  Grid<Cell>();

auto G_Objects::init_game(int32_t resolution, std::string mode) -> void {
    
    if(!G_Objects::game_window)
        G_Objects::game_window = std::make_unique<U_Window>();
    G_Objects::game_window->init();

    if(!G_Objects::game)
        G_Objects::game.reset(new Game()); 
    G_Objects::game->set_automaton(mode);
    G_Objects::game->set_resolution(resolution);

    
    if(!G_Objects::g_canvas)
        G_Objects::g_canvas.reset(new Canvas(G_Objects::game_window->get_renderer()));

    if(!G_Objects::output_cell_mutex)
        G_Objects::output_cell_mutex.reset(new std::mutex());
}
