use crate::embroidery::canvas::{Canvas, CanvasConfig};
use crate::error::{InvalidPayloadError, UploadError};
use crate::http::multipart::get_bytes;
use actix_multipart::Multipart;
use actix_web::{get, post, HttpResponse};
use futures_util::StreamExt;
use image::{io::Reader as ImageReader, DynamicImage};
use std::collections::HashSet;
use std::io::Cursor;

#[get("/")]
pub async fn index() -> &'static str {
    "Hello world!"
}

#[post("/upload")]
pub async fn upload(mut payload: Multipart) -> Result<HttpResponse, UploadError> {
    let mut fields: HashSet<String> = HashSet::new();
    let mut buffer: Vec<u8> = Vec::new();
    let mut filename: String = String::new();
    let mut n_cells_in_width: Option<u8> = None;
    let mut n_colors: Option<u8> = None;

    while let Some(item) = payload.next().await {
        let field = item?;
        let content_disposition = field.content_disposition();

        if let Some(name) = content_disposition.get_name() {
            if fields.contains(name) {
                return Err(UploadError::from(InvalidPayloadError::InvalidValue(
                    "file".into(),
                    "Field should contain 1 item".into(),
                )));
            } else {
                fields.insert(name.into());
            }
            match name {
                "file" => {
                    filename = content_disposition
                        .get_filename()
                        .unwrap_or("filename")
                        .to_string();
                    buffer = get_bytes(field).await?;
                }
                "n_cells_in_width" => {
                    let content = get_bytes(field).await?;
                    n_cells_in_width = Some(String::from_utf8(content)?.parse().map_err(|_| {
                        UploadError::from(InvalidPayloadError::MissingValue(
                            "n_cells_in_width".into(),
                        ))
                    })?);
                }
                "n_colors" => {
                    let content = get_bytes(field).await?;
                    n_colors = Some(
                        String::from_utf8(content)?
                            .parse()
                            .map_err(|_| InvalidPayloadError::MissingValue("n_colors".into()))?,
                    );
                    if n_colors.unwrap_or_default() > 200 {
                        return Err(UploadError::from(InvalidPayloadError::InvalidValue(
                            "n_colors".into(),
                            "Value should be less than 200".into(),
                        )));
                    }
                }
                _ => {}
            }
        };
    }
    if buffer.is_empty() {
        return Err(UploadError::from(InvalidPayloadError::MissingValue(
            "file".into(),
        )));
    }

    let img: DynamicImage = ImageReader::new(Cursor::new(buffer))
        .with_guessed_format()
        .map_err(UploadError::ImageFormat)?
        .decode()?;
    let config = CanvasConfig::new(img, n_cells_in_width, n_colors);
    let embroidery = Canvas::new(config).picture;

    // let mut bytes: Vec<u8> = Vec::new();
    // pxl_img.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)?;
    // Ok(HttpResponse::Ok()
    //     .content_type("image/png")
    //     .append_header((
    //         "Content-Disposition",
    //         format!("attachment; filename={filename}"),
    //     ))
    //     .body(bytes))
    Ok(HttpResponse::Ok().json(embroidery))
}
