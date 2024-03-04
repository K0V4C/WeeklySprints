#include <SDL2/SDL.h>
#include <SDL2/SDL_mutex.h>
#include <cstdint>
#include <mutex>

#include "../header/automatons/conways_game_of_life.hpp"
#include "../header/automatons/elementary_automaton.hpp"
#include "../header/game.hpp"
#include "../header/objects.hpp"

const int32_t SCREEN_FPS = 60;
const int32_t SCREEN_TICKS_PER_FPS = 1000.0 / SCREEN_FPS;

const int32_t DEFAULT_RESOLUTION = 50;
Game::State Game::state = Game::State::Running;
Game::Game() {
    Game::state = Game::State::Running;
    this->fps_timer = Timer();
    this->cap_timer = Timer();

    this->width = G_Objects::game_window->get_width();
    this->heigth = G_Objects::game_window->get_height();

    this->resolution = DEFAULT_RESOLUTION;
    this->square_side = width / this->resolution;

    this->horizotnal_squares = this->width / this->square_side;
    this->vertical_squares = this->heigth / this->square_side;   

    this->show_grid_lines = true;

    G_Objects::input_cells = Grid<Cell>(horizotnal_squares, vertical_squares);
    G_Objects::output_cells = Grid<Cell>(horizotnal_squares, vertical_squares);

    for(int32_t i = 0; i < G_Objects::output_cells.size(); i++) {
        int32_t x_pos = (i % horizotnal_squares) * square_side;
        int32_t y_pos = (i / horizotnal_squares) * square_side;
        
        G_Objects::output_cells[i] = *(new Cell(x_pos, y_pos, square_side, Cell_Type::None));
        G_Objects::input_cells[i] = *(new Cell(x_pos, y_pos, square_side, Cell_Type::None));
    }
}

auto Game::game_loop() -> void {    
    
    auto counted_frames = 0;
    fps_timer.start();

    while(Game::state != Game::State::Finished) {
        
        cap_timer.start();

        // Game can be paused inside here
        handle_events();
        
        auto average_fps = counted_frames / (fps_timer.get_ticks() / 1000.0f); 
        if(average_fps > 20000) { average_fps = 0; }
        
        if(Game::state == Game::State::Running)
            simulate();

        // In another thread is rendering

        counted_frames += 1;
        auto frame_ticks = cap_timer.get_ticks();
        if( frame_ticks < SCREEN_TICKS_PER_FPS ) {
            SDL_Delay(SCREEN_TICKS_PER_FPS - frame_ticks);
        } 
    }
}


// Handle sdl events
auto Game::handle_events() -> void {
    SDL_Event e;
    while(SDL_PollEvent(&e) != 0) {
        G_Objects::game_window->handle_window_event(e);
        this->automaton->handle_event(e);
        // Toggle pause
        if(e.type == SDL_KEYDOWN) {
            if(e.key.keysym.sym == SDLK_p) {
                if(this->state == Game::State::Running) {
                    this->state = Game::State::Paused;
                    this->cap_timer.pause();
                    this->fps_timer.pause();
                } else {
                    this->state = Game::State::Running;
                    this->cap_timer.unpause();
                    this->fps_timer.unpause();
                }
            }
            
            if(e.key.keysym.sym == SDLK_g) {
                show_grid_lines = !show_grid_lines;
            }

        }
    }
}

// game simulation
auto Game::simulate() -> void {
    {
        // lock object for output cells
        std::scoped_lock<std::mutex> lock(*G_Objects::output_cell_mutex);

        // Calcaulate new grid
        automaton->calculate_new_grid(G_Objects::input_cells, G_Objects::output_cells);
    }


    // Swap and calculate
    G_Objects::input_cells = G_Objects::output_cells;
}

// rendering
auto Game::render() -> void {    
    // Draws backgroudn
    G_Objects::g_canvas->draw_background();

    // Draws individual cells
    G_Objects::g_canvas->draw_cells();

    // Draws grid lines
    if(G_Objects::game->show_grid_lines)
        G_Objects::g_canvas->draw_grid_lines(); 

    // Sends it to graphics card
    G_Objects::g_canvas->present();
}


// Setters
auto Game::set_resolution(int32_t res) -> void {
    this->resolution = res;
    this->square_side = width / this->resolution;

    this->horizotnal_squares = this->width / this->square_side;
    this->vertical_squares = this->heigth / this->square_side;

    G_Objects::input_cells = Grid<Cell>(horizotnal_squares, vertical_squares);
    G_Objects::output_cells = Grid<Cell>(horizotnal_squares, vertical_squares);

    for(int32_t i = 0; i < G_Objects::output_cells.size(); i++) {
        int32_t x_pos = (i % horizotnal_squares) * square_side;
        int32_t y_pos = (i / horizotnal_squares) * square_side;

        G_Objects::output_cells[i] = *(new Cell(x_pos, y_pos, square_side, Cell_Type::None));
        G_Objects::input_cells[i] = *(new Cell(x_pos, y_pos, square_side, Cell_Type::None));
    }

}

auto Game::set_automaton(std::string mode) -> void {

    if( mode.substr(0,4) == "Rule" and mode.size() == 7 ) {
        // Add check in te history
        this->automaton = new Elementary_Automaton(std::atoi(mode.substr(4,7).c_str()));
    }

    if(mode == std::string("Conways_Game")) {
        this->automaton = new Conway_GOL();
    }
}

// Getters
auto Game::get_number_of_cells() const -> int32_t { return this->number_of_cells; }
auto Game::get_width() const -> int32_t { return this->width; } 
auto Game::get_heigth() const -> int32_t { return this->heigth; }
auto Game::get_resolution() const -> int32_t { return this->resolution; }
auto Game::get_horizontal_squares() const -> int32_t { return this->horizotnal_squares; }
auto Game::get_vertical_squares() const -> int32_t { return this->vertical_squares; }
auto Game::get_square_side() const -> int32_t { return this->square_side; }

Game::~Game() {}
