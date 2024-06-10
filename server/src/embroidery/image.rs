use image::DynamicImage;
use palette_extract::{get_palette_with_options, MaxColors, PixelEncoding, PixelFilter, Quality};
use std::collections::HashSet;
use std::io::{Error, ErrorKind};

use crate::embroidery::colors::{DmcColor, RgbColor};

pub trait ImagePalette {
    fn get_rgb_palette(&self, n_colors: u8) -> Result<Vec<RgbColor>, Error>;
    fn get_dmc_palette(&self, n_colors: u8) -> Result<Vec<DmcColor>, Error>;
}

impl ImagePalette for DynamicImage {
    fn get_rgb_palette(&self, n_colors: u8) -> Result<Vec<RgbColor>, Error> {
        if n_colors <= 2 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Number of colors should be bigger than 2",
            ));
        }
        // palette_extract library gives wrong number of colors depending on input
        // to fix this, modification done below
        let mut n_colors: u8 = n_colors;
        if n_colors > 7 {
            n_colors += 1;
        } else if n_colors < 4 {
            n_colors -= 1
        }

        let pixels = self.to_rgb8();
        let mut colors = get_palette_with_options(
            &pixels,
            PixelEncoding::Rgb,
            Quality::new(10),
            MaxColors::new(n_colors),
            PixelFilter::None,
        );

        Ok(colors
            .iter_mut()
            .map(|val| RgbColor {
                red: val.r,
                green: val.g,
                blue: val.b,
            })
            .collect())
    }

    fn get_dmc_palette(&self, n_colors: u8) -> Result<Vec<DmcColor>, Error> {
        let colors = self.get_rgb_palette(n_colors).unwrap();
        Ok(convert_rgb_to_dmc(&colors))
    }
}

fn convert_rgb_to_dmc(colors: &Vec<RgbColor>) -> Vec<DmcColor> {
    let mut dmc_colors: HashSet<DmcColor> = HashSet::new();
    for color in colors {
        let dmc_color = color.find_dmc();
        dmc_colors.insert(dmc_color);
    }

    dmc_colors.into_iter().collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use image::{ImageBuffer, Rgb};

    fn generate_image() -> DynamicImage {
        let image_buffer = ImageBuffer::from_fn(10, 10, |x, y| {
            if (x + y) % 2 == 0 {
                Rgb([0, 0, 0])
            } else {
                Rgb([255, 255, 255])
            }
        });
        DynamicImage::ImageRgb8(image_buffer)
    }

    #[test]
    fn it_gets_rgb_palette() {
        let image = generate_image();
        let palette = image.get_rgb_palette(3).unwrap();
        assert_eq!(palette.len(), 3)
    }

    #[test]
    fn it_gets_rgb_palette_invalid_input() {
        let image = generate_image();
        let err = image.get_rgb_palette(2).unwrap_err();
        assert_eq!(err.to_string(), "Number of colors should be bigger than 2");
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
    }

    #[test]
    fn it_gets_dmc_palette() {
        let image = generate_image();
        let mut colors = image.get_dmc_palette(3).unwrap();
        colors.sort_by(|color_1, color_2| {
            let lab_1 = Lab::from_rgb(&color_1.rgb.into());
            let lab_2 = Lab::from_rgb(&color_2.rgb.into());

            (lab_1.l, lab_1.a, lab_1.b)
                .partial_cmp(&(lab_2.l, lab_2.a, lab_2.b))
                .unwrap_or(Ordering::Equal)
        });
        assert_eq!(colors.len(), 3);
        assert_eq!(colors[0].name, "310");
        assert_eq!(colors[1].name, "13");
        assert_eq!(colors[2].name, "B5200");
    }
}
