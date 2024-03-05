### Cellular automata engine

HI! 

This is a simple cellular automata engine supporting following automatons:

- All 255 of elementary automatons.
- Conway's Game of Life
- WireWorld
- Langton's Ant

Engine uses 2 threads, one for control and simulation and other one (main) for rendering using SDL2. Synchronisation is done through `std::mutex` and `std::scoped_lock<>`.

#### How to build and run the engine

If not present create directory `build` inside this github repo, `cd` inside it and run the following command:

`cmake ../CMakeLists.txt -B . && make`

This will create `./cellular_automata_engine` program that you can run for simple insutrcitons. You can run it with argument list to see all possible automatons available.

After building you can simply run simulation you desire by typing in terminal:

`./cellular_automata_engine RESOLUTION DESIRED_SIMULATION`  

Where Resolution is desired number of squares in RESOLUTION x RESOLUTION grid, and DESIRED_SIMULATION one of the paramters shown after running `./cellular_automata_engine list`

#### Rules for each game

> For all the program letter `p` pauses and unpauses simulation, letter `g` toggled the grid.

1. RuleZZZ:
      No additional keybidnings after starting, just when you run RuleZZZ choose interesting ZZZ number i suggest 030 or 090.

2. Conways_Game:
   Please pause the game to be able to draw. You can change which cell you are drawing by pressing 1 (DEAD) or 2 (Alive) on your keyboard.

3. Langtons_Ant:
    No additional keybindings, just run the program

4. WireWorld:
   Key 1 -> None, Key 2 -> conductor, Key 3 -> Head, Key 4 -> Tail. You are able to draw by dragging while holding left mouse button simmilat to Conways Game of Life.

Example:
`./cellular_automata_engine 250 Conways_Game`

#### Thank you for reading and hope you like this mini project

For any bugs please contact me [currently only tested on linux].