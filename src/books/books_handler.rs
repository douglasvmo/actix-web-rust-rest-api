use super::books_model::{Book, NewBooks};
use actix_web::{http, web, HttpRequest, HttpResponse, Responder};

use crate::Pool;

pub async fn index(db: web::Data<Pool>) -> HttpResponse {
    let conn = db.get().unwrap();
    let books = Book::all(&conn);

    HttpResponse::Ok().json(books)
}

pub async fn create(db: web::Data<Pool>, book: web::Json<NewBooks>) -> HttpResponse {
    let conn = db.get().unwrap();
    let is_ok = Book::insert(book.into_inner(), &conn);

    let status = match is_ok {
        true => http::StatusCode::CREATED,
        false => http::StatusCode::BAD_REQUEST,
    };

    HttpResponse::Ok().status(status).finish()
}
