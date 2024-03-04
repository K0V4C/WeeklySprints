#include "../../header/automatons/elementary_automaton.hpp"
#include "../../header/automatons/i_automaton.hpp"
#include <cstdint>

Elementary_Automaton::Elementary_Automaton(int32_t rule) : I_Automaton(), rule(rule) {
    this->read_row = 0;
    this->write_row = 1;
    
    auto temp = rule;

    for(int32_t i = 0; i < 8; i++) {

        auto b_num = temp & 1;
        temp >>= 1;

        if(b_num != 0) {
            rule_set[i] = Cell_Type::Elementary_Cell;
        } else {
            rule_set[i] = Cell_Type::None;
        }
    }
}

auto Elementary_Automaton::find_pattern_rule(Cell_Vec3* vec) -> Cell_Type {

    if(vec->first == Cell_Type::None and vec->second == Cell_Type::None and vec->third == Cell_Type::None)
        return rule_set[0];

    if(vec->first == Cell_Type::None and vec->second == Cell_Type::None and vec->third == Cell_Type::Elementary_Cell)
        return rule_set[1];
    
    if(vec->first == Cell_Type::None and vec->second == Cell_Type::Elementary_Cell and vec->third == Cell_Type::None)
        return rule_set[2];
    
    if(vec->first == Cell_Type::None and vec->second == Cell_Type::Elementary_Cell and vec->third == Cell_Type::Elementary_Cell)
        return rule_set[3];

    if(vec->first == Cell_Type::Elementary_Cell and vec->second == Cell_Type::None and vec->third == Cell_Type::None)
        return rule_set[4];

    if(vec->first == Cell_Type::Elementary_Cell and vec->second == Cell_Type::None and vec->third == Cell_Type::Elementary_Cell)
        return rule_set[5];

    if(vec->first == Cell_Type::Elementary_Cell and vec->second == Cell_Type::Elementary_Cell and vec->third == Cell_Type::None)
        return rule_set[6];

    if(vec->first == Cell_Type::Elementary_Cell and vec->second == Cell_Type::Elementary_Cell and vec->third == Cell_Type::Elementary_Cell)
        return rule_set[7];

    return Cell_Type::None;
}

auto Elementary_Automaton::calculate_new_grid(Grid<Cell>& input_grid, Grid<Cell>& output_grid) -> void {
    

    static bool seed_set = false;
    if(!seed_set) {
        output_grid[input_grid.width()/2] = Cell_Type::Elementary_Cell;
        input_grid[input_grid.width()/2] = Cell_Type::Elementary_Cell;
        seed_set = true;
    }
    


    //  window of 3
    for(int32_t i = 0; i < input_grid.width(); i++) {

        auto index = [&](int32_t x) {
            return read_row*input_grid.width() + (i + x + input_grid.width())%input_grid.width();
        };
        
        Cell_Vec3 vec = {
            input_grid[index(-1)].type,
            input_grid[index(0)].type,
            input_grid[index(1)].type
        };


        Cell_Type new_cell = find_pattern_rule(&vec);
        
        output_grid[write_row*output_grid.width() + i%input_grid.width()]= new_cell;
    }
    
    read_row = write_row;
    write_row = (write_row + 1) % input_grid.heigth();

}


