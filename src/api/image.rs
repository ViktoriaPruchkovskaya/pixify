use std::io::Cursor;
use actix_multipart::Multipart;
use actix_web::{get, HttpResponse, post};
use futures_util::StreamExt as _;
use image::{DynamicImage, io::Reader as ImageReader};
use crate::error::UploadError;
use crate::image::DynamicImagePxl;

#[get("/")]
pub async fn index() -> &'static str {
    "Hello world!"
}

#[post("/upload")]
pub async fn upload(mut payload: Multipart) -> Result<HttpResponse, UploadError> {
    let mut buffer: Vec<u8> = Vec::new();
    let mut filename: String = String::new();
    while let Some(item) = payload.next().await {
        let mut field = item?;
        if field.name() != "file" {
            continue;
        }
        filename = field.content_disposition().get_filename().unwrap_or("filename").to_string();
        while let Some(chunk) = field.next().await {
            let mut vec = chunk?.clone().to_vec();
            buffer.append(&mut vec);
        }
    }

    if buffer.is_empty() { return Err(UploadError::PayloadError("file".into())); }

    let img: DynamicImage = ImageReader::new(Cursor::new(buffer)).with_guessed_format()
        .map_err(UploadError::ImageFormatError)?
        .decode()?;
    let pxl = img.pixelate();
    let mut bytes: Vec<u8> = Vec::new();
    pxl.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)?;
    Ok(HttpResponse::Ok().content_type("image/png")
        .append_header(("Content-Disposition", format!("attachment; filename={filename}")))
        .body(bytes))
}