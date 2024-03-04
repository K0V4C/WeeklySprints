#include "../header/utility.hpp"
#include "../header/objects.hpp"

namespace control {
    auto run() -> void {
        G_Objects::game->game_loop();
    }
}

namespace io {
    auto input_msg(int argc, char* argv[]) -> bool {
        if(argc <= 1) {

             std::cout << "\n\n\t=====================================================================================\n\n";

             std::cout << "\tHI! This is a simple cellular automata engine for 1d and 2d automatons\n";
             std::cout << "\tIf u want to list all possible simulations use ./cellular_automata list\n";

             std::cout << "\n\n\tTo run the program use ./cellular_automata 20 Conway_Game where 20 is desired resolution\n";

             std::cout << "\n\n\t======================================================================================\n\n";
            return false;
         }

         if(argc == 2 and argv[1] == std::string("list")) {
            
            std::cout << "\n\n\t=====================================================================================\n\n";
            
            std::cout << "\t To show elementary one dimension cellular automaton type RuleZZZ, where ZZZ should be\n";
            std::cout << "\t a number you want to see for example Rule030.\n\n";
            
            std::cout << "\t List of 2d cellular automatons:\n";
            for(auto& s :  simulations)
                std::cout << "\t " << s << std::endl;

            std::cout << "\n\n\t=====================================================================================\n\n";
            return false;
         }

        return true;
    }
}
