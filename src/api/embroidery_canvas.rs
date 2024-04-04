use std::collections::{HashMap, HashSet};
use image::{ColorType, DynamicImage, GenericImage, GenericImageView, Pixel, Rgb};
use lab::Lab;
use crate::api::colors::RGBColor;

#[derive(Debug)]
struct Stitch {
    pub x: u32,
    pub y: u32,
    color: RGBColor,
}

pub struct EmbroideryCanvas {
    pub picture: DynamicImage,
}

impl EmbroideryCanvas {
    pub fn new(img: DynamicImage) -> EmbroideryCanvas {
        let (width, height) = img.dimensions();
        let n_cells_in_width = 30;
        let cell_height = width as f32 / n_cells_in_width as f32;
        let rows = (height as f32 / cell_height).round() as u32;
        // let size = n_cells_in_width * rows;
        let mut color_palette: HashSet<RGBColor> = HashSet::new();
        let mut stitches: Vec<Stitch> = vec![];
        let mut pxl_img = DynamicImage::new(width, height, ColorType::Rgb8);
        for y in 0..rows {
            let y_start = (y as f32 * cell_height).round() as u32;
            let y_end = ((y_start as f32 + cell_height).round() as u32).min(height);
            for x in 0..n_cells_in_width {
                let x_start = (x as f32 * cell_height).round() as u32;
                let x_end = ((x_start as f32 + cell_height).round() as u32).min(width);
                let mut colors_count: HashMap<Rgb<u8>, u32> = HashMap::new();
                for y in y_start..y_end {
                    for x in x_start..x_end {
                        let color = img.get_pixel(x, y).to_rgb();
                        colors_count.entry(color.clone()).and_modify(|count| *count += 1).or_insert(1);
                    }
                }
                let mut sorted_entries: Vec<(Rgb<u8>, u32)> = colors_count.into_iter().collect();
                sorted_entries.sort_by(|&(key1, value1), &(key2, value2)| {
                    if value1 == value2 {
                        let lab1 = Lab::from_rgb(&key1.0);
                        let lab2 = Lab::from_rgb(&key2.0);
                        lab1.b.partial_cmp(&lab2.b).unwrap_or(std::cmp::Ordering::Equal)
                    } else {
                        value2.cmp(&value1)
                    }
                });
                let (major_color, ..) = sorted_entries[0];
                let rgb: RGBColor = major_color.into();
                let (rgb, ..): (RGBColor, &str) = rgb.find_dmc();
                for y in y_start..y_end {
                    for x in x_start..x_end {
                        pxl_img.put_pixel(x, y, Rgb([rgb.red, rgb.green, rgb.blue]).to_rgba());
                    }
                }
                stitches.push(Stitch { x: x_start, y: y_start, color: rgb });
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
        EmbroideryCanvas { picture: pxl_img }
    }

    fn get_palette(mut colors: HashSet<RGBColor>, colors_num: u8) -> HashMap<RGBColor, RGBColor> {
        let mut palette: Vec<RGBColor> = colors.clone().into_iter().collect();
        let mut diffs: Vec<(RGBColor, RGBColor, f32)> = vec![];
        for i in 0..palette.len() - 1 {
            let color = palette[i];
            let lab1 = Lab::from_rgb(&color.into());
            for j in i + 1..palette.len() {
                let other_color = palette[j];
                let lab2 = Lab::from_rgb(&other_color.into());
                let diff = RGBColor::calculate_diff(lab1, lab2);
                if diff != 0.0 && diff < 10.0 {
                    diffs.push((color, other_color, diff));
                }
            }
        }
        diffs.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
        let mut changed: HashMap<RGBColor, RGBColor> = HashMap::new();
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
}