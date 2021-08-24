use std::io::{self, Read};
use rand::{Rng, random};

pub fn start_game(number_of_chances: usize) {

    let mut rng = rand::thread_rng(); //Start the thread that handles RNG
    let possible_words: [&str; 3] = ["Hello", "Word", "Apple"]; // Words to be used for the hangman game
    let random_number = rng.gen_range(0..possible_words.len()); // Generate the index to get a random word
    let random_word = possible_words[random_number];
    let letters_correct = 0;
    let letters_guessed: Vec<char> = Vec::with_capacity(number_of_chances);

    //let index = possible_words[0].find('e').unwrap();


    // Give the user 10 tries to guess the whole word
    for _ in 0..number_of_chances {

        let mut guess_string = String::new();
        let word_length = random_word.len();

        if letters_correct == word_length {
            println!("You have won the game! congratulations");
        }

        //Check to see if char is inside of String
        //if true increment letters correct
        //finally add char to vector

        println!("Please enter a letter: ");
    
        match io::stdin().read_line(&mut guess_string) { // Basic standard input
            Ok(_) => println!("You entered: {}", guess_string),
            Err(e) => println!("Something went wrong: {}", e)
        };
    
        let letter_guessed = guess_string.chars().nth(0).unwrap(); // Get the first letter of the input
        

        if random_word.contains(letter_guessed) {
            let index = guess_string.find(letter_guessed).unwrap(); // Get the index where the letter is
            println!("Found letter '{letter}' in {word} at index {index}", letter=letter_guessed, word=random_word, index=index);
            
        } else {
            println!("Letter was not found, please try again!")
        }

       // letters_guessed.iter().find('a');


    }

   
    
    
    



    //println!("{}", index);

    //println!("{}", possible_words[random_number]);
    
}