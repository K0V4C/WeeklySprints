#include <SDL2/SDL.h>
#include <SDL2/SDL_events.h>
#include <SDL2/SDL_timer.h>

#include "../header/U_Window.hpp"
#include "../header/Objects.hpp"

int main (int argc, char *argv[]) {
    
    // Init sdl2 and prepare windows, renderers and other objects 
    G_Objects::init_game();

    // Main loop and where eveything should be written to
    Game::game_loop();

    return 0;
}
