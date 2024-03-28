use std::collections::HashMap;
use std::io::Cursor;
use actix_multipart::Multipart;
use actix_web::{get, HttpResponse, post};
use futures_util::StreamExt as _;
use image::{ColorType, DynamicImage, GenericImage, GenericImageView, io::Reader as ImageReader, Pixel, Rgb};
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
    //TODO: validate number of provided files in one field
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
    // let image = match img {
    //     DynamicImage::ImageRgb8(image) => image,
    //     DynamicImage::ImageLuma8(image) => image.convert(),
    //     DynamicImage::ImageLumaA8(image) => image.convert(),
    //     DynamicImage::ImageRgba8(image) => image.convert(),
    //     DynamicImage::ImageLuma16(image) => image.convert(),
    //     DynamicImage::ImageLumaA16(image) => image.convert(),
    //     DynamicImage::ImageRgb16(image) => image.convert(),
    //     DynamicImage::ImageRgba16(image) => image.convert(),
    //     DynamicImage::ImageRgb32F(image) => image.convert(),
    //     DynamicImage::ImageRgba32F(image) => image.convert(),
    //     _ => panic!("hello")
    // };
    let (width, height) = img.dimensions();
    let n_cells_in_width = 30;
    let cell_height = width / n_cells_in_width;
    let rows = (height as f32 / cell_height as f32).ceil() as u32;
    // let size = n_cells_in_width * rows;
    let mut pxl_img = DynamicImage::new(width, height, ColorType::Rgb8);
    for y in 0..rows {
        let y_start = y * cell_height;
        let y_end = (y_start + 1 + cell_height).min(height);
        for x in 0..n_cells_in_width {
            let x_start = x * cell_height; //maybe +y_start
            let x_end = (x_start + cell_height + 1).min(width);
            let mut color_counts: HashMap<Rgb<u8>, u32> = HashMap::new();
            for y in y_start..y_end {
                for x in x_start..x_end {
                    let color = img.get_pixel(x, y).to_rgb();
                    color_counts.entry(color.clone()).and_modify(|c| *c += 1).or_insert(1);
                }
            }
            let (major_color, ..) = color_counts.iter().max_by_key(|&(_, count)| count).unwrap();
            let rgb = RGBColor { red: major_color[0], green: major_color[1], blue: major_color[2] };
            let (rgb, ..) = rgb.find_dmc();
            for y in y_start..y_end {
                for x in x_start..x_end {
                    pxl_img.put_pixel(x, y, Rgb([rgb.red, rgb.green, rgb.blue]).to_rgba());
                }
            }
        }
    }

    let mut bytes: Vec<u8> = Vec::new();
    pxl_img.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)?;
    Ok(HttpResponse::Ok().content_type("image/png")
        .append_header(("Content-Disposition", format!("attachment; filename={filename}")))
        .body(bytes))
}