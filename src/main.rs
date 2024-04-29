use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use crate::n_gram_string::NGram;

mod n_gram_string;


fn main() {
    // vectors to store the word frequency data
    let mut words: Vec<String> = Vec::new();
    let mut numbers: Vec<u64> = Vec::new();

    // reading words from the file:
    {

        let file = File::open("./data/unigram_freq.csv");
        if !file.is_ok() {
            println!("file not found");
            return;
        }
        let mut buf_reader = BufReader::new(file.unwrap());
        
        // skip the csv header
        buf_reader.seek_relative(11).expect("could not skip csv header");

        // temporary buffers to read to
        let mut word: Vec<u8> = Vec::new();
        word.reserve(32);
        let mut number: Vec<u8> = Vec::new();
        number.reserve(32);
        let mut result: Result<usize, std::io::Error> = Ok(1);

        let mut parsed = 0;

        while result.is_ok_and(|read| read > 0) {
            word.clear();
            result = buf_reader.read_until(b',', &mut word);
            
            if !result.is_ok_and(|read| read > 0) {
                println!("ran out of words");
                break;
            }

            let mut word_string = String::from_utf8(word.clone())
                    .expect("could not turn Vec<u8> containing word into a string");
            word_string.truncate(word_string.len() - 1);

            words.push(word_string);

            number.clear();
            result = buf_reader.read_until(b'\n', &mut number);

            if !result.is_ok() {
                println!("could not read number");
                return;
            }

            let mut number_string = String::from_utf8(number.clone())
                    .expect("could not turn `number` into a string");
            number_string.truncate(number_string.len() - 1);
            numbers.push(number_string.parse::<u64>().expect("could not parse int 🙁"));

            parsed += 1;
            if parsed % 1000 == 0 {
                println!("parsed {} items", parsed);
            }
        }
    }
    
    for i in 0..10 {
        println!("{}", words[i]);
    }
    
    println!("building bigram hashmap");

    let mut bigrams: HashMap<[char; 2], u64> = HashMap::new();

    for i in 0..words.len() {
        for bigram in words[i].grams::<2>() {
            if bigrams.contains_key(&bigram) {
                *bigrams.get_mut(&bigram).unwrap() += numbers[i];
            } else {
                bigrams.insert(bigram, numbers[i]);
            }
        }
    }

    println!("sorting bigrams");
    let mut pairs: Vec<_> = bigrams.iter().collect();
    pairs.sort_by(|i0, i1| i1.1.cmp(i0.1));

    let mut i = 0;
    for (bigram, count) in pairs {
        println!("{:?}: {}", bigram, count);
        i += 1;
        if i > 10 { break; }
    }
}
