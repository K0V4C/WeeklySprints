#include <iostream>

#include "../header/utility.hpp"
#include "../header/objects.hpp"
#include "../header/types.hpp"

int main (int argc, char *argv[]) {
    
    io::input_msg(argc, argv);

    // Read input
    auto resolution = std::atoi(argv[1]);
    auto mode = std::string(argv[2]);

    // Init sdl2 and prepare windows, renderers and other objects 
    G_Objects::init_game(resolution, mode);

    // Seperate game control to thiis thread
    std::thread control(control::run);
     
    // Main thread handles the rendering
    while(G_Objects::game->state != Game::State::Finished)
        Game::render();

    control.join();

    return 0;
}
