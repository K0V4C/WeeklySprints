#ifndef CONWAYS_GAME_OF_LIFE
#define CONWAYS_GAME_OF_LIFE

#include "i_automaton.hpp"
#include <SDL2/SDL_events.h>

class Conway_GOL : public I_Automaton {
public:
   Conway_GOL();

    virtual auto calculate_new_grid(Grid<Cell>&, Grid<Cell>&) -> void override;

    virtual auto handle_event(SDL_Event& e) -> void override;
    
private: 
    
};


#endif 
