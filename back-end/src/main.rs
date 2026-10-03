use std::io;

// [!] you import all modules/files into main. 
// This makes it easier to import different modules in for example players, 
// since u only need to specify the name instead of an entire path
mod roulette;
mod players;


fn main() {
    roulette::game_loop();
}
