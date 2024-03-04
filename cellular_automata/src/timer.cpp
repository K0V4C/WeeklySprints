#include "../header/timer.hpp"
#include <SDL2/SDL_timer.h>
#include <cstdint>

//Initializes variables
Timer::Timer() :
    start_ticks(0), paused_ticks(0), started(false), paused(0){}

//The various clock actions
auto Timer::start() -> void {
    started = true;
    paused = false;

    start_ticks = SDL_GetTicks();
    paused_ticks = 0;
}

auto Timer::stop() -> void {
    started = false;
    paused = false;

    start_ticks = 0;
    paused_ticks = 0;
}

auto Timer::pause() -> void {
    if(started and !paused) {
        
        paused = true;

        paused_ticks = SDL_GetTicks() - start_ticks;
        start_ticks = 0;
    }
}

auto Timer::unpause() -> void {
    
    if(!started and paused) {

        paused = false;
        start_ticks = SDL_GetTicks() - paused_ticks;

        paused_ticks = 0;
    }
}

//Gets the timer's time
auto Timer::get_ticks() -> uint32_t {
    
    auto time = 0;

    if(!started)
        return time;

    if(paused) {
        time = paused_ticks;
    } else {
        time = SDL_GetTicks() - start_ticks;
    }

    return time;
}

//Checks the status of the timer
auto Timer::is_started() -> bool {
    return started;
}

auto Timer::is_paused() -> bool {
    return paused;
}
