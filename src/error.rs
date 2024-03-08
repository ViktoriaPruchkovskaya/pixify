use actix_multipart::MultipartError;
use actix_web::{HttpResponse, ResponseError};

#[derive(thiserror::Error, Debug)]
pub enum UploadError {
    #[error("Invalid payload. Expected '{0}'")]
    PayloadError(String),
    #[error("Cannot extract a file from form")]
    FormError(
        #[from]
        #[source]
        MultipartError,
    ),
    #[error(transparent)]
    ImageError(
        #[from]
        image::ImageError,
    ),
}

impl ResponseError for UploadError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UploadError::PayloadError(_) => HttpResponse::BadRequest().json(self.to_string()),
            UploadError::FormError(s) => HttpResponse::BadRequest().json(s.to_string()),
            s => HttpResponse::InternalServerError().json(s.to_string()),
        }
    }
}