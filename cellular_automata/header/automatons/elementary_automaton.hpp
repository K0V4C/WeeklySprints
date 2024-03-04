#ifndef ELEMENTARY_AUTOMATON
#define ELEMENTARY_AUTOMATON

#include "i_automaton.hpp"
#include <SDL2/SDL_events.h>
#include <cstdint>

class Elementary_Automaton : public I_Automaton {
public:
    
    Elementary_Automaton(int32_t);

    virtual auto calculate_new_grid(Grid<Cell>&, Grid<Cell>&) -> void override; 
    virtual auto handle_event(SDL_Event&) -> void override {}

    struct Cell_Vec3 {
        Cell_Type first;
        Cell_Type second;
        Cell_Type third;
    };

    auto find_pattern_rule(Cell_Vec3*) -> Cell_Type;

private: 
    
    int32_t rule;
    Cell_Type rule_set[8];

    int32_t read_row;
    int32_t write_row;

    
};

#endif
