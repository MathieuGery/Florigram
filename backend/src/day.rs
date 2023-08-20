use serde::{Serialize, Deserialize};
use std::sync::atomic::AtomicUsize;
use std::collections::hash_set::HashSet;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Date {
    day: usize,
    month: usize,
    year: usize
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Day {
    pub words: HashSet<String>,
    pub victories: AtomicUsize,
    pub date: Date,
    pub letters: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WordsSpaces {
    pub words: Vec<String>,
    pub victories: usize,
    pub date: Date,
    pub letters: String
}

fn to_spaces(word: &str) -> String {
    let mut ret = String::with_capacity(word.len());

    for _ in word.chars() {
        ret.push(' ');
    }
    return ret;
}

impl Day {
    pub fn new(date: Date, letters: &str, _words: &Vec<String>) -> Self {
        let mut words = HashSet::with_capacity(_words.len());

        for i in _words {
            words.insert(i.clone());
        }

        return Self { words, victories: AtomicUsize::new(0), date, letters: String::from(letters) };
    }

    pub fn set(&mut self, date: Date, letters: &str, _words: &Vec<String>) {
        let mut words = HashSet::with_capacity(_words.len());

        for i in _words {
            words.insert(i.clone());
        }

        self.words = words;
        self.date = date;
        self.victories = AtomicUsize::new(0);
        self.letters = String::from(letters);
    }

    pub fn convert_words_spaces(&self) -> WordsSpaces {
        let mut words = Vec::with_capacity(self.words.len());

        for i in &self.words {
            words.push(to_spaces(i));
        }

        return WordsSpaces {
            words,
            victories: self.victories.load(std::sync::atomic::Ordering::SeqCst),
            date: self.date.clone(),
            letters: self.letters.clone(),
        }
    }
}

impl Date {
    pub fn new(day: usize, month: usize, year: usize) -> Self {
        return Self{ day, month, year };
    }

}
