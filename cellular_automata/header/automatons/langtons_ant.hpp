#ifndef LANGTON
#define LANGTON 

#include "i_automaton.hpp"
#include <cstdint>
class Langton_Ant : public I_Automaton {
public:

    Langton_Ant();

    virtual auto calculate_new_grid(Grid<Cell>& input_grid, Grid<Cell>& output_grid) -> void override;

    virtual auto handle_event(SDL_Event& e) -> void override;

    ~Langton_Ant();
    

private:
    
    const std::pair<int32_t, int32_t> directions[4] = {
        {0, -1},
        {1, 0},
        {0, 1},
        {-1, 0}
    };

    int32_t x_pos;
    int32_t y_pos;
    
    uint8_t direction;
};

#endif
