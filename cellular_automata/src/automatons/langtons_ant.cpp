#include "../../header/automatons/langtons_ant.hpp"
#include "../../header/objects.hpp"

Langton_Ant::Langton_Ant() : I_Automaton() {
    
    x_pos = G_Objects::game->get_horizontal_squares() / 2; 
    y_pos = G_Objects::game->get_vertical_squares() / 2;
    
    // 0 1 2 3 -> N E S W
    direction = 0;
}

auto Langton_Ant::calculate_new_grid(Grid<Cell>& input_grid, Grid<Cell>& output_grid) -> void {

    static bool start_flag = true;
    if(start_flag) {
        for(auto& cell : input_grid) {
            *cell = Cell_Type::Langton_Black;
        }
        
        x_pos = G_Objects::game->get_horizontal_squares() / 2; 
        y_pos = G_Objects::game->get_vertical_squares() / 2;
        
        // 0 1 2 3 -> N E S W
        direction = 0; 
    }
    start_flag = false;

    if(input_grid[{x_pos, y_pos}] == Cell_Type::Langton_White) {
        output_grid[{x_pos, y_pos}] = Cell_Type::Langton_Black;
        direction = (direction + 1) % 4;
        x_pos += directions[direction].first;
        y_pos += directions[direction].second;
    } else {
        output_grid[{x_pos, y_pos}] = Cell_Type::Langton_White;
        
        if(direction == 0) 
            direction = 3;
        else 
            direction -= 1;

        x_pos += directions[direction].first;
        y_pos += directions[direction].second;
    }
}

auto Langton_Ant::handle_event(SDL_Event& e) -> void {}

Langton_Ant::~Langton_Ant() {}
