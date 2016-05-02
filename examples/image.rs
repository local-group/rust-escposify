
extern crate escposify;
extern crate image;
extern crate tempfile;

use escposify::printer::Printer;
use escposify::device::File;
use escposify::img::Image;

use tempfile::{NamedTempFileOptions};
use image::{ImageBuffer, DynamicImage};


fn main() {
    let tempf = NamedTempFileOptions::new()
        .create()
        .unwrap();
    let temp_path = tempf.path().to_str().unwrap().to_owned();

    let file = File::from(temp_path.as_str(), tempf);
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
        .bit_image(&image, Some("s8"))
        .bit_image(&image, Some("d8"))
        .bit_image(&image, Some("s24"))
        .bit_image(&image, Some("d24"))
        .flush();
}
