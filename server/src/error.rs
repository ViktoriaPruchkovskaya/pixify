use actix_multipart::MultipartError;
use actix_web::{HttpResponse, ResponseError};
use std::string::FromUtf8Error;

#[derive(thiserror::Error, Debug)]
pub enum UploadError {
    #[error(transparent)]
    InvalidPayload(#[from] InvalidPayloadError),
    #[error(transparent)]
    Conversion(#[from] FromUtf8Error),
    #[error(transparent)]
    Canvas(#[from] CanvasError),
}

impl ResponseError for UploadError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UploadError::InvalidPayload(err) => err.error_response(),
            s => HttpResponse::InternalServerError().json(s.to_string()),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ExportError {
    #[error(transparent)]
    InvalidPayload(#[from] InvalidPayloadError),
    #[error(transparent)]
    Conversion(#[from] FromUtf8Error),
    #[error(transparent)]
    Canvas(#[from] CanvasError),
}

impl ResponseError for ExportError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ExportError::InvalidPayload(err) => err.error_response(),
            s => HttpResponse::InternalServerError().json(s.to_string()),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum InvalidPayloadError {
    #[error("Missing value. Expected '{0}' to be provided")]
    MissingValue(String),
    #[error("Invalid value in '{0}'. {1}")]
    InvalidValue(String, String),
    #[error("Cannot extract a file from form")]
    Form(
        #[from]
        #[source]
        MultipartError,
    ),
    #[error(transparent)]
    Conversion(#[from] FromUtf8Error),
}

impl ResponseError for InvalidPayloadError {
    fn error_response(&self) -> HttpResponse {
        match self {
            InvalidPayloadError::Form(s) => HttpResponse::BadRequest().json(s.to_string()),
            _ => HttpResponse::BadRequest().json(self.to_string()),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum CanvasError {
    #[error("Error during DMC determination")]
    DmcNotFound,
    #[error(transparent)]
    ImageFormat(#[from] std::io::Error),
    #[error(transparent)]
    Image(#[from] image::ImageError),
}
