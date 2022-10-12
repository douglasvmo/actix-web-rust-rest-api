use crate::{config::Pool, repositories::user::NewUser};
use crate::repositories::user::UserRepository;
use actix_web::{web, HttpResponse};

pub async fn index(pool: web::Data<Pool>) -> HttpResponse {
    let users = UserRepository::new(&pool).get_all();

    HttpResponse::Ok().json(users)
}

pub async fn create(pool: web::Data<Pool>, new_user: web::Json<NewUser>) -> HttpResponse {
    let is_ok = UserRepository::new(&pool).create(new_user.into_inner());

    match is_ok {
        true => HttpResponse::Created().finish(),
        false => HttpResponse::BadRequest().finish(),
    }
}
