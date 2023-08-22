use serde::{Serialize, Deserialize};
use actix_web::{get, HttpResponse, web, Responder};
use std::sync::atomic::Ordering;
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
        let mut words = vec![String::new(); self.words.len()];

        for (key, val) in self.words.iter() {
            words[*val] = to_spaces(&key);
        }

        return words;
    }
}

impl Date {
    pub fn new(day: usize, month: usize, year: usize) -> Self {
        return Self{ day, month, year };
    }

}

#[derive(Serialize)]
struct DayRep {
    words: Vec<String>,
    victories: usize,
    date: Date,
    letters: String
}

#[get("/day")]
pub async fn day_route(day: web::Data<Day>) -> impl Responder {

    let words = day.get_words_spaces();
    let rep = DayRep{words,
        victories: day.victories.load(Ordering::Relaxed),
        date: day.date.clone(),
        letters: day.letters.clone()
    };
    let res = HttpResponse::Ok().json(rep);

    return res;
}
