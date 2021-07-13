use std::io::{self, Read};
use rand::{Rng, random};

pub fn start_game() {
    let mut rng = rand::thread_rng(); //Start the thread that handles RNG

    let possible_words: [&str; 3] = ["Hello", "Word", "Apple"]; // Words to be used for the hangman game

    let random_number = rng.gen_range(0..possible_words.len()); // Generate the index to get a random word
    
    let random_word = possible_words[random_number];

    let letters_correct = 0;

    let letters_guessed = ();



    //let index = possible_words[0].find('e').unwrap();


    // Give the user 10 tries to guess the whole word
    for _ in 0..10 {

        let mut guess_string = String::new();
        let word_length = random_word.len();

        if letters_correct == word_length {
            println!("You have won the game! congratulations");
        }

        println!("Please enter a letter: ");
        
    
        match io::stdin().read_line(&mut guess_string) { // Basic standard input
            Ok(_) => println!("You entered: {}", guess_string),
            Err(e) => println!("Something went wrong: {}", e)
        };
    
        let first_letter = guess_string.chars().nth(0).unwrap(); // Get the first letter of the input
        
        if random_word.contains(first_letter) {
            let index = guess_string.find(first_letter).unwrap(); // Get the index where the letter is
            println!("Found letter '{letter}' in {word} at index {index}", letter=first_letter, word=random_word, index=index);
        } else {
            println!("Letter was not found, please try again!")
        }

    }

   
    
    
    



    //println!("{}", index);

    //println!("{}", possible_words[random_number]);
    
}