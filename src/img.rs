
use std::path;
use std::iter::Iterator;

use image;
use image::{DynamicImage, GenericImage};


pub struct Image {
    // _path: String,
    pub width: u32,
    pub height: u32,
    img_buf: DynamicImage,
}

impl Image {
    pub fn new<P: AsRef<path::Path> + ToString>(path: P) -> Image {
        let img_buf = image::open(&path).unwrap();
        let (width, height) = img_buf.dimensions();
        Image {
            // _path: path.to_string(),
            width: width,
            height: height,
            img_buf: img_buf
        }
    }

    pub fn from(img_buf: DynamicImage) -> Image {
        let (width, height) = img_buf.dimensions();
        Image {
            // _path: "tmp".to_string(),
            width: width,
            height: height,
            img_buf: img_buf
        }
    }

    pub fn is_blank_pixel(&self, x: u32, y: u32) -> bool {
        let pixel = self.img_buf.get_pixel(x, y);
        // full transprant OR is white
        if pixel[3] == 0 || (pixel[0] & pixel[1] & pixel[2]) == 0xFF {
            true
        } else {
            false
        }
    }

    pub fn bitmap_lines(&self, density: u32) -> BitmapLines {
        BitmapLines {line: 0, density: density, image: self}
    }

    fn get_line(&self, num: u32, density: u32) -> Option<Box<[u8]>> {
        let n = self.height as u32 / density;
        let y = num - 1;
        if y >= n {
            return None
        }

        let c = density / 8;
        let mut data: Vec<u8> = vec![0; (self.width * c) as usize];
        // println!(">>> num={}, density={}, n={}, y={}, c={}, data.len()={}",
        //          num, density, n, y, c, data.len());
        for x in 0..self.width {
            for b in 0..density {
                let i = x * c + (b >> 3);
                // println!("x={}, b={}, i={}, b>>8={}", x, b, i, b>>3);
                let l = y * density + b;
                if l < self.height && !self.is_blank_pixel(x, l) {
                    data[i as usize] += 0x80>>(b & 0x07);
                }
            }
        }
        Some(data.into_boxed_slice())
    }

    pub fn get_raster(&self) -> Box<[u8]>{
        let n = self.width / 8;
        let mut data: Vec<u8> = vec![0; (self.width * self.height) as usize];
        for y in 0..self.height {
            for x in 0..n {
                for b in 0..8 {
                    let i = x * 8 + b;
                    if i < self.width && !self.is_blank_pixel(i, y) {
                        data[(y*n + x) as usize] += 0x80 >> (b & 0x7);
                    }
                }
            }
        }
        data.into_boxed_slice()
    }
}


pub struct BitmapLines<'a>{
    line: u32,
    density: u32,
    image: &'a Image,
}

impl<'a> Iterator for BitmapLines<'a> {
    type Item = Box<[u8]>;

    fn next(&mut self) -> Option<Box<[u8]>> {
        self.line += 1;
        self.image.get_line(self.line, self.density)
    }
}
