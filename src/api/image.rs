use std::io::Cursor;
use actix_multipart::Multipart;
use actix_web::{get, HttpResponse, post};
use futures_util::StreamExt as _;
use image::{DynamicImage, GenericImageView, ImageBuffer, io::Reader as ImageReader, Rgb, RgbImage};
use crate::api::colors::RGBColor;
use crate::error::UploadError;

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
    let (width, height) = img.dimensions();
    let mut small_img = img.resize(width / 5, height / 5, image::imageops::FilterType::CatmullRom);
    let (width_sm, height_sm) = small_img.dimensions();
    let mut out: RgbImage = ImageBuffer::new(width_sm, height_sm);
    for (x, y, pixel) in out.enumerate_pixels_mut() {
        let [red, green, blue, ..] = small_img.get_pixel(x, y).0;
        let rgb = RGBColor { red, green, blue };
        let (rgb, ..) = rgb.find_dmc();

        *pixel = Rgb([rgb.red, rgb.green, rgb.blue]);
    }
    let mut bytes: Vec<u8> = Vec::new();
    out.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)?;
    Ok(HttpResponse::Ok().content_type("image/png")
        .append_header(("Content-Disposition", format!("attachment; filename={filename}")))
        .body(bytes))
}
