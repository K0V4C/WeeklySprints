#ifndef TIMER
#define TIMER

#include <cstdint>
class Timer {
public:
    //Initializes variables
    Timer();

    //The various clock actions
    auto start() -> void;
    auto stop() -> void;
    auto pause() -> void;
    auto unpause() -> void;

    //Gets the timer's time
    auto get_ticks() -> uint32_t;

    //Checks the status of the timer
    auto is_started() -> bool;
    auto is_paused() -> bool;

private:
    //The clock time when the timer started
    uint32_t start_ticks;

    //The ticks stored when the timer was paused
    uint32_t paused_ticks; 

    //The timer status
    bool paused;
    bool started;

};

#endif
