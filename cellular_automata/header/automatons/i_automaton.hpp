#ifndef AUTOMATON_WIZARD
#define AUTOMATON_WIZARD

#include <SDL2/SDL.h>

#include "../grid.hpp"
#include "../cell.hpp"

class I_Automaton {
public:

    I_Automaton() {}

    virtual auto calculate_new_grid(Grid<Cell>& input_grid, Grid<Cell>& output_grid) -> void = 0;

    virtual auto handle_event(SDL_Event& e) -> void = 0;
    
    ~I_Automaton() {}
};

#endif
