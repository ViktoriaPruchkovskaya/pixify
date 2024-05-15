use image::{
    io::Reader as ImageReader, ColorType, DynamicImage, GenericImage, GenericImageView, Pixel, Rgb,
};
use lab::Lab;
use serde::Serialize;
use std::cmp::Ordering;
use std::{collections::HashMap, io::Cursor};

use crate::embroidery::colors::{DmcColor, RgbColor};
use crate::embroidery::image::ImagePalette;
use crate::error::CanvasError;

#[derive(Debug)]
pub struct CanvasConfig {
    pub img: DynamicImage,
    pub n_cells_in_width: u8,
    pub n_colors: u8,
}

impl CanvasConfig {
    pub fn new(
        bytes: Vec<u8>,
        n_cells_in_width: Option<u8>,
        n_colors: Option<u8>,
    ) -> Result<Self, CanvasError> {
        let img = ImageReader::new(Cursor::new(bytes))
            .with_guessed_format()
            .map_err(CanvasError::ImageFormat)?
            .decode()?;

        Ok(CanvasConfig {
            img,
            n_cells_in_width: n_cells_in_width.unwrap_or(32),
            n_colors: n_colors.unwrap_or(20),
        })
    }
}

#[derive(Serialize)]
pub struct Canvas {
    pub embroidery: Vec<Vec<RgbColor>>,
    pub palette: Vec<Palette>,

    #[serde(skip)]
    config: CanvasConfig,
}

#[derive(Serialize)]
pub struct Palette {
    identifier: String,
    color: DmcColor,
    thread_length: u32,
}

impl Canvas {
    pub fn new(config: CanvasConfig) -> Result<Self, CanvasError> {
        let n_cells_in_width = config.n_cells_in_width;
        let (width, height) = config.img.dimensions();
        let cell_height = width as f32 / n_cells_in_width as f32;
        let rows = (height as f32 / cell_height).round() as u32;
        let embroidery_colors = config.img.get_dmc_palette(config.n_colors)?;

        let mut canvas: Vec<Vec<RgbColor>> = Vec::with_capacity(rows as usize);
        for y in 0..rows {
            let y_start = (y as f32 * cell_height).round() as u32;
            let y_end = (y_start + cell_height as u32).min(height);

            let mut row: Vec<RgbColor> = Vec::with_capacity(n_cells_in_width as usize);
            for x in 0..n_cells_in_width {
                let x_start = (x as f32 * cell_height).round() as u32;
                let x_end = (x_start + cell_height as u32).min(width);

                let major_color = config
                    .img
                    .get_major_color_in_cell(x_start, x_end, y_start, y_end);
                let major_color_lab = Lab::from_rgb(&major_color.into());
                let closest_color = embroidery_colors
                    .iter()
                    .min_by_key(|&color| {
                        let dmc_color_lab = Lab::from_rgb(&color.rgb.into());
                        RgbColor::calculate_diff(major_color_lab, dmc_color_lab) as u32
                    })
                    .ok_or(CanvasError::DmcNotFound)?;

                row.push(closest_color.rgb);
            }
            canvas.push(row)
        }

        let palette = get_dmc_palette(&canvas, &embroidery_colors);

        Ok(Canvas {
            embroidery: canvas,
            palette,
            config,
        })
    }

    pub fn get_bytes(self) -> Result<Vec<u8>, CanvasError> {
        let (width, height) = self.config.img.dimensions();
        let cell_height = width as f32 / self.config.n_cells_in_width as f32;
        let mut image = DynamicImage::new(width, height, ColorType::Rgb8);

        for (n_row, row) in self.embroidery.iter().enumerate() {
            let n_row = n_row as f32;
            let y_start = (n_row * cell_height).ceil() as u32;
            let current_row_limit = (((n_row + 1.0) * cell_height).ceil() as u32).min(height);

            for (n_cell, cell) in row.iter().enumerate() {
                let n_cell = n_cell as f32;
                let x_start = (n_cell * cell_height).ceil() as u32;
                let cell_limit = (((n_cell + 1.0) * cell_height).ceil() as u32).min(width);

                for y in y_start..current_row_limit {
                    for x in x_start..cell_limit {
                        let color: Rgb<u8> = (*cell).into();
                        image.put_pixel(x, y, color.to_rgba())
                    }
                }
            }
        }

        let mut bytes: Vec<u8> = Vec::new();
        image.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)?;
        Ok(bytes)
    }
}

