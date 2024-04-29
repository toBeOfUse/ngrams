pub struct GramIterator<'z, const L: usize> {
    string: &'z String,
    position: usize,
}

pub trait NGram {
    fn char_at_with_stops(&self, index: usize) -> char;
    fn len_with_stops(&self) -> usize;
    fn grams<const L: usize>(&self) -> GramIterator<L>;
}

impl NGram for String {
    fn char_at_with_stops(&self, index: usize) -> char {
        if index == 0 {
            return '^';
        } else if index == self.len() + 1 {
            return '$';
        } else {
            return self.chars().nth(index-1).unwrap();
        }
    }
    
    fn len_with_stops(&self) -> usize {
        return self.len() + 2;
    }
    
    fn grams<const L: usize>(&self) -> GramIterator<L> {
        GramIterator {
            string: self,
            position: 0
        }
    }
}

impl<'a, const L: usize> Iterator for GramIterator<'a, L> {
    type Item = [char; L];

    fn next(&mut self) -> Option<Self::Item> {
        if self.position + L - 1 > self.string.len_with_stops() - 1 {
            None
        } else {
            let mut result: Self::Item = ['\0'; L];
            for i in self.position..self.position + L {
                result[i - self.position] = self.string.char_at_with_stops(i);
            }
            self.position += 1;
            Some(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::n_gram_string::NGram;

    #[test]
    fn bigrams() {
        let test = "test".to_string();
        let result: Vec<[char; 2]> = vec![
            ['^', 't'], ['t', 'e'], ['e', 's'], ['s', 't'], ['t', '$']
        ];
        for (i, bigram) in test.grams::<2>().enumerate() {
            println!("{:?}", bigram);
            assert_eq!(result[i], bigram);
        }
    }

    #[test]
    fn trigrams() {
        let test = "test".to_string();
        let result: Vec<[char; 3]> = vec![
            ['^', 't', 'e'], ['t', 'e', 's'], ['e', 's', 't'], ['s', 't', '$']
        ];
        for (i, trigram) in test.grams::<3>().enumerate() {
            println!("{:?}", trigram);
            assert_eq!(result[i], trigram);
        }
    }

    #[test]
    fn quadgrams() {
        let test = "test".to_string();
        let result: Vec<[char; 4]> = vec![
            ['^', 't', 'e', 's'], ['t', 'e', 's', 't'], ['e', 's', 't', '$']
        ];
        for (i, quadgram) in test.grams::<4>().enumerate() {
            println!("{:?}", quadgram);
            assert_eq!(result[i], quadgram);
        }
    }

    #[test]
    fn one_char_trigram() {
        let test = "a".to_string();
        let result: Vec<[char; 3]> = vec![
            ['^', 'a', '$']
        ];
        for (i, trigram) in test.grams::<3>().enumerate() {
            println!("{:?}", trigram);
            assert_eq!(result[i], trigram);
        }
    }

    #[test]
    fn one_char_quadgram() {
        let test = "a".to_string();
        for _nothing in test.grams::<4>() {
            panic!("this iterator should yield nothing!");
        }
    }
}
