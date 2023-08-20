mod repertoire;
mod day;

use actix_web::{HttpServer, App, Responder, HttpResponse, get, web, post};
use day::{Date, Day};
use std::sync::atomic::Ordering;
use serde::Deserialize;
use actix_cors::Cors;
// use serde_json;

#[get("/day")]
async fn test(day: web::Data<Day>) -> impl Responder {

    let ret = day.convert_words_spaces();
    let res = HttpResponse::Ok().json(ret);

    return res;
}

#[get("/add")]
async fn add(day: web::Data<Day>) -> impl Responder {
    day.victories.fetch_add(1, Ordering::SeqCst);

    let res = HttpResponse::Ok().content_type("text/html").body(format!("{}", day.victories.load(Ordering::Relaxed)));

    return res;
}

#[derive(Deserialize, Debug)]
struct FormData {
    word: String,
}

#[post("/valide")]
async fn valide(day: web::Data<Day>, form: web::Json<FormData>) -> impl Responder {
    println!("{:?}", form);
    let res = HttpResponse::Ok().content_type("text/html").body(format!("{}", day.words.contains(&form.word.to_uppercase())));

    return res;
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let repertoire = repertoire::Repertoire::new("French ODS dictionary.txt");

    let day = 2;
    let month = 2;
    let year = 2005;

    let letters = "XAVIER";

    let mut words = repertoire.filter(letters).iter().map(|i| repertoire[*i].clone()).collect::<Vec<String>>();
    words.resize(8, String::new());

    let date = Date::new(day, month, year);
    let day = web::Data::new(Day::new(date, "XAVIER", &words));


    HttpServer::new(move || {
        let cors = Cors::permissive().allow_any_origin();

        App::new()
            .wrap(cors)
            .app_data(day.clone())
            .service(test)
            .service(add)
            .service(valide)
     }).workers(1).bind(("0.0.0.0", 3000))?
    .run()
    .await
}
