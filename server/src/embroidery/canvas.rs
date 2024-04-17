use crate::embroidery::colors::{DmcColor, RgbColor};
use image::{ColorType, DynamicImage, GenericImage, GenericImageView, Pixel, Rgb};
use lab::Lab;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

pub struct CanvasConfig {
    pub img: DynamicImage,
    pub n_cells_in_width: u8,
    pub n_colors: usize,
}

impl CanvasConfig {
    pub fn new(img: DynamicImage, n_cells_in_width: Option<u8>, n_colors: Option<u8>) -> Self {
        CanvasConfig {
            img,
            n_cells_in_width: n_cells_in_width.unwrap_or(32),
            n_colors: n_colors.unwrap_or(20) as usize,
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
    pub picture: DynamicImage,
}

impl Canvas {
    pub fn new(config: CanvasConfig) -> Canvas {
        let n_cells_in_width = config.n_cells_in_width;
        let n_colors = config.n_colors;
        let (width, height) = config.img.dimensions();
        let cell_height = width as f32 / n_cells_in_width as f32;
        let rows = (height as f32 / cell_height).round() as u32;

        let mut color_palette: HashSet<RgbColor> = HashSet::new();
        // let mut canvas: Vec<Vec<RgbColor>> = Vec::with_capacity(rows as usize); //matrix
        let mut stitches: Vec<Stitch> = vec![];
        for y in 0..rows {
            let y_start = (y as f32 * cell_height).round() as u32;
            let y_end = (y_start + cell_height as u32).min(height);
            // let mut row: Vec<RgbColor> = vec![];
            for x in 0..n_cells_in_width {
                let x_start = (x as f32 * cell_height).round() as u32;
                let x_end = (x_start + cell_height as u32).min(width);
                let major_color =
                    Self::get_major_color_in_cell(&config.img, x_start, x_end, y_start, y_end);
                let DmcColor { rgb, .. } = major_color.find_dmc();
                stitches.push(Stitch {
                    x: x_start,
                    y: y_start,
                    color: rgb,
                });
                // row.push(rgb);
                color_palette.insert(rgb);
            }
            // canvas.push(row)
        }

        let (.., changed_colors) = Self::get_palette(color_palette, n_colors);
        let mut pxl_img = DynamicImage::new(width, height, ColorType::Rgb8);
        for stitch in stitches {
            for y in stitch.y..((stitch.y as f32 + cell_height).ceil() as u32).min(height) {
                for x in stitch.x..((stitch.x as f32 + cell_height).ceil() as u32).min(width) {
                    let mut color: Rgb<u8> = stitch.color.into();
                    if let Some(changed_color) = changed_colors.get(&stitch.color) {
                        color = (*changed_color).into();
                    }
                    pxl_img.put_pixel(x, y, color.to_rgba());
                }
            }
        }
        Canvas { picture: pxl_img }
    }

    fn get_palette(
        colors: HashSet<RgbColor>,
        n_colors: usize,
    ) -> (HashSet<RgbColor>, HashMap<RgbColor, RgbColor>) {
        let mut changed_colors: HashMap<RgbColor, RgbColor> = HashMap::new();
        if colors.len() == n_colors {
            return (colors, changed_colors);
        }

        let mut palette: Vec<RgbColor> = colors.clone().into_iter().collect();
        palette.sort_by(|color_1, color_2| {
            let lab_1 = Lab::from_rgb(&(*color_1).into());
            let lab_2 = Lab::from_rgb(&(*color_2).into());
            lab_2.b.partial_cmp(&lab_1.b).unwrap_or(Ordering::Equal)
        });

        let mut new_palette: HashSet<RgbColor> = HashSet::with_capacity(n_colors);
        while changed_colors.len() <= n_colors && !palette.is_empty() {
            let target_color = palette.pop().unwrap();
            let closest_color = palette
                .iter()
                .min_by_key(|&&color| {
                    let lab_1 = Lab::from_rgb(&target_color.into());
                    let lab_2 = Lab::from_rgb(&color.into());
                    RgbColor::calculate_diff(lab_1, lab_2) as u32
                })
                .copied()
                .unwrap_or(target_color);
            new_palette.insert(closest_color);
            changed_colors.insert(target_color, closest_color);
        }

        (new_palette, changed_colors)
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
}
