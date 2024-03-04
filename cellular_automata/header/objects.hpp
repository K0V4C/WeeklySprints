#ifndef GLOBAL_OBJECTS
#define GLOBAL_OBJECTS

#include <SDL2/SDL_mutex.h>
#include <memory>
#include <mutex>
#include <thread>

#include "grid.hpp"
#include "canvas.hpp"
#include "u_window.hpp"
#include "game.hpp"
#include "cell.hpp"

struct G_Objects {
    static std::unique_ptr<U_Window> game_window;
    static std::shared_ptr<Game> game;
    static std::shared_ptr<Canvas> g_canvas;

    static Grid<Cell> input_cells;
    static Grid<Cell> output_cells;

    static std::shared_ptr<std::mutex> output_cell_mutex;

    static auto init_game(int32_t, std::string) -> void; 
};



#endif // !DEBUG
