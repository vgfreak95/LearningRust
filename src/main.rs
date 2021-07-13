use std::io;
use rand::Rng;
use std::cmp::Ordering;
mod hangman;



struct Player { // Example Structure of a player
    is_human: bool,
    health: i32,
    attack: i16
}
fn main() {
    hangman::start_game();
}