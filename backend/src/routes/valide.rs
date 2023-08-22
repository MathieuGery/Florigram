use serde::{Serialize, Deserialize};
use actix_web::{web, post, Responder, HttpResponse};
use crate::routes::day::Day;

#[derive(Deserialize, Debug, Serialize)]
pub struct FormData {
    word: String,
    words: Vec<String>
}

#[derive(Deserialize, Serialize)]
struct ValideRep {
    words: Vec<String>,
    is_valide: bool,
}

#[post("/valide")]
pub async fn valide(day: web::Data<Day>, mut form: web::Json<FormData>) -> impl Responder {
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
