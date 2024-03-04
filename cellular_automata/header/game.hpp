#ifndef GAME_CLASS
#define GAME_CLASS

#include "timer.hpp"
#include "automatons/i_automaton.hpp"

#include <string>
#include <SDL2/SDL_events.h>
#include <cstdint>

class Game {
public:

    enum class State {
        Running,
        Paused,
        Finished
    };

    Game();
    
    // Maybe change this
    Game(const Game&) = delete;
    Game(Game&&) = delete;
    auto operator=(const Game&) = delete;
    auto operator=(Game&&) = delete;

    auto game_loop() -> void;
    auto handle_events() -> void;
    auto simulate() -> void;
    static auto render() -> void;

    // Global used to control the game
    static Game::State state;
    
    // Set resolution
    auto set_resolution(int32_t) -> void;
    // setting automaton for simulation
    auto set_automaton(std::string) -> void;

    // getter
    auto get_number_of_cells() const -> int32_t;
    auto get_width() const -> int32_t;
    auto get_heigth() const -> int32_t;
    auto get_resolution() const -> int32_t;
    auto get_horizontal_squares() const -> int32_t;
    auto get_vertical_squares() const -> int32_t;
    auto get_square_side() const -> int32_t;

    ~Game();

private:
    int32_t number_of_cells;
    
    int32_t width = 0;
    int32_t heigth = 0;
    int32_t resolution = 0;

    int32_t horizotnal_squares = 0; // this is number of square width wise 
    int32_t vertical_squares = 0;  // this is number of squares vertical
    int32_t square_side = 0;
 
    Timer fps_timer;
    Timer cap_timer;

    // Strategy pattern like autumaton  
    I_Automaton *automaton;
};

#endif // !GAME_CLASS
