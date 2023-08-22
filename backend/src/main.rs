mod repertoire;
mod routes;

use actix_web::{HttpServer, App, Responder, HttpResponse, get, web};
use repertoire::Repertoire;
use std::sync::atomic::Ordering;
use actix_cors::Cors;
use crate::routes::day::{Day, day_route, Date};
use crate::routes::valide::valide;

#[get("/add")]
async fn add(day: web::Data<Day>) -> impl Responder {
    day.victories.fetch_add(1, Ordering::SeqCst);

    let res = HttpResponse::Ok().content_type("text/html").body(format!("{}", day.victories.load(Ordering::Relaxed)));

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
     }).workers(1).bind(("0.0.0.0", 8080))?
    .run()
    .await
}
