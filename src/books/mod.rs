use actix_web::web;

mod books_handler;
mod books_model;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/books")
            .route(web::get().to(books_handler::index))
            .route(web::post().to(books_handler::create)),
    );
}
