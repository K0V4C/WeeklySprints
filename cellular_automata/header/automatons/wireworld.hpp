#ifndef CUSTOM_AUTOMATON
#define CUSTOM_AUTOMATON

#include "i_automaton.hpp"
class WireWorld : public I_Automaton {
public:
    
    WireWorld();

    virtual auto calculate_new_grid(Grid<Cell>& input_grid, Grid<Cell>& output_grid) -> void override;

    virtual auto handle_event(SDL_Event& e) -> void override;
        
    auto change_cell_at_index(int32_t) -> void;

    ~WireWorld();

private:

    const std::pair<int32_t,int32_t> directions3x3[8] = {
        {-1, -1},  {-1, 0}, {-1, 1},
        { 0, -1},           { 0, 1},
        { 1, -1},  { 1, 0}, { 1, 1}
    }; 
    
    Cell_Type pen;

    bool left_mouse_pressed;
};

#endif
