use crossterm::cursor;
use image::{io::Reader as ImageReader, DynamicImage, GenericImageView, Rgba, RgbaImage};
use viuer::Config;

pub struct PokemonPrinter {
    pub file_path: String,
}

impl PokemonPrinter {
    pub fn new(file_path: String) -> Self {
        PokemonPrinter { file_path }
    }

    pub fn print(&self) {
        let image = ImageReader::open(self.file_path.clone())
            .expect("Failed to open image file")
            .decode()
            .expect("Failed to decode image");

        let (x, y) = cursor::position().expect("Failed to get cursor position");

        let config = Config {
            x,
            y: y as i16,
            transparent: true,
            ..Default::default()
        };

        viuer::print(&remove_transparency(image), &config).expect("Image printing failed");
    }
}

fn remove_transparency(image: DynamicImage) -> DynamicImage {
    let (width, height) = image.dimensions();
    let mut rgba_image = RgbaImage::new(width, height);

    let mut min_x = width;
    let mut min_y = height;
    let mut max_x = 0;
    let mut max_y = 0;

    for (x, y, pixel) in image.pixels() {
        let Rgba([r, g, b, a]) = pixel;
        if a != 0 {
            if x < min_x {
                min_x = x;
            }
            if y < min_y {
                min_y = y;
            }
            if x > max_x {
                max_x = x;
            }
            if y > max_y {
                max_y = y;
            }
            rgba_image.put_pixel(x, y, Rgba([r, g, b, 255])); // 保留不透明部分
        } else {
            rgba_image.put_pixel(x, y, Rgba([0, 0, 0, 0]));
        }
    }

    if min_x > max_x || min_y > max_y {
        return DynamicImage::ImageRgba8(rgba_image); // 如果没有不透明部分，返回原图
    }

    let cropped_image = DynamicImage::ImageRgba8(rgba_image).crop_imm(
        min_x,
        min_y,
        max_x - min_x + 1,
        max_y - min_y + 1,
    );
    cropped_image
}
