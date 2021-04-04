use super::books_model;
use crate::Pool;
use actix_web::{web, HttpResponse};

pub async fn index(db: web::Data<Pool>) -> HttpResponse {
    let conn = db.get().unwrap();
    let books = books_model::Book::all(&conn);

    HttpResponse::Ok().json(books)
}
