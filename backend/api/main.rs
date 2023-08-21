mod repertoire;
mod day;

use actix_web::{HttpServer, App, Responder, HttpResponse, get, web, post};
use day::{Date, Day};
use repertoire::Repertoire;
use std::sync::atomic::Ordering;
use serde::{Deserialize, Serialize};
use actix_cors::Cors;

#[derive(Serialize)]
struct DayRep {
    words: Vec<String>,
    victories: usize,
    date: Date,
    letters: String
}

#[get("/day")]
async fn day_route(day: web::Data<Day>) -> impl Responder {

    let words = day.get_words_spaces();
    let rep = DayRep{words,
        victories: day.victories.load(Ordering::Relaxed),
        date: day.date.clone(),
        letters: day.letters.clone()
    };
    let res = HttpResponse::Ok().json(rep);

    return res;
}

#[get("/add")]
async fn add(day: web::Data<Day>) -> impl Responder {
    day.victories.fetch_add(1, Ordering::SeqCst);

    let res = HttpResponse::Ok().content_type("text/html").body(format!("{}", day.victories.load(Ordering::Relaxed)));

    return res;
}

#[derive(Deserialize, Debug, Serialize)]
struct FormData {
    word: String,
    words: Vec<String>
}

#[derive(Deserialize, Serialize)]
struct ValideRep {
    words: Vec<String>,
    is_valide: bool,
}

#[post("/valide")]
async fn valide(day: web::Data<Day>, mut form: web::Json<FormData>) -> impl Responder {
    println!("{:?}", form);

    let is_valide;
    if let Some(index) = day.words.get(&form.word.to_uppercase()) {
        is_valide = true;
        println!("index: {index}");
        form.words[*index] = form.word.clone();
    } else {
        is_valide = false;
    }

    let rep = ValideRep{words: form.words.clone(), is_valide};
    let res = HttpResponse::Ok().json(rep);

    return res;
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let repertoire = Repertoire::new("dictionary.txt");

    let day = 2;
    let month = 2;
    let year = 2005;

    let letters = "XAVIER";

    let mut words: Vec<&repertoire::WordWeight> = repertoire.filter(letters)
        .iter()
        .map(|w| &repertoire[*w]).collect();

    words.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap());

    for pair in &words {
        println!("{}: {}", pair.word, pair.weight);
    }

    let mut words: Vec<String> = words.iter().map(|w| w.word.clone()).collect();

    words.resize(8, String::from("empty"));

    words.sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap());

    for word in &words {
        println!("{word}");
    }

    let date = Date::new(day, month, year);
    let day = web::Data::new(Day::new(date, "XAVIER", &words));


    HttpServer::new(move || {
        let cors = Cors::permissive().allow_any_origin();

        App::new()
            .wrap(cors)
            .app_data(day.clone())
            .service(day_route)
            .service(add)
            .service(valide)
     }).workers(1).bind(("0.0.0.0", 443))?
    .run()
    .await
}
