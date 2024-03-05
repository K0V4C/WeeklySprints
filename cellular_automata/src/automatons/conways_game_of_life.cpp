#include "../../header/automatons/conways_game_of_life.hpp"
#include "../../header/objects.hpp"

#include <SDL2/SDL_mouse.h>
#include <cstdint>
#include <mutex>

Conway_GOL::Conway_GOL() {
    this->pen = Cell_Type::Alive;
    this->left_mouse_pressed = false;
}

auto Conway_GOL::change_cell_at_index(int32_t idx) -> void {
    std::scoped_lock<std::mutex> lock(*G_Objects::output_cell_mutex);
    G_Objects::output_cells[idx] = pen;
}

auto Conway_GOL::handle_event(SDL_Event& e) -> void {
    
    int32_t x_mouse_pos, y_mouse_pos;

    SDL_GetMouseState(&x_mouse_pos,&y_mouse_pos);
     
    int32_t matrix_index = (x_mouse_pos / G_Objects::game->get_square_side()) +
        (y_mouse_pos / G_Objects::game->get_square_side()) * G_Objects::game->get_horizontal_squares();

    if(e.type == SDL_KEYDOWN) {
        if(e.key.keysym.sym == SDLK_1) {
            this->pen = Cell_Type::Dead; 
        }
    
        if(e.key.keysym.sym == SDLK_2) {
            this->pen = Cell_Type::Alive; 
        }
    }
    

    if(left_mouse_pressed) {
        change_cell_at_index(matrix_index);
    }

    if(SDL_MOUSEBUTTONDOWN == e.type ) {
        if(SDL_BUTTON_LEFT == e.button.button) {
            change_cell_at_index(matrix_index);
            left_mouse_pressed = true;
        }
    }
    
    if(SDL_MOUSEBUTTONUP == e.type) {
        if(SDL_BUTTON_LEFT == e.button.button) {
            left_mouse_pressed = false;
        }
    }

}

const std::pair<int32_t,int32_t> directions3x3[] = {
            {-1, -1},  {-1, 0}, {-1, 1},
            { 0, -1},           { 0, 1},
            { 1, -1},  { 1, 0}, { 1, 1}
};

auto Conway_GOL::calculate_new_grid(Grid<Cell>& input_grid, Grid<Cell>& output_grid) -> void {
    
    static bool prepare_grid = false;
    if(!prepare_grid) {
        for(auto i = 0; i < input_grid.size(); i++) {
            input_grid[i] = Cell_Type::Dead;
        }
    }
    prepare_grid = true;

    
    for(int32_t i = 0; i < input_grid.heigth(); i++) {
        for(int32_t j = 0; j < input_grid.width(); j++) {

            int32_t num_of_conway_neighbours = 0;

            auto& current_cell = input_grid[{i,j}];

            for(auto &dir : directions3x3 ){
                
                int32_t x = i + dir.first;
                int32_t y = j + dir.second;
                
                if(x >= 0 and x < input_grid.width() and y >= 0 and y < input_grid.heigth())
                    if(input_grid[{x,y}] == Cell_Type::Alive) num_of_conway_neighbours++;
            }

            if(current_cell == Cell_Type::Alive) {

                if(num_of_conway_neighbours < 2) {
                    // Die

                    output_grid[{i,j}] = Cell_Type::None;
                } 

                if(num_of_conway_neighbours == 2 or num_of_conway_neighbours == 3) {
                    // Stay alive

                    output_grid[{i,j}] = Cell_Type::Alive;
                }

                if(num_of_conway_neighbours > 3) {
                    // Die

                    output_grid[{i,j}] = Cell_Type::Dead;
                }

                
            } else {
                if(num_of_conway_neighbours == 3) {
                    // Revive

                    output_grid[{i,j}] = Cell_Type::Alive;
                }
            }
        }
    }
}

Conway_GOL::~Conway_GOL() {


}
