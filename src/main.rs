use std::io::Cursor;
use actix_multipart::Multipart;
use actix_web::{App, get, HttpResponse, HttpServer, post, web};
use futures_util::StreamExt as _;
use image::DynamicImage;
use image::io::Reader as ImageReader;

use pixify::error::UploadError;

#[get("/")]
async fn index() -> &'static str {
    "Hello world!"
}

#[post("/upload")]
async fn upload(mut payload: Multipart) -> Result<HttpResponse, UploadError> {
    let mut buffer: Vec<u8> = Vec::new();
    while let Some(item) = payload.next().await {
        let mut field = item?;
        if field.name() != "file" {
            continue;
        }

        while let Some(chunk) = field.next().await {
            let mut vec = chunk?.clone().to_vec();
            buffer.append(&mut vec);
        }
    }
    if buffer.is_empty() { return Err(UploadError::PayloadError("file".into())); }
    let _img: DynamicImage = ImageReader::new(Cursor::new(buffer))
        .decode()?;
    Ok(HttpResponse::Ok().into())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::scope("/api").service(index).service(upload)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
