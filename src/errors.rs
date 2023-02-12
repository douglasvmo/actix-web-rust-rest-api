use actix_web::{http, HttpResponse, ResponseError};
use diesel::result::Error as DBError;

#[derive(Debug, derive_more::Display)]
pub enum ServiceError {
    InternalServerError,
    BadRequest,
    Conflict,
    NotFound,
    Unauthorized,
}

impl From<DBError> for ServiceError {
    fn from(error: DBError) -> Self{
        match error {
            DBError::DatabaseError(_kind, _info) => {
                match _kind {
                    diesel::result::DatabaseErrorKind::UniqueViolation => Self::Conflict,
                    diesel::result::DatabaseErrorKind::ForeignKeyViolation => Self::Conflict,
                    _ => Self::BadRequest,
                }

            }
            DBError::NotFound  => Self::NotFound,
            _ => Self::InternalServerError,
        }
    }
    
}

impl ResponseError for ServiceError {
    fn status_code(&self) -> http::StatusCode {
        match self {
            Self::InternalServerError => http::StatusCode::INTERNAL_SERVER_ERROR,
            Self::BadRequest => http::StatusCode::BAD_REQUEST,
            Self::Unauthorized => http::StatusCode::UNAUTHORIZED,
            Self::Conflict => http::StatusCode::CONFLICT,
            Self::NotFound => http::StatusCode::NOT_FOUND,
            
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            Self::InternalServerError => HttpResponse::InternalServerError().json("Internal Server Error, Please try later"),
            Self::BadRequest => HttpResponse::BadRequest().into(),
            Self::Conflict => HttpResponse::Conflict().into(),
            Self::NotFound => HttpResponse::NotFound().into(),
            Self::Unauthorized => HttpResponse::Unauthorized().into(),
        }
    }
}