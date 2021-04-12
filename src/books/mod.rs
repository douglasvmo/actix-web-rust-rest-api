use actix_web::{guard, web};

mod books_handler;
mod books_model;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/books")
            .guard(guard::fn_guard(|head| head.uri.to_string() == "/books"))
            .route(web::get().to(books_handler::index))
            .route(web::post().to(books_handler::create)),
    )
    .service(web::resource("/books").route(web::get().to(books_handler::show_by_author)));
}
