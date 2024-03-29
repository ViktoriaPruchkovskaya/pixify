use std::collections::HashMap;
use image::{ColorType, DynamicImage, GenericImage, GenericImageView, Pixel, Rgb};
use crate::api::colors::RGBColor;

pub struct EmbroideryCanvas {
    pub picture: DynamicImage,
}

impl EmbroideryCanvas {
    pub fn new(img: DynamicImage) -> EmbroideryCanvas {
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
                let x_start = x * cell_height;
                let x_end = (x_start + cell_height + 1).min(width);
                let mut colors_count: HashMap<Rgb<u8>, u32> = HashMap::new();
                for y in y_start..y_end {
                    for x in x_start..x_end {
                        let color = img.get_pixel(x, y).to_rgb();
                        colors_count.entry(color.clone()).and_modify(|c| *c += 1).or_insert(1);
                    }
                }
                let (major_color, ..) = colors_count.iter().max_by_key(|&(_, count)| count).unwrap();
                let rgb = RGBColor { red: major_color[0], green: major_color[1], blue: major_color[2] };
                let (rgb, ..) = rgb.find_dmc();
                for y in y_start..y_end {
                    for x in x_start..x_end {
                        pxl_img.put_pixel(x, y, Rgb([rgb.red, rgb.green, rgb.blue]).to_rgba());
                    }
                }
            }
        }

        EmbroideryCanvas { picture: pxl_img }
    }
}