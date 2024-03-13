use image::{DynamicImage, GenericImageView};

pub trait DynamicImagePxl {
    fn pixelate(&self) -> DynamicImage;
}

impl DynamicImagePxl for DynamicImage {
    fn pixelate(&self) -> DynamicImage {
        let (width, height) = self.dimensions();
        let small_img = self.resize(width / 10, height / 10, image::imageops::FilterType::CatmullRom);
        small_img.resize(width, height, image::imageops::FilterType::Nearest)
    }
}