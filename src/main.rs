use std::{io, str};
use rand::Rng;
use std::cmp::Ordering;
mod hangman;



struct Player { // Example Structure of a player
    is_human: bool,
    health: i32,
    attack: i16
}

enum CardinalDirections {
    North,
    South,
    East,
    West
}


fn main() {

    
    //println!("{}", count_digits(123));
    println!("Hello World");

    

}

fn count_digits(mut n: i32) -> i32 {
    let mut digit_count = 0;
    while n != 0 {
        n = n / 10;
        digit_count+=1;
    }
    digit_count

}
