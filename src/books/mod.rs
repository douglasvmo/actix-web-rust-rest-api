use actix_web::{guard, web};

mod books_handler;
mod books_model;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/books")
            .guard(guard::fn_guard(|header| header.uri.query().is_none()))
            .route(web::get().to(books_handler::index))
            .route(web::post().to(books_handler::create)),
    )
    .route("/books", web::get().to(books_handler::show_by_author));
}
