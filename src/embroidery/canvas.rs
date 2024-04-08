use crate::embroidery::colors::{DmcColor, RgbColor};
use image::{ColorType, DynamicImage, GenericImage, GenericImageView, Pixel, Rgb};
use lab::Lab;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

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
    pub fn new(img: DynamicImage) -> Canvas {
        let (width, height) = img.dimensions();
        let n_cells_in_width = 30;
        let cell_height = width as f32 / n_cells_in_width as f32;
        let rows = (height as f32 / cell_height).round() as u32;
        // let size = n_cells_in_width * rows;
        let mut color_palette: HashSet<RgbColor> = HashSet::new();
        let mut stitches: Vec<Stitch> = vec![];
        for y in 0..rows {
            let y_start = (y as f32 * cell_height).round() as u32;
            let y_end = ((y_start as f32 + cell_height).round() as u32).min(height);
            for x in 0..n_cells_in_width {
                let x_start = (x as f32 * cell_height).round() as u32;
                let x_end = ((x_start as f32 + cell_height).round() as u32).min(width);
                let major_color =
                    Self::get_major_color_in_cell(&img, x_start, x_end, y_start, y_end);
                let DmcColor { rgb, .. } = major_color.find_dmc();
                stitches.push(Stitch {
                    x: x_start,
                    y: y_start,
                    color: rgb,
                });
                color_palette.insert(rgb);
            }
        }
        let mut pxl_img = DynamicImage::new(width, height, ColorType::Rgb8);
        let changed_colors = Self::get_palette(color_palette, 30);
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

    fn get_palette(mut colors: HashSet<RgbColor>, colors_num: u8) -> HashMap<RgbColor, RgbColor> {
        let palette: Vec<RgbColor> = colors.clone().into_iter().collect();
        let mut diffs: Vec<(RgbColor, RgbColor, f32)> = vec![];
        for i in 0..palette.len() - 1 {
            let color = palette[i];
            let lab1 = Lab::from_rgb(&color.into());
            for j in i + 1..palette.len() {
                let other_color = palette[j];
                let lab2 = Lab::from_rgb(&other_color.into());
                let diff = RgbColor::calculate_diff(lab1, lab2);
                if diff != 0.0 && diff < 10.0 {
                    diffs.push((color, other_color, diff));
                }
            }
        }
        diffs.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
        let mut changed: HashMap<RgbColor, RgbColor> = HashMap::new();
        while colors.len() > colors_num as usize {
            if let Some(diff) = diffs.pop() {
                changed.insert(diff.0, diff.1);
                colors.remove(&diff.0);
            } else {
                break;
            }
        }
        changed
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
