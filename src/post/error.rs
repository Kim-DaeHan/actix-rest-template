use actix_web::{
    error::ResponseError,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum PostError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "bad request")]
    BadClientData,

    #[display(fmt = "timeout")]
    Timeout,
}

impl ResponseError for PostError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            PostError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            PostError::BadClientData => StatusCode::BAD_REQUEST,
            PostError::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}
