use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Date {
    day: usize,
    month: usize,
    year: usize
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Day {
    pub words: HashMap<String, usize>,
    pub victories: AtomicUsize,
    pub date: Date,
    pub letters: String
}

pub fn to_spaces(word: &str) -> String {
    let mut ret = String::with_capacity(word.len());

    for _ in word.chars() {
        ret.push(' ');
    }
    return ret;
}

impl Day {
    pub fn new(date: Date, letters: &str, _words: &Vec<String>) -> Self {
        let mut words = HashMap::with_capacity(_words.len());

        for (i, word) in _words.iter().enumerate() {
            words.insert(word.clone(), i);
        }

        return Self { words, victories: AtomicUsize::new(0), date, letters: String::from(letters) };
    }

    pub fn set(&mut self, date: Date, letters: &str, _words: &Vec<String>) {
        let mut words = HashMap::with_capacity(_words.len());

        for (i, word) in _words.iter().enumerate() {
            words.insert(word.clone(), i);
        }

        self.words = words;
        self.date = date;
        self.victories = AtomicUsize::new(0);
        self.letters = String::from(letters);
    }

    pub fn get_words_spaces(&self) -> Vec<String> {
        let mut words = Vec::with_capacity(self.words.len());

        for key in self.words.keys() {
            words.push(to_spaces(&key));
        }

        return words;
    }
}

impl Date {
    pub fn new(day: usize, month: usize, year: usize) -> Self {
        return Self{ day, month, year };
    }

}

//it is possible to change the day through this stuct
// pub struct TMP {
//     pub which: AtomicUsize,
//     pub a: Day,
//     pub b: Day,
// }
