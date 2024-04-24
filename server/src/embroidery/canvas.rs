use crate::embroidery::colors::{DmcColor, RgbColor};
use crate::error::CanvasError;
use image::{DynamicImage, GenericImageView, Pixel, Rgb};
use lab::Lab;
use palette_extract::{get_palette_with_options, MaxColors, PixelEncoding, PixelFilter, Quality};
use std::cmp::Ordering;
use std::collections::HashMap;

pub struct CanvasConfig {
    pub img: DynamicImage,
    pub n_cells_in_width: u8,
    pub n_colors: u8,
}

impl CanvasConfig {
    pub fn new(img: DynamicImage, n_cells_in_width: Option<u8>, n_colors: Option<u8>) -> Self {
        CanvasConfig {
            img,
            n_cells_in_width: n_cells_in_width.unwrap_or(32),
            n_colors: n_colors.unwrap_or(20),
        }
    }
}

#[derive(Debug)]
struct Stitch {
    pub x: u32,
    pub y: u32,
    color: RgbColor,
}

pub struct Canvas {
    pub picture: Vec<Vec<RgbColor>>,
}

impl Canvas {
    pub fn new(config: CanvasConfig) -> Result<Canvas, CanvasError> {
        let n_cells_in_width = config.n_cells_in_width;
        let (width, height) = config.img.dimensions();
        let cell_height = width as f32 / n_cells_in_width as f32;
        let rows = (height as f32 / cell_height).round() as u32;

        let palette = Self::get_image_palette(&config.img, config.n_colors);
        let dmc_palette = Self::convert_colors_to_dmc(&palette);

        let mut canvas: Vec<Vec<RgbColor>> = Vec::with_capacity(rows as usize);
        // let mut stitches: Vec<Stitch> = vec![];
        for y in 0..rows {
            let y_start = (y as f32 * cell_height).round() as u32;
            let y_end = (y_start + cell_height as u32).min(height);

            let mut row: Vec<RgbColor> = Vec::with_capacity(n_cells_in_width as usize);
            for x in 0..n_cells_in_width {
                let x_start = (x as f32 * cell_height).round() as u32;
                let x_end = (x_start + cell_height as u32).min(width);

                let major_color =
                    Self::get_major_color_in_cell(&config.img, x_start, x_end, y_start, y_end);
                let major_color_lab = Lab::from_rgb(&major_color.into());
                let closest_color = dmc_palette
                    .iter()
                    .min_by_key(|&&c| {
                        let dmc_color_lab = Lab::from_rgb(&c.into());
                        RgbColor::calculate_diff(major_color_lab, dmc_color_lab) as u32
                    })
                    .ok_or(CanvasError::DmcNotFound)?;

                // stitches.push(Stitch {
                //     x: x_start,
                //     y: y_start,
                //     color: *closest_color,
                // });
                row.push(*closest_color);
            }
            canvas.push(row)
        }

        // let mut pxl_img = DynamicImage::new(width, height, ColorType::Rgb8);
        // for stitch in stitches {
        //     for y in stitch.y..((stitch.y as f32 + cell_height).ceil() as u32).min(height) {
        //         for x in stitch.x..((stitch.x as f32 + cell_height).ceil() as u32).min(width) {
        //             let mut color: Rgb<u8> = stitch.color.into();
        //             pxl_img.put_pixel(x, y, color.to_rgba());
        //         }
        //     }
        // }
        Ok(Canvas { picture: canvas })
    }

    fn get_major_color_in_cell(
        image: &DynamicImage,
        x_start: u32,
        x_end: u32,
        y_start: u32,
        y_end: u32,
    ) -> RgbColor {
        let mut colors_count: HashMap<Rgb<u8>, u32> = HashMap::new();
        for y in y_start..y_end {
            for x in x_start..x_end {
                let color = image.get_pixel(x, y).to_rgb();
                colors_count
                    .entry(color)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }
        let mut sorted_entries: Vec<(Rgb<u8>, u32)> = colors_count.into_iter().collect();
        sorted_entries.sort_by(|&(color, count), &(color_2, count_2)| {
            if count != count_2 {
                return count_2.cmp(&count);
            }
            let lab_1 = Lab::from_rgb(&color.0);
            let lab_2 = Lab::from_rgb(&color_2.0);
            lab_1.b.partial_cmp(&lab_2.b).unwrap_or(Ordering::Equal)
        });
        sorted_entries[0].0.into()
    }

    fn get_image_palette(img: &DynamicImage, n_colors: u8) -> Vec<RgbColor> {
        let pixels = img.to_rgb8();
        let mut colors = get_palette_with_options(
            &pixels,
            PixelEncoding::Rgb,
            Quality::new(10),
            MaxColors::new(n_colors - 1),
            PixelFilter::None,
        );

        colors
            .iter_mut()
            .map(|val| RgbColor {
                red: val.r,
                green: val.g,
                blue: val.b,
            })
            .collect()
    }

    fn convert_colors_to_dmc(colors: &Vec<RgbColor>) -> Vec<RgbColor> {
        let mut dmc_colors: Vec<RgbColor> = Vec::with_capacity(colors.len());
        for color in colors {
            let DmcColor { rgb, .. } = color.find_dmc();
            dmc_colors.push(rgb);
        }
        dmc_colors
    }
}
