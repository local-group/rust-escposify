extern crate byteorder;
extern crate encoding;
extern crate image;

#[cfg(feature = "qrcode_builder")]
extern crate qrcode;

pub mod consts;
pub mod device;
pub mod img;
pub mod printer;
