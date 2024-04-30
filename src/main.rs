use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

mod n_gram_string;
use crate::n_gram_string::NGram;

struct Freq {
    word: String,
    relative_frequency: u64,
}

fn read_words(file_path: &Path) -> Vec<Freq> {
    let file = File::open(file_path).expect("could not open file");
    let mut buf_reader = BufReader::new(file);

    // skip the csv header
    buf_reader
        .seek_relative(11)
        .expect("could not skip csv header");

    // temporary buffers to read into
    let mut word: Vec<u8> = Vec::new();
    word.reserve(32);
    let mut number: Vec<u8> = Vec::new();
    number.reserve(32);
    let mut file_read_result: Result<usize, std::io::Error> = Ok(1);
    let mut parsed_count = 0;

    let mut freqs: Vec<Freq> = vec![];

    while file_read_result.is_ok_and(|read| read > 0) {
        word.clear();
        file_read_result = buf_reader.read_until(b',', &mut word);

        if !file_read_result.is_ok_and(|read| read > 0) {
            println!("ran out of words");
            break;
        }

        let mut word_string = String::from_utf8(word.clone())
            .expect("could not turn Vec<u8> containing word into a string");
        word_string.truncate(word_string.len() - 1);

        number.clear();
        file_read_result = buf_reader.read_until(b'\n', &mut number);

        if !file_read_result.is_ok() {
            panic!("could not read number :(");
        }

        let mut number_string =
            String::from_utf8(number.clone()).expect("could not turn `number` into a string");
        number_string.truncate(number_string.len() - 1);
        let number_int = number_string
            .parse::<u64>()
            .expect("could not parse int 🙁");

        freqs.push(Freq {
            word: word_string,
            relative_frequency: number_int,
        });

        parsed_count += 1;
        if parsed_count % 1000 == 0 {
            println!("parsed {} items", parsed_count);
        }
        if parsed_count >= 50000 {
            break;
        }
    }
    freqs
}

fn main() {
    // vectors to store the word frequency data
    let freqs = read_words(Path::new("./data/unigram_freq.csv"));

    for i in 0..10 {
        println!("{}", freqs[i].word);
    }

    fn build_gram_freq_table<const GRAM_SIZE: usize>(
        freqs: &Vec<Freq>,
    ) -> HashMap<[char; GRAM_SIZE], u64> {
        let mut freq_table: HashMap<[char; GRAM_SIZE], u64> = HashMap::new();
        for freq in freqs {
            for bigram in freq.word.grams::<GRAM_SIZE>() {
                if freq_table.contains_key(&bigram) {
                    *freq_table.get_mut(&bigram).unwrap() += freq.relative_frequency;
                } else {
                    freq_table.insert(bigram, freq.relative_frequency);
                }
            }
        }
        freq_table
    }

    let bigrams = build_gram_freq_table::<2>(&freqs);
    // let trigrams = build_gram_freq_table::<3>(&freqs);

    println!("sorting bigrams");
    let mut pairs: Vec<_> = bigrams.iter().collect();
    pairs.sort_by(|i0, i1| i1.1.cmp(i0.1));

    for (i, (bigram, count)) in pairs.iter().enumerate() {
        println!("{:?}: {}", bigram, count);
        if i > 20 {
            break;
        }
    }
}
