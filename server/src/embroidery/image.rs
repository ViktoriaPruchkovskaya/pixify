use crate::embroidery::colors::{DmcColor, RgbColor};
use image::DynamicImage;
use palette_extract::{get_palette_with_options, MaxColors, PixelEncoding, PixelFilter, Quality};
use std::io::{Error, ErrorKind};

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
        let mut n_colors: u8 = n_colors;
        if n_colors > 7 {
            n_colors += 1;
        } else if n_colors < 4 {
            n_colors -= 1
        }

        let pixels = self.to_rgb8(); //interestingly it generates 1 color less if n_colors >=7
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
    let mut dmc_colors: Vec<DmcColor> = Vec::with_capacity(colors.len());
    for color in colors {
        let dmc_color = color.find_dmc();
        dmc_colors.push(dmc_color);
    }
    dmc_colors
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
}
