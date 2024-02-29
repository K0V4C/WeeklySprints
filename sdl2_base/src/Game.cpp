#include "../header/Game.hpp"
#include <SDL2/SDL.h>

#include "../header/Objects.hpp"

bool Game::_running = true;
Game::Game() {
    this->_running = true;
}


auto Game::game_loop() -> void {
    
    while(_running) {

        Game::handle_events();

        Game::simulate();

        Game::render();

    }
}

// Handle sdl events
auto Game::handle_events() -> void {
    SDL_Event e;
    SDL_PollEvent(&e);
    G_Objects::game_window->handle_window_event(e);
}

// game simulation
auto Game::simulate() -> void {

}

// rendering
auto Game::render() -> void {
        
        // Test code to check if sdl2 works
        SDL_SetRenderDrawColor(G_Objects::game_window->get_renderer(), 255, 255, 0, 0);
        SDL_RenderClear(G_Objects::game_window->get_renderer());
        SDL_RenderPresent(G_Objects::game_window->get_renderer());

}
