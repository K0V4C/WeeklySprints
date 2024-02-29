#ifndef GLOBAL_OBJECTS
#define GLOBAL_OBJECTS

#include "U_Window.hpp"
#include "Game.hpp"
#include <memory>

struct G_Objects {
    static std::unique_ptr<U_Window> game_window;
    static std::shared_ptr<Game> game;

    static auto init_game() -> void; 
};



#endif // !DEBUG
