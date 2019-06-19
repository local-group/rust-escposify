extern crate escposify;
extern crate image;
extern crate tempfile;

use escposify::device::File;
use escposify::img::Image;
use escposify::printer::Printer;

use image::{DynamicImage, ImageBuffer};
use tempfile::NamedTempFileOptions;

fn main() {
    let tempf = NamedTempFileOptions::new().create().unwrap();

    let file = File::from(tempf);
    let mut printer = Printer::new(file, None, None);

    let img = ImageBuffer::from_fn(512, 512, |x, _| {
        if x % 2 == 0 {
            image::Rgb([0, 0, 0])
        } else {
            image::Rgb([0xFF, 0xFF, 0xFF])
        }
    });
    let image = Image::from(DynamicImage::ImageRgb8(img));
    let _ = printer
        .align("ct")
        .raster(&image, None)
        .raster(&image, Some("dw"))
        .raster(&image, Some("dh"))
        .raster(&image, Some("dwdh"))
        .flush();
}
