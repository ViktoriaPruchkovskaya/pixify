use actix_multipart::MultipartError;
use actix_web::{HttpResponse, ResponseError};
use std::string::FromUtf8Error;

#[derive(thiserror::Error, Debug)]
pub enum UploadError {
    #[error(transparent)]
    InvalidPayload(#[from] InvalidPayloadError),
    #[error("Cannot extract a file from form")]
    Form(
        #[from]
        #[source]
        MultipartError,
    ),
    #[error(transparent)]
    Image(#[from] image::ImageError),
    #[error(transparent)]
    ImageFormat(#[from] std::io::Error),
    #[error(transparent)]
    Conversion(#[from] FromUtf8Error),
}

#[derive(thiserror::Error, Debug)]
pub enum InvalidPayloadError {
    #[error("Missing value. Expected '{0}' to be provided")]
    MissingValue(String),
    #[error("Invalid value in '{0}'. {1}")]
    InvalidValue(String, String),
}

impl ResponseError for UploadError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UploadError::InvalidPayload(_) => HttpResponse::BadRequest().json(self.to_string()),
            UploadError::Form(s) => HttpResponse::BadRequest().json(s.to_string()),
            s => HttpResponse::InternalServerError().json(s.to_string()),
        }
    }
}
