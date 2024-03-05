#include "../../header/automatons/wireworld.hpp"
#include "../../header/objects.hpp"

WireWorld::WireWorld() : I_Automaton() {
    this->pen = Cell_Type::None;
    left_mouse_pressed = false;
}

auto WireWorld::calculate_new_grid(Grid<Cell>& input_grid, Grid<Cell>& output_grid) -> void {
    
    static bool start_flag = true;
    if(start_flag) {
        for(auto& cell : input_grid) {
            *cell = Cell_Type::None;
        }
    }
    start_flag = false;

    for(int32_t i = 0; i < input_grid.heigth(); i++) {
        for(int32_t j = 0; j < input_grid.width(); j++) {
        
            auto cell = input_grid[{i,j}];

            // If it is None it stays None
            
            if (cell == Cell_Type::Electron_Head) {
                output_grid[{i,j}] = Cell_Type::Electron_Tail;
            }
            else if (cell == Cell_Type::Electron_Tail) {
                output_grid[{i,j}] = Cell_Type::Conductor;
            }
            else if (cell == Cell_Type::Conductor) {
                auto heads = 0; 
                for(auto &dir : directions3x3 ){
                    
                    int32_t x = i + dir.first;
                    int32_t y = j + dir.second;
                    
                    if(x >= 0 and x < input_grid.width() and y >= 0 and y < input_grid.heigth())
                        if(input_grid[{x,y}] == Cell_Type::Electron_Head) heads += 1;
                }

                if(heads == 1 or heads == 2) {
                    output_grid[{i,j}] = Cell_Type::Electron_Head;
                }
            }
        }
    }
}

auto WireWorld::change_cell_at_index(int32_t idx) -> void {
    std::scoped_lock<std::mutex> lock(*G_Objects::output_cell_mutex);
    G_Objects::output_cells[idx] = this->pen; 
}

auto WireWorld::handle_event(SDL_Event& e) -> void {
    int32_t x_mouse_pos, y_mouse_pos;

    SDL_GetMouseState(&x_mouse_pos,&y_mouse_pos);
     
    int32_t matrix_index = (x_mouse_pos / G_Objects::game->get_square_side()) +
        (y_mouse_pos / G_Objects::game->get_square_side()) * G_Objects::game->get_horizontal_squares();
    
    if(left_mouse_pressed) {
        change_cell_at_index(matrix_index);
    }

    if(e.type == SDL_KEYDOWN) {
        if(e.key.keysym.sym == SDLK_1) {
            this->pen = Cell_Type::None; 
        }
    
        if(e.key.keysym.sym == SDLK_2) {
            this->pen = Cell_Type::Conductor; 
        }

        if(e.key.keysym.sym == SDLK_3) {
            this->pen = Cell_Type::Electron_Head; 
        }

        if(e.key.keysym.sym == SDLK_4) {
            this->pen = Cell_Type::Electron_Tail; 
        }
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

WireWorld::~WireWorld() {}
