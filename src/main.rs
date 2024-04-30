use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

mod n_gram_string;
use crate::n_gram_string::NGramString;

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

fn score_word<const GRAM_SIZE: usize>(
    word: &String,
    gram_freq: &HashMap<[char; GRAM_SIZE], u64>,
) -> u64 {
    let mut score: u64 = 0;
    for gram in word.grams::<GRAM_SIZE>() {
        score += gram_freq.get(&gram).unwrap_or(&0);
    }
    score
}

fn main() {
    // vectors to store the word frequency data
    let freqs = read_words(Path::new("./data/unigram_freq.csv"));

    dbg!(freqs[0..10].iter().map(|f| &f.word ).collect::<Vec<&String>>());

    fn build_gram_freq_table<const GRAM_SIZE: usize>(
        freqs: &Vec<Freq>,
    ) -> HashMap<[char; GRAM_SIZE], u64> {
        let mut freq_table: HashMap<[char; GRAM_SIZE], u64> = HashMap::new();
        for freq in freqs {
            for gram in freq.word.grams::<GRAM_SIZE>() {
                if freq_table.contains_key(&gram) {
                    *freq_table.get_mut(&gram).unwrap() += freq.relative_frequency;
                } else {
                    freq_table.insert(gram, freq.relative_frequency);
                }
            }
        }
        freq_table
    }

    let bigrams = build_gram_freq_table::<2>(&freqs);
    let trigrams = build_gram_freq_table::<3>(&freqs);

    println!("sorting bigrams");
    let mut pairs: Vec<_> = bigrams.iter().collect();
    pairs.sort_by(|i0, i1| i1.1.cmp(i0.1));

    for (bigram, count) in pairs[0..15].iter() {
        println!("{:?}: {}", bigram, count);
    }

    let mut test_words: Vec<String> = vec![
        "boorish".to_string(),
        "dslkdsflkjf".to_string(),
        "vioghem".to_string(),
        "violin".to_string(),
        "violence".to_string(),
        "abracadabra".to_string(),
        "happy".to_string(),
        "sad".to_string(),
    ];

    test_words.sort_by(|w1, w2| {
        let score1 = (score_word::<2>(w1, &bigrams) + score_word::<3>(w1, &trigrams)) as f64;
        let score2 = (score_word::<2>(w2, &bigrams) + score_word::<3>(w2, &trigrams)) as f64;
        let length1 = (w1.gram_count(2) + w1.gram_count(3)) as f64;
        let length2 = (w2.gram_count(2) + w2.gram_count(3)) as f64;
        (score1 / length1).total_cmp(&(score2 / length2))
    });
    test_words.reverse();

    println!("{:?}", test_words);
}
