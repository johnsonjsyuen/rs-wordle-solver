use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

// All words indexed by letter and occurrance.
// Each letter has an alphabetical index (A-Z). Words with a corresponding letter in the index will have a reference for that letter.
// We also store a frequency count for each letter for each position.
struct WordIndex{
    word_to_num: HashMap<String, usize>,
    num_to_word: HashMap<usize, String>,
    letter_index: Vec<HashSet<usize>>,
    freq_index: Vec<Vec<(String,usize)>>
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn letter_to_num(mut maybe_letter: String) ->Result<usize, dyn Error>{
    if maybe_letter.len() != 1 {
     return Err("Only converting a letter a time");
    }
    maybe_letter.make_ascii_lowercase();
    let conver:HashMap<String,usize> = HashMap(["a",1,"b",2,"c",3,"d",4,"e",5,"f",6,"g",7,"h",8,"i",9,"j",10,"k",11,"l",12,
    "m",13,"n",14,"o",15,"p",16,"q",17,"r",18,"s",19,"t",20,"u",21,"v",22,"w",23,"x",24,"y",25,"z",26]);
    return match conver.get(&maybe_letter) {
        Some(n) => Ok(*n),
        None => Err("")
    }
}

impl WordIndex {
    fn new()->WordIndex{
        let mut word_to_num: HashMap<String, usize> = HashMap::new();
        let mut num_to_word: HashMap<usize, String> = HashMap::new();
        let mut letter_index= Vec::new();
        let mut freq_index= Vec::new();


        if let Ok(lines) = read_lines("/home/johnso/CLionProjects/rs-wordle-solver/src/resources/english_words_original_wordle.txt") {
            // Consumes the iterator, returns an (Optional) String
            for (idx, line) in lines.enumerate() {
                if let Ok(word) = line {
                    word_to_num.insert(word.clone(),idx);
                    num_to_word.insert(idx,word);

                }
            }
        }

        WordIndex{
            word_to_num,
            num_to_word,
            letter_index,
            freq_index
        }
    }
}