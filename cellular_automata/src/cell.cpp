#include <SDL2/SDL_rect.h>
#include <cstdint>

#include "../header/cell.hpp"

Cell::Cell() {
    type = Cell_Type::None;
    rect = nullptr;
}

Cell::Cell(int32_t x, int32_t y, int32_t s, Cell_Type type) {
    
    this->type = type;
    this->color = cell_colors[(int32_t)type];
    rect = new SDL_Rect();

    rect->x = x;
    rect->y = y;
    rect->h = rect->w = s;
}

Cell::Cell(const Cell& obj) {
    this->type = obj.type;
    this->color = obj.color;

    this->rect = new SDL_Rect();

    this->rect->h = obj.rect->h;
    this->rect->w = obj.rect->w;
    this->rect->x = obj.rect->x;
    this->rect->y = obj.rect->y;
}

Cell::Cell(Cell&& obj) {
    this->type = obj.type;
    this->color = obj.color;

    this->rect = obj.rect;

    obj.rect = nullptr;
}

auto Cell::operator=(const Cell& obj) -> Cell& {
    if(this != &obj) {
        delete this->rect;

        this->type = obj.type;
        this->color = obj.color;

        this->rect = new SDL_Rect();

        this->rect->h = obj.rect->h;
        this->rect->w = obj.rect->w;
        this->rect->x = obj.rect->x;
        this->rect->y = obj.rect->y;
    }
    return *this;
}

auto Cell::operator=(Cell&& obj) -> Cell& {
    if(this != &obj) {
        delete this->rect;

        this->type = obj.type;
        this->color = obj.color;

        this->rect = obj.rect;

        obj.rect = nullptr;
    }
    return *this;
}

auto Cell::operator=(Cell_Type type) -> Cell& {
    this->type = type;
    color = cell_colors[(int32_t)type];
    return *this;
}


Cell::~Cell() {}
