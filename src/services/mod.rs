use actix_web::{guard, web};

mod users;

pub fn all_services(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
            .guard(guard::fn_guard(|header| header.uri.query().is_none()))
            .route(web::get().to(users::index))
            .route(web::post().to(users::create)),
    );
    // .route("/books", web::get().to(books_handler::show_by_author))
    // .route("/users/{id}", web::get().to(books_handler::show_a_book));
}
