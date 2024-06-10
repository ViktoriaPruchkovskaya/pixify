use actix_multipart::Multipart;
use actix_web::{post, HttpResponse};
use futures_util::StreamExt;
use serde::Serialize;
use std::collections::HashSet;

use crate::embroidery::canvas::{Canvas, CanvasConfig, Palette};
use crate::embroidery::colors::RgbColor;
use crate::error::{ExportError, InvalidPayloadError, UploadError};
use crate::http::multipart::get_bytes;

#[derive(Default)]
struct ImageData {
    pub file: FileData,
    pub n_cells_in_width: Option<u8>,
    pub n_colors: Option<u8>,
}

#[derive(Default)]
struct FileData {
    pub buffer: Vec<u8>,
    pub filename: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UploadResponse {
    pub embroidery: Vec<Vec<RgbColor>>,
    pub palette: Vec<Palette>,
}

#[post("/upload")]
pub async fn upload(mut payload: Multipart) -> Result<HttpResponse, UploadError> {
    let data: ImageData = get_data_from_payload(&mut payload).await?;

    let config = CanvasConfig::new(data.file.buffer, data.n_cells_in_width, data.n_colors)?;
    let canvas = Canvas::new(config)?;
    let canvas_palette = canvas.get_dmc_palette();

    Ok(HttpResponse::Ok().json(UploadResponse {
        embroidery: canvas.embroidery,
        palette: canvas_palette,
    }))
}

#[post("/export")]
pub async fn export(mut payload: Multipart) -> Result<HttpResponse, ExportError> {
    let data: ImageData = get_data_from_payload(&mut payload).await?;

    let config = CanvasConfig::new(data.file.buffer, data.n_cells_in_width, data.n_colors)?;
    let canvas_bytes = Canvas::new(config)?.get_bytes()?;

    Ok(HttpResponse::Ok()
        .content_type("image/png")
        .append_header((
            "Content-Disposition",
            format!("attachment; filename={}", data.file.filename),
        ))
        .body(canvas_bytes))
}

async fn get_data_from_payload(payload: &mut Multipart) -> Result<ImageData, InvalidPayloadError> {
    let mut fields: HashSet<String> = HashSet::new();
    let mut data: ImageData = Default::default();

    while let Some(item) = payload.next().await {
        let field = item?;
        let content_disposition = field.content_disposition();

        if let Some(name) = content_disposition.get_name() {
            if fields.contains(name) {
                return Err(InvalidPayloadError::InvalidValue(
                    name.into(),
                    "Field should contain 1 item".into(),
                ));
            } else {
                fields.insert(name.into());
            }
            match name {
                "file" => {
                    data.file.filename = content_disposition
                        .get_filename()
                        .unwrap_or("filename")
                        .to_string();
                    data.file.buffer = get_bytes(field).await?;
                }
                "nCellsInWidth" => {
                    let content = get_bytes(field).await?;
                    data.n_cells_in_width =
                        Some(String::from_utf8(content)?.parse().map_err(|_| {
                            InvalidPayloadError::MissingValue("nCellsInWidth".into())
                        })?);
                }
                "nColors" => {
                    let content = get_bytes(field).await?;
                    let value = String::from_utf8(content)?.parse().map_err(|_| {
                        InvalidPayloadError::InvalidValue(
                            "nColors".into(),
                            "Value should be within 2 and 200".into(),
                        )
                    })?;

                    if value <= 2 || value > 200 {
                        return Err(InvalidPayloadError::InvalidValue(
                            "nColors".into(),
                            "Value should be within 2 and 200".into(),
                        ));
                    }
                    data.n_colors = Some(value);
                }
                _ => {}
            }
        };
    }
    if data.file.buffer.is_empty() {
        return Err(InvalidPayloadError::MissingValue("file".into()));
    }
    Ok(data)
}
