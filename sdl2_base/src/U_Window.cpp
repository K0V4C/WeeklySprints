#include <SDL2/SDL.h>
#include <cstdint>
#include <iostream>

#include "../header/U_Window.hpp"
#include "../header/Game.hpp"

const int32_t DEFAULT_WIDTH = 1600;
const int32_t DEFAULT_HEIGHT = 800;

U_Window::U_Window() {

    this->renderer = nullptr;
    this->window = nullptr;

    this->width = 0;
    this->height = 0;

    this->shown = false;
    this->mouse_focus = false;
    this->keyboard_focus = false;
    this->minimized = false;
}

auto U_Window::init() -> bool {
    this->width = DEFAULT_WIDTH;
    this->height = DEFAULT_HEIGHT;
   
    this->window = SDL_CreateWindow( "cellular automata", SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED,
            this->width, this->height, 0);

    if(!this->window) {
        std::cout << "error creating window!\n";
        return false;
    }

    this->renderer = SDL_CreateRenderer(this->window, -1, 0);

    if(!this->renderer) {
        std::cout << "error creating rendered\n";
        return false;
    }

    this->mouse_focus = true;
    this->keyboard_focus = true;

    return true;
}

auto U_Window::handle_window_event(SDL_Event& event) -> void {
    
    switch (event.type) {
        case SDL_QUIT:     
        case SDL_WINDOWEVENT_CLOSE:
            switch (event.window.event) {
                case SDL_WINDOWEVENT_CLOSE:
                    Game::_running = false;
                    break;
                default:
                    break;
            }
            break;

        default:
            break;
    }

}

auto U_Window::get_width() const -> int32_t {return this->width;}

auto U_Window::get_height() const -> int32_t {return this->height;}

auto U_Window::get_renderer() const -> SDL_Renderer* {return this->renderer;}

U_Window::~U_Window() {
     //Destroy window    
    SDL_DestroyRenderer( renderer );
    SDL_DestroyWindow( window );

}
