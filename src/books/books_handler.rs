use super::books_model::{Book, NewBooks};
use actix_web::{web, HttpResponse};
use serde::Deserialize;

use crate::Pool;

pub async fn index(db: web::Data<Pool>) -> HttpResponse {
    let conn = db.get().unwrap();
    let books = Book::all(&conn);

    HttpResponse::Ok().json(books)
}

pub async fn create(db: web::Data<Pool>, book: web::Json<NewBooks>) -> HttpResponse {
    let conn = db.get().unwrap();
    let is_ok = Book::insert(book.into_inner(), &conn);

    match is_ok {
        true => HttpResponse::Created().finish(),
        false => HttpResponse::BadRequest().finish(),
    }
}

#[derive(Deserialize)]
pub struct ShowByAuthor {
    author: String,
}

pub async fn show_by_author(db: web::Data<Pool>, query: web::Query<ShowByAuthor>) -> HttpResponse {
    let conn = db.get().unwrap();
    let books = Book::all_by_author(query.author.clone(), &conn);

    HttpResponse::Ok().json(books)
}

pub async fn show_a_book(db: web::Data<Pool>, req: web::HttpRequest) -> HttpResponse {
    let conn = db.get().unwrap();
    let id = req.match_info().query("id").parse::<i32>().unwrap();
    let book = Book::show(id, &conn).pop();

    HttpResponse::Ok().json(book)
}