fn get_dmc_palette(canvas: &Vec<Vec<RgbColor>>, colors: &Vec<DmcColor>) -> Vec<Palette> {
    let mut palette: Vec<Palette> = Vec::with_capacity(colors.len());
    let threads: HashMap<RgbColor, u32> = calculate_threads(canvas, colors);
    let mut colors = colors.clone();
    colors.sort_by(|color_1, color_2| {
        let lab_1 = Lab::from_rgb(&color_1.rgb.into());
        let lab_2 = Lab::from_rgb(&color_2.rgb.into());

        (lab_1.l, lab_1.a, lab_1.b)
            .partial_cmp(&(lab_2.l, lab_2.a, lab_2.b))
            .unwrap_or(Ordering::Equal)
    });

    let mut identifier: u8 = 1;
    for &color in colors.iter() {
        if let Some(thread) = threads.get(&color.rgb) {
            palette.push(Palette {
                identifier: format!("{:02}", identifier),
                color,
                thread_length: *thread,
            });
            identifier += 1;
        }
    }
    palette
}

fn calculate_threads(canvas: &Vec<Vec<RgbColor>>, colors: &[DmcColor]) -> HashMap<RgbColor, u32> {
    let mut threads: HashMap<RgbColor, u32> = HashMap::with_capacity(colors.len());
    for row in canvas {
        for &color in row {
            threads
                .entry(color)
                .and_modify(|count| *count += 1)
                .or_insert(10);
        }
    }
    threads
}

#[cfg(test)]
mod test {
    use super::*;
    use image::ImageBuffer;

    fn generate_image_bytes(width: Option<u32>, height: Option<u32>) -> Vec<u8> {
        let image_buffer =
            ImageBuffer::from_fn(width.unwrap_or(50), height.unwrap_or(50), |x, y| {
                let r = ((x % 256) + 20) as u8;
                let g = ((y % 256) + 30) as u8;
                let b = ((x + y) % 256) as u8;
                Rgb([r, g, b])
            });
        let dynamic_image = DynamicImage::ImageRgb8(image_buffer);

        let mut bytes = Vec::new();
        dynamic_image
            .write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)
            .unwrap();

        bytes
    }

    #[test]
    fn it_creates_config_invalid_format() {
        let bytes = vec![123, 200, 1];
        let n_cells_in_width: u8 = 10;
        let n_colors: u8 = 5;

        let err = CanvasConfig::new(bytes, Some(n_cells_in_width), Some(n_colors)).unwrap_err();
        assert_eq!(err.to_string(), "The image format could not be determined");
    }

    #[test]
    fn it_gets_canvas() {
        let bytes = generate_image_bytes(None, None);
        let n_cells_in_width: u8 = 10;
        let n_colors: u8 = 5;

        let config = CanvasConfig::new(bytes, Some(n_cells_in_width), Some(n_colors)).unwrap();
        let canvas = Canvas::new(config).unwrap();

        assert_eq!(canvas.embroidery[0].len(), n_cells_in_width as usize);
        assert_eq!(canvas.palette.len(), n_colors as usize);
    }

    #[test]
    fn it_gets_canvas_bytes() {
        let bytes = generate_image_bytes(Some(10), Some(10));
        let n_cells_in_width: u8 = 10;
        let n_colors: u8 = 5;

        let config = CanvasConfig::new(bytes, Some(n_cells_in_width), Some(n_colors)).unwrap();
        let canvas_bytes = Canvas::new(config).unwrap().get_bytes().unwrap();

        let canvas = ImageReader::new(Cursor::new(canvas_bytes))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap();
        let (width, height) = canvas.dimensions();
        assert_eq!(width, 10);
        assert_eq!(height, 10);
    }
}
