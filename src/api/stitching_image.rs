use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb, RgbImage};
use crate::api::colors::RGBColor;

pub trait DynamicImageStitching {
    fn to_dmc_in_rgb(&self) -> DynamicImage;
}

impl DynamicImageStitching for DynamicImage {
    fn to_dmc_in_rgb(&self) -> DynamicImage {
        let (width, height) = self.dimensions();
        let mut img: RgbImage = ImageBuffer::new(width, height);
        for (width, height, pixel) in img.enumerate_pixels_mut() {
            let [red, green, blue, ..] = self.get_pixel(width, height).0;
            let rgb = RGBColor { red, green, blue };
            let (rgb, ..) = rgb.find_dmc();

            *pixel = Rgb([rgb.red, rgb.green, rgb.blue]);
        }
        DynamicImage::from(img)
    }
}