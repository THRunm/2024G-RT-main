use image::{DynamicImage, GenericImageView,  Pixel};
use std::env;
use std::path::Path;

#[derive(Clone)]
pub(crate) struct RtwImage {
    data: Option<DynamicImage>,
}

impl RtwImage {
    pub(crate) fn new(image_filename: &str) -> Self {
        let mut image = RtwImage { data: None };
        if let Some(img) = image.load_image(image_filename) {
            image.data = Some(img);
        } else {
            eprintln!("ERROR: Could not load image file '{}'.", image_filename);
        }
        image
    }

    fn load_image(&self, image_filename: &str) -> Option<DynamicImage> {
        let search_paths = [
            "", "images/", "../images/", "../../images/",
            "../../../images/", "../../../../images/", "../../../../../images/"
        ];

        let imagedir = env::var("RTW_IMAGES").ok();

        for base in search_paths.iter() {
            let path = if let Some(ref dir) = imagedir {
                format!("{}/{}", dir, image_filename)
            } else {
                format!("{}{}", base, image_filename)
            };

            if Path::new(&path).exists() {
                return image::open(&path).ok();
            }
        }

        None
    }

    pub(crate) fn width(&self) -> u32 {
        self.data.as_ref().map_or(0, |img| img.width())
    }

    pub(crate) fn height(&self) -> u32 {
        self.data.as_ref().map_or(0, |img| img.height())
    }

    pub(crate) fn pixel_data(&self, x: u32, y: u32) -> [u8; 3] {
        if let Some(ref img) = self.data {
            let pixel = img.get_pixel(x, y).to_rgb();
            return pixel.0;
        }
        [255, 0, 255]
    }
}


