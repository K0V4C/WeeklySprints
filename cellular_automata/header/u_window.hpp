#ifndef UTILITY_WINDOW
#define UTILITY_WINDOW

#include <cstdint>
#include <SDL2/SDL.h>

class U_Window {
public:

    U_Window();
    
    // TODO: maybe change this
    U_Window(const U_Window&) = delete;
    U_Window(U_Window&&) = delete;
    auto operator=(const U_Window&) -> U_Window& = delete;
    auto operator=(U_Window&&) -> U_Window&& = delete;

    // Getters
    auto get_height() const -> int32_t;
    auto get_width() const -> int32_t;
    auto get_renderer() const -> SDL_Renderer*;
    
    // init
    auto init() -> bool;

    // Handle window events
    auto handle_window_event(SDL_Event&) -> void;

    ~U_Window();
private:
    
    // SDL2 stuff
    SDL_Window* window;
    SDL_Renderer* renderer;

    // Window id
    int32_t window_id;
    
    // Dimensions
    int32_t height;
    int32_t width;

    // Focus
    bool mouse_focus;
    bool keyboard_focus;
    bool fullscreen;
    bool minimized;
    bool shown;


};

#endif // !UTILITY_WINDOW
#define UTILITY_WINDOW
