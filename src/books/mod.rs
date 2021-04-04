use actix_web::web;

mod books_model;
mod books_handler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/books").route(web::get().to(books_handler::index)));
}