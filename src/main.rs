#![allow(warnings)]

use std::{io, str};
use rand::Rng;
use std::cmp::Ordering;
mod hangman;

struct Book {
    title: String,
    author: String,
    year_released: String
}


fn main() {

    let Book1 = Book {
        title: String::from("The Hunger Games"),
        author: String::from("kate spade"),
        year_released: String::from("1298")
    };

    let some_str = String::from("Hello");
    let reference = &some_str;
    
    

    
}