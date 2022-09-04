use crate::config::Pool;
use crate::repositories::user::UserRepository;
use actix_web::{web, HttpResponse};

pub async fn index(pool: web::Data<Pool>) -> HttpResponse {
    let users = UserRepository::new(&pool).get_all();

    HttpResponse::Ok().json(users)
}

pub async fn create(db: web::Data<Pool>, book: web::Json<()>) -> HttpResponse {
    let is_ok = true;

    match is_ok {
        true => HttpResponse::Created().finish(),
        false => HttpResponse::BadRequest().finish(),
    }
}
