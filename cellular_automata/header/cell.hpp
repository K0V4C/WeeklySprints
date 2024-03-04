#ifndef INTERFACE_CELL
#define INTERFACE_CELL

#include "types.hpp"

#include <SDL2/SDL_rect.h>
#include <cstdint>
#include <iostream>

class Cell {
public:
    Cell_Type type;
    RGBA color;
    SDL_Rect* rect;

    Cell();
    Cell(int32_t x, int32_t y,int32_t s, Cell_Type type);
    
    Cell(const Cell&);
    Cell(Cell&&);
    auto operator=(const Cell&) -> Cell&;
    auto operator=(Cell&&) -> Cell&;

    friend std::ostream& operator<< (std::ostream& os, const Cell& obj) {
        os << (int32_t) obj.type;
        return os;
    }

    friend auto operator==(const Cell& left, const Cell& right) -> bool {
        return left.type == right.type;
    }

    friend auto operator==(const Cell& left, const Cell_Type right) -> bool {
        return left.type == right;
    }

    auto operator=(Cell_Type type) -> Cell&; 
    
    // this is an interface 
    virtual ~Cell();

    
};

#endif
