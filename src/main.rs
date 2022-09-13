mod word_index;

use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, Read};
use std::path::Path;

const HELP_TEXT: &str = r#"
The WORDLE Solver CLI
(Word length: 5; Plurals: No)
Press CTRL+C to exit

Meanings of symbols:
 +	letter in the word and in the right spot (green box)
 ?	letter in the word but in a wrong spot (orange box)
 _	letter not in the word (grey box)

Commands:
 !done		you're done guessing a hidden word.  this will reset the state of the solver for you to guess a new hidden word
 !tries		see the tries entered
 !remove_last	remove the last try entered

Please enter you last try as word:symbols
"#;



fn get_candidates(in_place_characters:&str,other_characters:&HashSet<String>) -> Vec<&str>{

}

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}",HELP_TEXT);

    /*loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
    }*/
    Ok(())
}
