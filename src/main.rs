#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{App, HttpServer};
mod errors;
mod config;
mod schema;
mod services;
mod repositories;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
   let config = config::ServerConfig::from_env();

    let pool = config.configured_pool();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(services::users::create)
            .service(services::users::index)
            .service(services::users::find)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
