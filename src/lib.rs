extern crate encoding;
extern crate byteorder;
extern crate image;

#[cfg(feature="qrcode_builder")]
extern crate qrcode;

pub mod consts;
pub mod printer;
pub mod device;
pub mod img;
