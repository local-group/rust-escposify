extern crate escposify;
extern crate image;
extern crate tempfile;

use escposify::device::File;
use escposify::img::Image;
use escposify::printer::Printer;

use image::{DynamicImage, ImageBuffer};
use std::io;
use tempfile::NamedTempFileOptions;

fn main() -> io::Result<()> {
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
    printer
        .chain_align("ct")?
        .chain_raster(&image, None)?
        .chain_raster(&image, Some("dw"))?
        .chain_raster(&image, Some("dh"))?
        .chain_raster(&image, Some("dwdh"))?
        .flush()
}
