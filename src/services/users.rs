use crate::repositories::user::UserRepository;
use crate::{config::Pool, repositories::user::NewUser};
use actix_web::{get, post, web, HttpResponse};

#[get("/users")]
pub async fn index(pool: web::Data<Pool>) -> HttpResponse {
     UserRepository::new(&pool).get_all()
     .map(|users|HttpResponse::Ok().json(users))
     .into()
}

#[get("/users/{id}")]
pub async fn find(pool: web::Data<Pool>, info: web::Path<i32>) -> HttpResponse {
    UserRepository::new(&pool).find(info.0)
    .map(|user|HttpResponse::Ok().json(user))
    .into()
}

#[post("/users")]
pub async fn create(pool: web::Data<Pool>, new_user: web::Json<NewUser>) -> HttpResponse {
    UserRepository::new(&pool)
        .create(new_user.into_inner())
        .map(|user| HttpResponse::Created().json(user))
        .into()
}
