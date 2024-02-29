#ifndef GAME_CLASS
#define GAME_CLASS

#include <SDL2/SDL_events.h>
class Game {
public:

    Game();
    
    // Maybe change this
    Game(Game&) = delete;
    Game(Game&&) = delete;
    auto operator=(Game&) = delete;
    auto operator=(Game&&) = delete;

    static auto game_loop() -> void;

    static auto handle_events() -> void;
    static auto simulate() -> void;
    static auto render() -> void;
    
    // Global used to turn of the game
    static bool _running;

private:
};

#endif // !GAME_CLASS
