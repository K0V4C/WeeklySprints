#include "../header/Objects.hpp"
#include <memory>

std::unique_ptr<U_Window> G_Objects::game_window = nullptr;
std::shared_ptr<Game> G_Objects::game = nullptr;


auto G_Objects::init_game() -> void {
   
    if(!G_Objects::game_window)
        G_Objects::game_window = std::make_unique<U_Window>();
    G_Objects::game_window->init();

    if(!G_Objects::game)
        G_Objects::game.reset(new Game()); 
}
