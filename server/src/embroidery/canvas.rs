use image::imageops::FilterType;
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

#[derive(Debug, Clone)]
pub struct CanvasConfig {
    pub img: DynamicImage,
    width: u32,
    height: u32,
    cell_height: f32,
    rows: u32,
    columns: u32,
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
        let (width, height) = img.dimensions();

        let columns = n_cells_in_width.unwrap_or(32) as u32;
        let cell_height = width as f32 / columns as f32;
        let rows = (height as f32 / cell_height).round() as u32;

        Ok(CanvasConfig {
            cell_height,
            img,
            width,
            height,
            columns,
            rows,
            n_colors: n_colors.unwrap_or(20),
        })
    }
}

#[derive(Serialize)]
pub struct Canvas {
    pub embroidery: Vec<Vec<RgbColor>>,
    pub colors: Vec<DmcColor>,
    #[serde(skip)]
    config: CanvasConfig,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Palette {
    identifier: String,
    color: DmcColor,
    n_stitches: u32,
}

impl Canvas {
    pub fn new(config: CanvasConfig) -> Result<Self, CanvasError> {
        let colors = config.img.get_dmc_palette(config.n_colors)?;

        let mut pic = config
            .img
            .resize(config.columns, config.rows, FilterType::CatmullRom)
            .to_rgb8();

        let mut embroidery: Vec<Vec<RgbColor>> = Vec::with_capacity(config.rows as usize);
        for y in 0..config.rows {
            let mut row: Vec<RgbColor> = Vec::with_capacity(config.columns as usize);
            for x in 0..config.columns {
                let major_color = pic.get_pixel_mut(x, y).0;
                let major_color_lab = Lab::from_rgb(&major_color);
                let closest_color = colors
                    .iter()
                    .min_by_key(|&color| {
                        let dmc_color_lab = Lab::from_rgb(&color.rgb.into());
                        RgbColor::calculate_diff(major_color_lab, dmc_color_lab) as u32
                    })
                    .ok_or(CanvasError::DmcNotFound)?;
                row.push(closest_color.rgb);
            }
            embroidery.push(row)
        }

        Ok(Canvas {
            config,
            embroidery,
            colors,
        })
    }

    pub fn get_bytes(&self) -> Result<Vec<u8>, CanvasError> {
        let width = self.config.width;
        let height = self.config.height;
        let mut image = DynamicImage::new(width, height, ColorType::Rgb8);

        for (n_row, row) in self.embroidery.iter().enumerate() {
            let n_row = n_row as f32;
            let y_start = (n_row * self.config.cell_height).ceil() as u32;
            let current_row_limit =
                (((n_row + 1.0) * self.config.cell_height).ceil() as u32).min(height);

            for (n_cell, cell) in row.iter().enumerate() {
                let n_cell = n_cell as f32;
                let x_start = (n_cell * self.config.cell_height).ceil() as u32;
                let cell_limit =
                    (((n_cell + 1.0) * self.config.cell_height).ceil() as u32).min(width);

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

    pub fn get_dmc_palette(&self) -> Vec<Palette> {
        let mut palette: Vec<Palette> = Vec::with_capacity(self.colors.len());
        let threads: HashMap<RgbColor, u32> = Self::calculate_stitches(self);
        let mut colors = self.colors.clone();
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
                    n_stitches: *thread,
                });
                identifier += 1;
            }
        }
        palette
    }

    fn calculate_stitches(&self) -> HashMap<RgbColor, u32> {
        let mut stitches: HashMap<RgbColor, u32> = HashMap::with_capacity(self.colors.len());
        for row in &self.embroidery {
            for color in row {
                stitches
                    .entry(*color)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }
        stitches
    }
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
        let canvas_palette = canvas.get_dmc_palette();

        assert_eq!(canvas.embroidery[0].len(), n_cells_in_width as usize);
        assert_eq!(canvas_palette.len(), n_colors as usize);
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
